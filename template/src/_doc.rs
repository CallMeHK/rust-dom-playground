use web_sys::{ Element };

pub fn query_selector(element: String) -> Element {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    
    document
        .query_selector(&element)
        .expect(&format!("should have {} on the page", &element))
        .unwrap()
}
