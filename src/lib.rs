use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn parse_json(json_str: &str) {
    let parsed: serde_json::Value = serde_json::from_str(json_str).unwrap();
    alert(&format!("Parsed JSON: {:?}", parsed));
}
