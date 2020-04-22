pub mod wkt_errors {
    use wasm_bindgen::JsValue;
    pub fn invalid_wkt_type(wkt: &str) -> JsValue {
        JsValue::from_str(
            format!("The wkt geometry type is invalid or unsupported: {}", wkt).as_ref(),
        )
    }

    pub fn wkt_cannot_be_parsed(wkt: &str) -> JsValue {
        JsValue::from_str(
            format!("The wkt geometry could not be successfully parsed: {}", wkt).as_ref(),
        )
    }

    pub fn invalid_geometry(wkt: &str) -> JsValue {
        JsValue::from_str(
            format!(
                "The submitted shape resulted in an invalid geometry: {}",
                wkt
            )
            .as_ref(),
        )
    }
}

pub mod geometry_processing_error {
    use wasm_bindgen::JsValue;
    pub fn invalid_boolean_geom_pair(g1: &str, g2: &str) -> JsValue {
        JsValue::from_str(
            format!(
                "Cannot perform a boolean operation on geometries {} and {}",
                g1, g2
            )
            .as_ref(),
        )
    }
    pub fn invalid_geom(reason: &str) -> JsValue {
        JsValue::from_str(format!("The geometry is bad. {}", reason).as_ref())
    }
    pub fn irreparable_geom() -> JsValue {
        JsValue::from_str("The attempt to repair the geometry failed")
    }
}

pub mod svg_error {
    use wasm_bindgen::JsValue;
    pub fn could_not_parse() -> JsValue {
        JsValue::from_str("The submitted SVG element/d-string could not be parsed")
    }
}
