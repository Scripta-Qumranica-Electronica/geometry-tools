use geo_svg_io::geo_svg_reader::svg_to_geometry;
use geo_svg_io::geo_svg_writer::{ToSvg, ToSvgString};
use geo_types::Geometry;
use geo_wkt_writer::ToWkt;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wkt::Wkt;

/// Convert an SVG element into a WKT representation.
/// This function can read a <path>, <polygon>. <polyline>,
/// <rect>, and <line>, all other SVG elements will return
/// and error.
///
#[wasm_bindgen(js_name = svgToWkt)]
pub fn svg_to_wkt(svg: String) -> Result<String, JsValue> {
    let geom = match svg_to_geometry(&svg) {
        Ok(geom) => geom,
        Err(_) => return Err("The submitted SVG could not be parsed into WKT".into()),
    };

    if geom.0.len() == 1 {
        let single = geom.0[0].clone();
        return match single {
            Geometry::MultiPolygon { .. } => Ok(single.into_multi_polygon().unwrap().to_wkt()),
            Geometry::Polygon { .. } => Ok(single.into_polygon().unwrap().to_wkt()),
            Geometry::MultiLineString { .. } => {
                Ok(single.into_multi_line_string().unwrap().to_wkt())
            }
            Geometry::LineString { .. } => Ok(single.into_line_string().unwrap().to_wkt()),
            Geometry::Line { .. } => Ok(single.into_line().unwrap().to_wkt()),
            _ => Ok(single.to_wkt()),
        };
    }

    Ok(geom.to_wkt())
}

/// Convert an SVG <path> d-string into a WKT representation.
/// This function can read a <path>, <polygon>. <polyline>,
/// <rect>, and <line>, all other SVG elements will return
/// and error.
///
#[wasm_bindgen(js_name = svgPathStringToWkt)]
pub fn svg_path_string_to_wkt(d_string: String) -> Result<String, JsValue> {
    svg_to_wkt(format!("<path d=\"{}\"/>", d_string))
}

/// Converts a WKT geometry into an SVG element.
///
#[wasm_bindgen(js_name = wktToSvg)]
pub fn wkt_to_svg(wkt: String) -> Result<String, JsValue> {
    let wkt_geom: Wkt<f64> = match Wkt::from_str(&wkt) {
        Ok(geom) => geom,
        Err(err) => return Err(JsValue::from_str(err.to_string().as_str())),
    };
    let geom = match wkt::conversion::try_into_geometry(&wkt_geom.items[0]) {
        Ok(parsed_geom) => parsed_geom,
        Err(err) => return Err(JsValue::from_str(err.to_string().as_str())),
    };

    match geom {
        Geometry::MultiPolygon { .. } => Ok(geom.into_multi_polygon().unwrap().to_svg()),
        Geometry::Polygon { .. } => Ok(geom.into_polygon().unwrap().to_svg()),
        Geometry::MultiLineString { .. } => Ok(geom.into_multi_line_string().unwrap().to_svg()),
        Geometry::LineString { .. } => Ok(geom.into_line_string().unwrap().to_svg()),
        Geometry::Line { .. } => Ok(geom.into_line().unwrap().to_svg()),
        _ => Ok(geom.to_svg()),
    }
}

/// Converts a WKT geometry into an SVG <path> d-string.
///
#[wasm_bindgen(js_name = wktToSvgPathString)]
pub fn wkt_to_svg_path_string(wkt: String) -> Result<String, JsValue> {
    let wkt_geom: Wkt<f64> = match Wkt::from_str(&wkt) {
        Ok(geom) => geom,
        Err(err) => return Err(JsValue::from_str(err.to_string().as_str())),
    };
    let geom = match wkt::conversion::try_into_geometry(&wkt_geom.items[0]) {
        Ok(parsed_geom) => parsed_geom,
        Err(err) => return Err(JsValue::from_str(err.to_string().as_str())),
    };

    Ok(geom.to_svg_string())
}

/** Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_svg_to_wkt() {
        let svg = r#"<path d="M0 0L10 0L10 10L0 10L0 0M3 3L6 3L6 6L3 6L3 3"/>"#;
        let wkt = svg_to_wkt(svg.into());
        assert_eq!(
            "POLYGON((0 0,0 10,10 10,10 0,0 0),(3 3,6 3,6 6,3 6,3 3))",
            wkt.ok().unwrap()
        );
    }
}
