

// https://github.com/rust-lang/getopts
extern crate getopts;

extern crate libc;

// Rust FFI for:
// krb5_init_context
// krb5_cc_default
// krb5_cc_get_principal
// com_err
mod com_err;

use getopts::Options;
use std::env;
use std::ffi::{CString};
use std::io::{self,Write};
use std::process;

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
    

    unsafe {
        let code = 1234567;
        if code != 0 {
            let whoami = CString::new((*PROGNAME).clone())
                .unwrap().into_raw();
            let detail = CString::new("while decoding com_err code 1234567")
                .unwrap().into_raw();
            com_err::com_err(whoami, code as libc::c_long, detail);
        }
    }


}
