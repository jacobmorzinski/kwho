

// https://github.com/rust-lang/getopts
extern crate getopts;

use getopts::Options;
use std::env;
use std::io::{self,Write};
use std::process;
use std::ptr;

// https://doc.rust-lang.org/1.0.0/book/ffi.html
extern crate libc;

#[allow(non_camel_case_types)]
pub mod ffi {
    use libc::{int32_t};

    pub type krb5_int32 = int32_t;
    pub type krb5_error_code = krb5_int32;

    pub enum _krb5_context {}            // internal/opaque

    #[link(name="krb5")]
    extern "C" {
        pub fn krb5_init_context(context: *mut _krb5_context) -> krb5_error_code;
    }
}

#[derive(Debug)]
#[allow(raw_pointer_derive)]     // TODO: fix
pub struct Krb5Context {
    ctx: *mut ffi::_krb5_context
}

impl Krb5Context {
    pub fn new() -> Krb5Context {
        Krb5Context {
            ctx: ptr::null_mut(),
        }
    }
    pub fn krb5_init_context(&self) {
        unsafe { ffi::krb5_init_context(self.ctx); }
    }
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


// use libc::c_int;
// // krb5_context from krb5-1.8.2/src/include/k5-int.h
// #[repr(C)]
// pub struct _krb5_context {
//     // ...etc...
//     pub ser_ctx_count: c_int,
//     // ...etc...
// }


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

    let ctx = Krb5Context::new();
//    let code = ctx.krb5_init_context();
    println!("ctx is {:?}", ctx);
//    println!("code is {:?}", code);
}
