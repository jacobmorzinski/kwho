# kwho-rust

Learning how to call C libraries from Rust.

Resources that have helped:

https://www.reddit.com/r/rust/comments/1wu4yl/binding_opaque_interfaces/

https://www.reddit.com/r/rust/comments/2fmvcy/rust_ffi_and_opaque_pointer_idiom/

https://doc.rust-lang.org/book/ffi.html

https://doc.rust-lang.org/book/raw-pointers.html

http://stackoverflow.com/questions/24145823/rust-ffi-c-string-handling

http://stackoverflow.com/questions/24759028/how-should-you-do-pointer-arithmetic-in-rust

https://www.reddit.com/r/rust/comments/2sa4lc/char_to_rust_vecstr_u32_and_u64_array_pointer/

## rust-bindgen

Kerberos header krb5.h auto-converted to rust with `rust-bindgen`

https://github.com/crabtw/rust-bindgen

    bindgen `krb5-config --libs --cflags` -match krb5.h -o /tmp/krb5.rs /usr/include/krb5/krb5.h

