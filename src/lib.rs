#![doc = include_str!("../README.md")]


#[doc = include_str!("data/posts.md")]
pub mod posts {
    #[doc = include_str!("data/posts/how_i_made_this_site.md")]
    pub mod how_i_made_this_site {}
}

#[doc = include_str!("data/guide_to_rust.md")]
pub mod guide_to_rust {}


use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
