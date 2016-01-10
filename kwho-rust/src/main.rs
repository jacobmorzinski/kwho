

// https://github.com/rust-lang/getopts
extern crate getopts;

// https://doc.rust-lang.org/stable/book/ffi.html
extern crate libc;

// Rust/C bindings for:
// krb5_init_context
// krb5_cc_default
// krb5_cc_get_principal
// com_err
#[allow(non_camel_case_types)]
mod krb5;
mod com_err;

use getopts::Options;
use std::env;
use std::ffi::{CStr,CString};
use std::io::{self,Write};
use std::process;
use std::ptr;


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

    let progname = CString::new((*PROGNAME).clone())
        .unwrap()
        .into_raw();

    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { error(f) }
    };

    
    if matches.opt_present("h") {
        usage(&opts);
    }
    
    let mut ctx: krb5::krb5_context = ptr::null_mut();
    let mut code: krb5::krb5_error_code = unsafe { krb5::krb5_init_context(&mut ctx) };
    if code != 0 {
        unsafe {
            com_err::com_err(progname,
                             code as i64,
                             CString::new("while initializing krb5").unwrap().into_raw());
        }
        std::process::exit(1);
    }

    let mut cache: krb5::krb5_ccache = ptr::null_mut();
    code = unsafe {krb5::krb5_cc_default(ctx, &mut cache)};
    if code != 0 {
        unsafe {
            com_err::com_err(progname,
                             code as i64,
                             CString::new("while getting default ccache").unwrap().into_raw());
        }
        std::process::exit(1);
    }

    let mut princ: krb5::krb5_principal = ptr::null_mut();
    code = unsafe { krb5::krb5_cc_get_principal(ctx, cache, &mut princ) };
    if code != 0 {
        unsafe {
            com_err::com_err(progname,
                             code as i64,
                             CString::new("while retrieving principal name").unwrap().into_raw());
        }
        std::process::exit(1);
    }


    let kpd = unsafe{(*princ)};
    let len = kpd.length;

    let d = unsafe { (*kpd.data).data };
    let s = unsafe { CStr::from_ptr(d).to_string_lossy() };
    print!("{}", s);

    for i in 1..len {
        let d = unsafe { (*kpd.data.offset(i as isize)).data };
        let s = unsafe { CStr::from_ptr(d).to_string_lossy() };
        print!("/{}", s);
    }
    println!("");

}
