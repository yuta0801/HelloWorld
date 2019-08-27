#[macro_use]
extern crate neon;

extern {
    fn hello_c();
}

use neon::prelude::*;

fn hello_rust(mut cx: FunctionContext) -> JsResult<JsString> {
    unsafe { hello_c() };
    println!("Hello Rust");
    Ok(cx.string("Hello Node.js"))
}

register_module!(mut cx, {
    cx.export_function("hello", hello_rust)
});
