#[macro_use]
extern crate neon;

use neon::prelude::*;

fn hello_rust(mut cx: FunctionContext) -> JsResult<JsString> {
    println!("Hello Rust");
    Ok(cx.string("Hello Node.js"))
}

register_module!(mut cx, {
    cx.export_function("hello", hello_rust)
});
