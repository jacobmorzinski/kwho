

// https://github.com/rust-lang/getopts
extern crate getopts;

extern crate libc;

// Rust FFI for:
// krb5_init_context
// krb5_cc_default
// krb5_cc_get_principal
// com_err
mod krb5;
mod com_err;

use getopts::Options;
use std::env;
use std::ffi::{CStr,CString};
use std::io::{self,Write};
use std::process;
use std::ptr;

pub struct Krb5Context {
    ctx: krb5::krb5_context,
}
impl Krb5Context {
    pub fn new() -> Option<Krb5Context> {
        let mut ctx: krb5::krb5_context = ptr::null_mut();
        unsafe {
            let code = krb5::krb5_init_context(&mut ctx);
            if code != 0 {
                let whoami = CString::new((*PROGNAME).clone())
                    .unwrap().into_raw();
                let detail = CString::new("while initializing krb5")
                    .unwrap().into_raw();
                com_err::com_err(whoami, code as i64, detail);
                return None;
            }
        }
        Some(Krb5Context { ctx: ctx, })
    }
}

pub struct Krb5Cache {
    cache: krb5::krb5_ccache,
}
impl Krb5Cache {
    pub fn get(ctx: &Krb5Context) -> Option<Krb5Cache> {
        let mut cache: krb5::krb5_ccache = ptr::null_mut();
        unsafe {
            let code = krb5::krb5_cc_default(ctx.ctx, &mut cache);
            if code != 0 {
                let whoami = CString::new((*PROGNAME).clone())
                    .unwrap().into_raw();
                let detail = CString::new("while getting default ccache")
                    .unwrap().into_raw();
                com_err::com_err(whoami, code as i64, detail);
                return None;
            }
        }
        Some(Krb5Cache { cache: cache, })
    }
}

pub struct Krb5Princ {
    princ: krb5::krb5_principal,
}
impl Krb5Princ {
    pub fn get(ctx: &Krb5Context, cache: &Krb5Cache) -> Option<Krb5Princ> {
        let mut princ: krb5::krb5_principal = ptr::null_mut();
        unsafe {
            let code = krb5::krb5_cc_get_principal(ctx.ctx,
                                                   cache.cache,
                                                   &mut princ);
            if code != 0 {
                let whoami = CString::new((*PROGNAME).clone())
                    .unwrap().into_raw();
                let detail = CString::new("while retrieving principal name")
                    .unwrap().into_raw();
                com_err::com_err(whoami, code as i64, detail);
                return None;
            }
        }
        Some(Krb5Princ { princ: princ, })
    }
    pub fn realm(&self) -> String {
        let kp = unsafe { *self.princ };
        let d = kp.realm.data;
        let s = unsafe { CStr::from_ptr(d).to_string_lossy() };
        s.into_owned()
    }
    pub fn data(&self) -> String {
        let kp = unsafe { *self.princ };
        let len = kp.length;

        // Pointer offset arithmetic
        // C code is: krb5_principal->(data+(i))->data
        (0..len).map(
            |i|
            unsafe{
                CStr::from_ptr((*kp.data.offset(i as isize)).data)
                    .to_string_lossy()
            })
            .collect::<Vec<_>>()
            .join("/")
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
    
    let k5ctx = Krb5Context::new().unwrap();
    let k5cache = Krb5Cache::get(&k5ctx).unwrap();
    let k5princ = Krb5Princ::get(&k5ctx, &k5cache).unwrap();

    println!("{}", k5princ.data());
}
