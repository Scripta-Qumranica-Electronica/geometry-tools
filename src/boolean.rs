use crate::geometry_boolean::{
    wkt_multi_polygon_polygon_union, wkt_multi_polygon_union, wkt_polygon_union,
};
use crate::information::get_geometry_type;
use crate::json_errors;
use wasm_bindgen::prelude::*;

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
            let result = wkt_polygon_union(&geom1, &geom2)?;
            Ok(result)
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
