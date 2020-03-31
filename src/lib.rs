//! # The Geometry Tools Library
//!
//! The geometry tools library provides functionality to interoperate between
//! SVG and WellKnown geometry types. It offers a set of validation functions
//! along with the standard geometric boolean operations.
//!
mod geometry_boolean;
mod geometry_wkt_writer;
mod geometry_svg_writer;
mod geometry_svg_reader;
mod polygon_validator;
mod json_errors;
mod utils;
use wasm_bindgen::prelude::*;

use geometry_boolean::{
    wkt_multi_polygon_polygon_union, wkt_multi_polygon_union, wkt_polygon_union,
};

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

/// Generates a union from two WKT geometries.
/// It throws an error if the union of the two geometry types is not supported,
/// or if invalid geometries have been submitted.
///
#[wasm_bindgen(js_name = wktUnion)]
pub fn wkt_union(geom1: String, geom2: String) -> Result<String, JsValue> {
    // Grab the stated type of each input
    let geom1_type = get_geometry_type(&geom1)?;
    let geom2_type = get_geometry_type(&geom2)?;

    if geom1_type.eq_ignore_ascii_case("MultiPolygon") {
        return if geom1_type == geom2_type {
            //union two multipolygons
            Ok(wkt_multi_polygon_union(&geom1, &geom2))
        } else if geom2_type.eq_ignore_ascii_case("Polygon") {
            // union multi + polygon
            Ok(wkt_multi_polygon_polygon_union(&geom1, &geom2))
        } else {
            Err(json_errors::boolean_geometry_errors::invalid_boolean_geom_pair(&geom1, &geom2))
        };
    } else if geom1_type.eq_ignore_ascii_case("Polygon") {
        return if geom1_type == geom2_type {
            // union two polygons
            let result = wkt_polygon_union(&geom1, &geom2);
            if result.is_ok() { Ok(result.unwrap()) }
            else { Err(result.err().unwrap()) }

        } else if geom2_type.eq_ignore_ascii_case("MulitPolygon") {
            // union multi + polygon
            Ok(wkt_multi_polygon_polygon_union(&geom2, &geom1))
        } else {
            Err(json_errors::boolean_geometry_errors::invalid_boolean_geom_pair(&geom1, &geom2))
        };
    }

    Err(JsValue::from_str(
        "Could not process the submitted geometries.",
    ))
}

/// This function reads a submitted string and makes a very quick decision
/// about the WKT geometry type it must contain. This function does not
/// actually check if the string really contains the shape it claims to
/// contain, nor does it do any processing or validation. All validation should
/// be down downstream.
///
fn get_geometry_type(geom: &String) -> Result<String, JsValue> {
    match geom.get(..1) {
        Some("G") => Ok(String::from("GeometryCollection")),
        Some("L") => Ok(String::from("LineString")),
        Some("M") => {
            if geom.starts_with("MultiL") {
                Ok(String::from("MultiLineString"))
            } else if geom.starts_with("MultiPoi") {
                Ok(String::from("MultiPoint"))
            } else if geom.starts_with("MultiP") {
                Ok(String::from("MultiPolygon"))
            } else {
                Err(json_errors::wkt_errors::wkt_cannot_be_parsed(geom))
            }
        }
        Some("P") => {
            if geom.starts_with("Poi") {
                Ok(String::from("Point"))
            } else {
                Ok(String::from("Polygon"))
            }
        }
        // Return immediately on empty string
        Some(&_) => Err(json_errors::wkt_errors::wkt_cannot_be_parsed(geom)),
        None => Err(json_errors::wkt_errors::wkt_cannot_be_parsed(geom)),
    }
}

/** Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_join_polygons() {
        let res = wkt_union(
            "POLYGON((0 0,10 0,10 10,0 10,0 0),(3 3,6 3,6 6,3 6,3 3))".into(),
            "POLYGON((2 2,4 2,4 4,2 4,2 2))".into(),
        );
        assert_eq!(res, res);
    }
}
