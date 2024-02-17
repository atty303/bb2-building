use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/bundle.js")]
extern "C" {
    pub type CodeMirror;

    #[wasm_bindgen(constructor)]
    pub fn new(parent: &web_sys::Element, on_change: &Closure<dyn FnMut(String)>) -> CodeMirror;

    #[wasm_bindgen(method, getter)]
    pub fn value(this: &CodeMirror) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_value(this: &CodeMirror, value: String);
}
