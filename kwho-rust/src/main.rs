

// https://github.com/rust-lang/getopts
extern crate getopts;

use getopts::Options;
use std::env;
use std::ffi::{CStr,CString};
use std::io::{self,Write};
use std::process;
use std::ptr;

// https://doc.rust-lang.org/stable/book/ffi.html
extern crate libc;

#[allow(dead_code, non_camel_case_types)]
pub mod com_err {
    #[link(name = "krb5")]
    #[link(name = "k5crypto")]
    #[link(name = "com_err")]
    extern "C" {
        pub fn com_err(arg1: *const ::libc::c_char, arg2: ::libc::c_int,
                       arg3: *const ::libc::c_char, ...) -> ();
    }
}

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

    let progname = (*PROGNAME).clone();

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
        // println!("Sorry, IGNORING requested ccache ({})", ccache.clone().unwrap());
    }

    let mut ctx: krb5::krb5_context = ptr::null_mut();
    let mut code: krb5::krb5_error_code = unsafe { krb5::krb5_init_context(&mut ctx) };
    if code != 0 {
        unsafe {
            com_err::com_err(CString::new(progname).unwrap().into_raw(),
                             code,
                             CString::new("while initializing krb5").unwrap().into_raw());
        }
        std::process::exit(1);
    }

    let mut cache: krb5::krb5_ccache = ptr::null_mut();
    code = unsafe {krb5::krb5_cc_default(ctx, &mut cache)};
    if code != 0 {
        unsafe {
            com_err::com_err(CString::new(progname).unwrap().into_raw(),
                             code,
                             CString::new("while getting default ccache").unwrap().into_raw());
        }
        std::process::exit(1);
    }

    let mut princ: krb5::krb5_principal = ptr::null_mut();
    code = unsafe { krb5::krb5_cc_get_principal(ctx, cache, &mut princ) };
    if code != 0 {
        unsafe {
            com_err::com_err(CString::new(progname).unwrap().into_raw(),
                             code,
                             CString::new("while retrieving principal name").unwrap().into_raw());
        }
        std::process::exit(1);
    }

    let len = unsafe{(*princ).length};

    let d = unsafe { CStr::from_ptr((*(*princ).data).data).to_string_lossy() };
    print!("{}", d);

    for i in 1..len {
        let d2 = unsafe { CStr::from_ptr((*(*princ).data.offset(i as isize)).data).to_string_lossy() };
        print!("/{}", d2);
    }
    println!("");

}
