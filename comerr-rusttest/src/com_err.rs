/* automatically generated by rust-bindgen */

/*
 * Generated with something like:
 *  bindgen `krb5-config --libs --cflags` -match com_err.h -o /tmp/com_err.rs /usr/include/com_err.h
 *
 * and then hand-edited to keep only the needed parts.
 */

#[link(name = "com_err")]
extern "C" {
    pub fn com_err(arg1: *const ::libc::c_char, arg2: ::libc::c_long,
                   arg3: *const ::libc::c_char, ...) -> ();
}
