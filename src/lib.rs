#![doc = include_str!("../README.md")]


#[doc = include_str!("data/posts.md")]
pub mod posts {
    #[doc = include_str!("data/posts/how_i_made_this_site.md")]
    pub mod how_i_made_this_site {}
}

#[doc = include_str!("data/guide_to_rust.md")]
pub mod guide_to_rust {}


use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;



// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");
    body.append_child(&val)?;

    
    let btn_action = Closure::<dyn FnMut()>::new(move || {
        window.alert_with_message("Trying to change pages...").unwrap();
    });
    document
        .get_element_by_id("test-button")
        .expect("should have #test-button on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#test-button should be an `HtmlElement`")
        .set_onclick(Some(btn_action.as_ref().unchecked_ref()));

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
