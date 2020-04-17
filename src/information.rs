use crate::json_errors;
use geo_svg_io::geo_svg_reader::svg_to_geometry;
use geo_types::Geometry;
use wasm_bindgen::prelude::*;

/// Returns the Geometry type recognized for the submitted SVG element.
/// Only <path>, <polygon>. <polyline>, <rect>, and <line> will be recognized
/// as valid Geom types.
///
#[wasm_bindgen(js_name = svgGeomType)]
pub fn svg_geom_type(svg: String) -> String {
    let geom = match svg_to_geometry(&svg) {
        Ok(geom) => geom,
        Err(_) => return "None".into(),
    };

    if geom.0.len() > 1 {
        return "GeometryCollection".into();
    }

    match geom.0[0] {
        Geometry::MultiPolygon { .. } => "MultiPolygon".into(),
        Geometry::Polygon { .. } => "Polygon".into(),
        Geometry::MultiLineString { .. } => "MultiLineString".into(),
        Geometry::LineString { .. } => "LineString".into(),
        Geometry::Line { .. } => "Line".into(),
        _ => "None".into(),
    }
}

/// Returns the Geometry type recognized for the submitted SVG path d-string.
///
#[wasm_bindgen(js_name = svgPathGeomType)]
pub fn svg_path_geom_type(d_string: String) -> String {
    svg_geom_type(format!("<path d=\"{}\"/>", d_string))
}

/// This function reads a submitted string and makes a very quick decision
/// about the WKT geometry type it must contain. This function does not
/// actually check if the string really contains the shape it claims to
/// contain, nor does it do any processing or validation. All validation should
/// be down downstream.
///
pub fn get_geometry_type(geom: &str) -> Result<String, JsValue> {
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
