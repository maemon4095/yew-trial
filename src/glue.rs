use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/wwwroot/assets/js/glue.js")]
extern "C" {
    pub type Window;

    #[wasm_bindgen(static_method_of = Window)]
    pub fn alert(message: &str);
}
