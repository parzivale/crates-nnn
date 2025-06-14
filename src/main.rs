use std::{
    env,
    ffi::{CString, c_char, c_int},
};

unsafe extern "C" {
    fn c_main(argc: c_int, argv: *const *const c_char) -> c_int;
}
fn main() {
    let args: Vec<CString> = env::args().map(|arg| CString::new(arg).unwrap()).collect();
    let c_args: Vec<*const c_char> = args.iter().map(|s| s.as_ptr()).collect();
    unsafe {
        let exit_code = c_main(c_args.len() as c_int, c_args.as_ptr());
        std::process::exit(exit_code);
    }
}
