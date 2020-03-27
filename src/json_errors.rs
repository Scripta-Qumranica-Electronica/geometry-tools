pub mod wkt_errors {
    use wasm_bindgen::JsValue;
    pub fn invalid_wkt_type(wkt: &String) -> JsValue {
        JsValue::from_str(format!("The wkt geometry type is invalid or unsupported: {}", wkt).as_ref())
    }

    pub fn wkt_cannot_be_parsed(wkt: &String) -> JsValue {
        JsValue::from_str(format!("The wkt geometry could not be successfully parsed: {}", wkt).as_ref())
    }
}

pub mod boolean_geometry_errors {
    use wasm_bindgen::JsValue;
    pub fn invalid_boolean_geom_pair(g1: &String, g2: &String) -> JsValue {
        JsValue::from_str(format!("Cannot perform a boolean operation on geometries {} and {}", g1, g2).as_ref())
    }
}
