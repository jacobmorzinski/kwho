

// https://github.com/rust-lang/getopts
extern crate getopts;

// https://github.com/Kimundi/lazy-static.rs
#[macro_use]
extern crate lazy_static;

use getopts::Options;
use std::env;
use std::io::{self,Write};
use std::process;

// https://doc.rust-lang.org/1.0.0/book/ffi.html
extern crate libc;
use libc::{int32_t};

#[allow(non_camel_case_types)]
enum krb5_context {}            // internal/opaque

// typedef krb5_int32 krb5_error_code
// typedef int32_t krb5_int32

#[link(name="krb5")]
extern {
    fn krb5_init_context(context: *mut krb5_context) -> int32_t;
}


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

    let kcontext = krb5_context;
    let rv = krb5_init_context(kcontext);
    println!("rv is {}", rv);
}
