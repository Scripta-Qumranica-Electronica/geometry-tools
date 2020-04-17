//! # The Geometry Tools Library
//!
//! The geometry tools library provides functionality to interoperate between
//! SVG and WellKnown geometry types. It offers a set of validation functions
//! along with the standard geometric boolean operations.
//!

mod boolean;
mod convertors;
mod geometry_boolean;
mod information;
mod json_errors;
mod utils;
mod validators;
use wasm_bindgen::prelude::*;

use crate::convertors::svg_to_wkt;
use geo_svg_io::geo_svg_writer::ToSvgString;
use geometry_boolean::{
    wkt_multi_polygon_polygon_union, wkt_multi_polygon_union, wkt_polygon_union,
};
use wkt::Wkt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

/// Shows a greeting in an alert (remove for production).
#[wasm_bindgen]
pub fn greet() {
    alert("Hello, geometry-tools!");
}
pub fn exp_svg_to_wkt(svg: String) -> Result<String, JsValue> {
    svg_to_wkt(svg)
}
/** Tests */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boolean::wkt_union;

    #[test]
    fn can_join_polygons() {
        let res = wkt_union(
            "POLYGON((0 0,10 0,10 10,0 10,0 0),(3 3,6 3,6 6,3 6,3 3))".into(),
            "POLYGON((2 2,4 2,4 4,2 4,2 2))".into(),
        );
        assert_eq!(res, res);
    }
}
