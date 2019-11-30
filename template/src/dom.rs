use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};
use crate::_doc;

pub fn setup_clicker(document: &Document) {
    let dom_str = "button";
    let button = _doc::query_selector(dom_str.to_string());
    let button_text = "Count: ";
    let mut clicks = 0;
    let handle_click = Closure::wrap(Box::new(move || {
        clicks += 1;
        let new_btn_text = format!("{}{}", button_text, clicks);
        button.set_inner_html(&new_btn_text);
    }) as Box<dyn FnMut()>);
    _doc::query_selector(dom_str.to_string())
        .dyn_ref::<HtmlElement>()
        .expect("#green-square be an `HtmlElement`")
        .set_onclick(Some(handle_click.as_ref().unchecked_ref()));

    handle_click.forget();
}

pub fn append_text() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let root = "#root";
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.query_selector(&root).expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("button")?;
    val.set_inner_html("Count: 0");

    body.unwrap().append_child(&val)?;

    setup_clicker(&document);

    Ok(())
}


