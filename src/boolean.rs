use crate::convertors::wkt_to_svg;
use crate::geometry_boolean::{geometry_boolean, wkt_boolean};
use crate::information::get_geometry_type;
use crate::json_errors;
use geo_svg_io::geo_svg_reader::svg_to_geometry;
use geo_svg_io::geo_svg_writer::{ToSvg, ToSvgString};
use geo_wkt_writer::ToWkt;
use wasm_bindgen::prelude::*;

/** WKT Booleans */

/// Generates a union from two WKT geometries.
///
/// It throws an error if the union operation on the two geometry types is not supported,
/// or if invalid geometries have been submitted.
///
#[wasm_bindgen(js_name = wktPolygonUnion)]
pub fn wkt_polygon_union(geom1: String, geom2: String) -> Result<String, JsValue> {
    wkt_polygon_boolean(geom1, geom2, geo_booleanop::boolean::Operation::Union)
}

/// Generates a difference from two WKT geometries.
///
/// It throws an error if the difference operation on the two geometry types is not supported,
/// or if invalid geometries have been submitted.
///
#[wasm_bindgen(js_name = wktPolygonDifference)]
pub fn wkt_polygon_difference(geom1: String, geom2: String) -> Result<String, JsValue> {
    wkt_polygon_boolean(geom1, geom2, geo_booleanop::boolean::Operation::Difference)
}

/// Generates an intersection from two WKT geometries.
///
/// It throws an error if the intersection operation on the two geometry types is not supported,
/// or if invalid geometries have been submitted.
///
#[wasm_bindgen(js_name = wktPolygonIntersection)]
pub fn wkt_polygon_intersection(geom1: String, geom2: String) -> Result<String, JsValue> {
    wkt_polygon_boolean(
        geom1,
        geom2,
        geo_booleanop::boolean::Operation::Intersection,
    )
}

/// Generates a symmetric difference from two WKT geometries.
///
/// It throws an error if the symmetric difference operation on the two geometry types is not supported,
/// or if invalid geometries have been submitted.
///
#[wasm_bindgen(js_name = wktPolygonSymmetricDifference)]
pub fn wkt_polygon_xor(geom1: String, geom2: String) -> Result<String, JsValue> {
    wkt_polygon_boolean(geom1, geom2, geo_booleanop::boolean::Operation::Xor)
}

/// Perform a boolean operation on the submitted WKT geometries
///
/// This performs a quick test first to determine if the submitted geometry types are
/// suitable for the boolean operation (only MULTIPOLYGON and POLYGON are supported)
fn wkt_polygon_boolean(
    geom1: String,
    geom2: String,
    op: geo_booleanop::boolean::Operation,
) -> Result<String, JsValue> {
    // Grab the stated type of each input
    let geom1_type = get_geometry_type(&geom1)?;
    let geom2_type = get_geometry_type(&geom2)?;

    if (geom1_type.eq_ignore_ascii_case("MultiPolygon")
        || geom1_type.eq_ignore_ascii_case("Polygon"))
        && (geom2_type.eq_ignore_ascii_case("MultiPolygon")
            || geom2_type.eq_ignore_ascii_case("Polygon"))
    {
        return wkt_boolean(&geom1, &geom2, op);
    }

    Err(json_errors::geometry_processing_error::invalid_boolean_geom_pair(&geom1, &geom2))
}

/** SVG Booleans */

/// Generates a union from two SVG geometries.
///
/// It throws an error if the union operation on the two geometry types is not supported,
/// or if invalid geometries have been submitted.
///
#[wasm_bindgen(js_name = svgPolygonUnion)]
pub fn svg_polygon_union(geom1: String, geom2: String) -> Result<String, JsValue> {
    svg_polygon_boolean(&geom1, &geom2, geo_booleanop::boolean::Operation::Union)
}

/// Generates a difference from two SVG geometries.
///
/// It throws an error if the difference operation on the two geometry types is not supported,
/// or if invalid geometries have been submitted.
///
#[wasm_bindgen(js_name = svgPolygonDifference)]
pub fn svg_polygon_difference(geom1: String, geom2: String) -> Result<String, JsValue> {
    svg_polygon_boolean(
        &geom1,
        &geom2,
        geo_booleanop::boolean::Operation::Difference,
    )
}

/// Generates an intersection from two SVG geometries.
///
/// It throws an error if the intersection operation on the two geometry types is not supported,
/// or if invalid geometries have been submitted.
///
#[wasm_bindgen(js_name = svgPolygonIntersection)]
pub fn svg_polygon_intersection(geom1: String, geom2: String) -> Result<String, JsValue> {
    svg_polygon_boolean(
        &geom1,
        &geom2,
        geo_booleanop::boolean::Operation::Intersection,
    )
}

/// Generates a symmetric difference from two SVG geometries.
///
/// It throws an error if the symmetric difference operation on the two geometry types is not supported,
/// or if invalid geometries have been submitted.
///
#[wasm_bindgen(js_name = svgPolygonSymmetricDifference)]
pub fn svg_polygon_xor(geom1: String, geom2: String) -> Result<String, JsValue> {
    svg_polygon_boolean(&geom1, &geom2, geo_booleanop::boolean::Operation::Xor)
}

/// Perform a boolean operation on the submitted SVG geometries
///
/// This performs a quick test first to determine if the submitted geometry types are
/// suitable for the boolean operation (only MULTIPOLYGON and POLYGON are supported)
fn svg_polygon_boolean(
    geom1: &String,
    geom2: &String,
    op: geo_booleanop::boolean::Operation,
) -> Result<String, JsValue> {
    let g1 = match svg_to_geometry(geom1) {
        Ok(g) => g,
        Err(e) => return Err(json_errors::svg_error::could_not_parse()),
    };
    let g2 = match svg_to_geometry(geom2) {
        Ok(g) => g,
        Err(e) => return Err(json_errors::svg_error::could_not_parse()),
    };

    match geometry_boolean(&g1, &g2, op) {
        Ok(g) => Ok(g.to_svg()),
        Err(e) => Err(e),
    }
}

/// Generates a union from two SVG geometries.
///
/// It throws an error if the union operation on the two geometry types is not supported,
/// or if invalid geometries have been submitted.
///
#[wasm_bindgen(js_name = svgStringPolygonUnion)]
pub fn svg_string_polygon_union(geom1: String, geom2: String) -> Result<String, JsValue> {
    svg_string_polygon_boolean(&geom1, &geom2, geo_booleanop::boolean::Operation::Union)
}

/// Generates a difference from two SVG geometries.
///
/// It throws an error if the difference operation on the two geometry types is not supported,
/// or if invalid geometries have been submitted.
///
#[wasm_bindgen(js_name = svgStringPolygonDifference)]
pub fn svg_string_polygon_difference(geom1: String, geom2: String) -> Result<String, JsValue> {
    svg_string_polygon_boolean(
        &geom1,
        &geom2,
        geo_booleanop::boolean::Operation::Difference,
    )
}

/// Generates an intersection from two SVG geometries.
///
/// It throws an error if the intersection operation on the two geometry types is not supported,
/// or if invalid geometries have been submitted.
///
#[wasm_bindgen(js_name = svgStringPolygonIntersection)]
pub fn svg_string_polygon_intersection(geom1: String, geom2: String) -> Result<String, JsValue> {
    svg_string_polygon_boolean(
        &geom1,
        &geom2,
        geo_booleanop::boolean::Operation::Intersection,
    )
}

/// Generates a symmetric difference from two SVG geometries.
///
/// It throws an error if the symmetric difference operation on the two geometry types is not supported,
/// or if invalid geometries have been submitted.
///
#[wasm_bindgen(js_name = svgStringPolygonSymmetricDifference)]
pub fn svg_string_polygon_xor(geom1: String, geom2: String) -> Result<String, JsValue> {
    svg_string_polygon_boolean(&geom1, &geom2, geo_booleanop::boolean::Operation::Xor)
}

/// Perform a boolean operation on the submitted SVG geometries
///
/// This performs a quick test first to determine if the submitted geometry types are
/// suitable for the boolean operation (only MULTIPOLYGON and POLYGON are supported)
fn svg_string_polygon_boolean(
    geom1: &String,
    geom2: &String,
    op: geo_booleanop::boolean::Operation,
) -> Result<String, JsValue> {
    let g1 = match svg_to_geometry(geom1) {
        Ok(g) => g,
        Err(e) => return Err(json_errors::svg_error::could_not_parse()),
    };
    let g2 = match svg_to_geometry(geom2) {
        Ok(g) => g,
        Err(e) => return Err(json_errors::svg_error::could_not_parse()),
    };

    match geometry_boolean(&g1, &g2, op) {
        Ok(g) => Ok(g.to_svg_string()),
        Err(e) => Err(e),
    }
}
