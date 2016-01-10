

// https://github.com/rust-lang/getopts
extern crate getopts;

use getopts::Options;
use std::env;
use std::io::{self,Write};
use std::process;
use std::ptr;

// https://doc.rust-lang.org/stable/book/ffi.html
extern crate libc;

#[allow(dead_code, non_camel_case_types)]
pub mod krb5 {
    pub type krb5_int32 = ::libc::c_int;
    pub type krb5_error_code = krb5_int32;
    pub type krb5_magic = krb5_error_code;

    pub enum Struct__krb5_context { }
    pub type krb5_context = *mut Struct__krb5_context;
    #[link(name = "krb5")]
    #[link(name = "k5crypto")]
    #[link(name = "com_err")]
    extern "C" {
        pub fn krb5_init_context(context: *mut krb5_context)
          -> krb5_error_code;
        pub fn krb5_cc_default(context: krb5_context, ccache: *mut krb5_ccache)
          -> krb5_error_code;
        pub fn krb5_cc_get_principal(context: krb5_context, cache: krb5_ccache,
                                     principal: *mut krb5_principal)
          -> krb5_error_code;

    }

    pub enum Struct__krb5_ccache { }
    pub type krb5_ccache = *mut Struct__krb5_ccache;

    #[repr(C)]
    #[derive(Copy)]
    pub struct Struct__krb5_data {
        pub magic: krb5_magic,
        pub length: ::libc::c_uint,
        pub data: *mut ::libc::c_char,
    }
    impl ::std::clone::Clone for Struct__krb5_data {
        fn clone(&self) -> Self { *self }
    }
    impl ::std::default::Default for Struct__krb5_data {
        fn default() -> Self { unsafe { ::std::mem::zeroed() } }
    }
    pub type krb5_data = Struct__krb5_data;

    #[repr(C)]
    #[derive(Copy)]
    pub struct Struct_krb5_principal_data {
        pub magic: krb5_magic,
        pub realm: krb5_data,
        pub data: *mut krb5_data,
        pub length: krb5_int32,
        pub _type: krb5_int32,
    }
    impl ::std::clone::Clone for Struct_krb5_principal_data {
        fn clone(&self) -> Self { *self }
    }
    impl ::std::default::Default for Struct_krb5_principal_data {
        fn default() -> Self { unsafe { ::std::mem::zeroed() } }
    }
    pub type krb5_principal_data = Struct_krb5_principal_data;
    pub type krb5_principal = *mut krb5_principal_data;

}

// #[derive(Debug)]
// struct Krb5Context {
//     ctx: krb5::krb5_context,
// }
// 
// impl Krb5Context {
//     pub fn new() -> Krb5Context {
//         Krb5Context {
//             ctx: ptr::null_mut(),
//         }
//     }
//     pub fn krb5_init_context(&self) {
//         unsafe { krb5::krb5_init_context(self.ctx); }
//     }
// }


// This is a silly amount of work, but I wanted to learn how to
// get the progname into a global variable.
// 
// https://github.com/Kimundi/lazy-static.rs
#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref PROGNAME: String = env::current_exe().unwrap() // PathBuf
        .file_name().unwrap()                                 // &OsStr
        .to_string_lossy().into_owned();                      // Cow<str> -> String
}

fn error(err: getopts::Fail) -> ! {
    let mut stderr = io::stderr();
    writeln!(&mut stderr, "Error: {}", err).unwrap();
    writeln!(&mut stderr, "Try '{} --help' for more information.", *PROGNAME).unwrap();
    process::exit(1);
}

fn usage(opts: &Options) -> ! {
    let brief = opts.short_usage(&*PROGNAME);
    println!("{}", opts.usage(&brief));
    process::exit(0);
}

fn main() {
// Debugging code:
//    println!("Progname is {}", *PROGNAME);


    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("c", "", "specifies credential cache", "CCNAME");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { error(f) }
    };

    
    if matches.opt_present("h") {
        usage(&opts);
    }
    
    let ccache: Option<String> = matches.opt_str("c");
// Debugging code:
    if ccache.is_some() {
        println!("ccache is {}", ccache.clone().unwrap());
    }

    let mut ctx: krb5::krb5_context = ptr::null_mut();
    let mut code: krb5::krb5_error_code = unsafe { krb5::krb5_init_context(&mut ctx) };
    if code != 0 {
        panic!("Need better errors!");
    }
    println!("ctx is {:?}", ctx);

    let mut cache: krb5::krb5_ccache = ptr::null_mut();
    code = unsafe {krb5::krb5_cc_default(ctx, &mut cache)};
    if code != 0 {
        panic!("Need better errors!");
    }
    println!("cache is {:?}", cache);

    let mut princ: krb5::krb5_principal = ptr::null_mut();
    code = unsafe { krb5::krb5_cc_get_principal(ctx, cache, &mut princ) };
    if code != 0 {
        panic!("Need better errors!");
    }
    println!("princ is {:?}", princ);

    println!("princ length is {:?}", unsafe{(*princ).length});

    println!("{:?}", unsafe{(*(*princ).data).data});

//    #define krb5_princ_size(context, princ) (princ)->length
//
//    #define krb5_princ_component(context, princ,i)  \
//    (((i) < krb5_princ_size(context, princ))    \
//     ? (princ)->data + (i)                      \
//     : NULL)

}
