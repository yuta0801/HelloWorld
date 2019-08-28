#[macro_use]
extern crate neon;
extern crate libc;

use libc::c_char;
use std::ffi::{CString, CStr};

extern {
    fn hello_cpp(world: *const c_char) -> *const c_char;
}

use neon::prelude::*;

fn hello_rust(mut cx: FunctionContext) -> JsResult<JsString> {
    let world = cx.argument::<JsString>(0)?.value();

    let c_arg = CString::new(world).unwrap();
    let c_buf = unsafe { hello_cpp(c_arg.as_ptr()) };
    let c_res = unsafe { CStr::from_ptr(c_buf) };
    let hello_world = c_res.to_str().unwrap();

    Ok(cx.string(hello_world))
}

register_module!(mut cx, {
    cx.export_function("hello", hello_rust)
});
