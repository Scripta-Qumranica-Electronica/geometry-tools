use geo_repair_polygon::repair::Repair;
use geo_svg_io::geo_svg_reader::svg_to_geometry_collection;
use geo_types::Geometry;
use geo_validator::Validate;
use geo_wkt_writer::ToWkt;
use wasm_bindgen::prelude::*;
use wkt::Wkt;

/** Validators */

/// Tests whether an SVG element can represent a valid Geometry.
/// This function can read a <path>, <polygon>. <polyline>,
/// <rect>, and <line>, all other SVG elements will fail
/// immediately.
///
#[wasm_bindgen(js_name = svgIsValidGeom)]
pub fn svg_is_valid_geom(svg: String) -> bool {
    let geom = match svg_to_geometry_collection(&svg) {
        Ok(geom) => geom,
        Err(_) => return false,
    };
    for shape in geom.0 {
        match shape {
            Geometry::MultiPolygon { .. } => {
                if !shape.into_multi_polygon().unwrap().validate() {
                    return false;
                }
            }
            Geometry::Polygon { .. } => {
                if !shape.into_polygon().unwrap().validate() {
                    return false;
                }
            }
            Geometry::MultiLineString { .. } => {
                if shape.into_multi_line_string().is_none() {
                    return false;
                }
            }
            Geometry::LineString { .. } => {
                if shape.into_line_string().is_none() {
                    return false;
                }
            }
            Geometry::Line { .. } => {
                if shape.into_line().is_none() {
                    return false;
                }
            }
            _ => return false,
        }
    }
    false
}

/// Tests whether an SVG <path> d-string can represent a valid Geometry.
///
#[wasm_bindgen(js_name = svgPathStringIsValidGeom)]
pub fn svg_path_string_is_valid_geom(d_string: String) -> bool {
    svg_is_valid_geom(format!("<path d=\"{}\"/>", d_string))
}

/// Tests whether an SVG element is a valid polygon.
/// This function can read a <path>, <polygon>. <polyline>,
/// <rect>, and <line>, all other SVG elements will fail
/// immediately.
///
#[wasm_bindgen(js_name = validateSvgPolygon)]
pub fn validate_svg_polygon(svg: String) -> bool {
    let geom = match svg_to_geometry_collection(&svg) {
        Ok(geom) => geom,
        Err(_) => return false,
    };
    if geom.0.len() != 1 {
        return false;
    }
    let poly = match geom.0[0].clone().into_polygon() {
        Some(p) => p,
        None => return false,
    };
    poly.validate()
}

/// Tests whether an SVG <path> d-string is a valid polygon.
///
#[wasm_bindgen(js_name = validateSvgPathStringAsPolygon)]
pub fn validate_svg_path_string_as_polygon(d_string: String) -> bool {
    validate_svg_polygon(format!("<path d=\"{}\"/>", d_string))
}

/// Tests whether an SVG element represents a valid multi polygon geometry.
/// This function can read a <path>, <polygon>. <polyline>,
/// <rect>, and <line>, all other SVG elements will fail
/// immediately.
///
#[wasm_bindgen(js_name = validateSvgMultiPolygon)]
pub fn validate_svg_multi_polygon(svg: String) -> bool {
    let geom = match svg_to_geometry_collection(&svg) {
        Ok(geom) => geom,
        Err(_) => return false,
    };
    if geom.0.len() != 1 {
        return false;
    }
    let poly = match geom.0[0].clone().into_multi_polygon() {
        Some(p) => p,
        None => return false,
    };
    poly.validate()
}

/// Tests whether an SVG <path> d-string represents a valid multi polygon geometry.
///
#[wasm_bindgen(js_name = validateSvgPathStringAsMultiPolygon)]
pub fn validate_svg_path_string_as_multi_polygon(d_string: String) -> bool {
    validate_svg_multi_polygon(format!("<path d=\"{}\"/>", d_string))
}

/// Repairs a WKT geometry.
///
#[wasm_bindgen(js_name = repairWkt)]
pub fn repair_wkt(wkt: String) -> String {
    let wkt_geom: Wkt<f64> = match Wkt::from_str(&wkt) {
        Ok(g1) => g1,
        Err(_) => return "INVALIDGEOMETRY".into(),
    };

    let geo = match wkt::conversion::try_into_geometry(&wkt_geom.items[0]) {
        Ok(g1) => g1,
        Err(_) => return "INVALIDGEOMETRY".into(),
    };

    match geo.repair() {
        Some(g) => g.to_wkt(),
        None => "INVALIDGEOMETRY".into(),
    }
}
