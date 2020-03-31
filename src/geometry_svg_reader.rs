use geo_types::{
    Coordinate, Geometry, GeometryCollection, LineString, MultiLineString, MultiPoint,
    MultiPolygon, Point, Polygon, line_string
};
use wasm_bindgen::JsValue;
use std::f64;

pub fn to_geometry(svg: &String) -> Result<Geometry<f64>, JsValue> {
    Ok(Geometry::LineString(line_string![
            (x: 11.0_f64, y: 21.0),
            (x: 34.0, y: 21.0),
            (x: 24.0, y: 54.0),
            (x: 31.50, y: 34.0),
        ]))
}

pub fn svg_d_path_to_geometry(svg: &String) -> Result<Polygon<f64>, JsValue> {
    let paths: Vec<&str> = svg.split('M').collect();
    let mut paths_iter = paths.iter();

    // Skip the first (empty) iteration
    paths_iter.next();

    // Get outer ring
    let outer_ring_line_string: LineString<f64> = LineString::from(paths_iter.next()
        .unwrap()
        .split("L")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&p| map_svg_path_coords_to_coordinate(p)).collect::<Vec<Coordinate<f64>>>());

    // Get inner rings
    let inner_rings_line_strings: Vec<LineString<f64>> = paths_iter.map(|&r|
        LineString::from(r.split("L")
            .collect::<Vec<&str>>()
            .iter()
            .map(|&p| map_svg_path_coords_to_coordinate(p)).collect::<Vec<Coordinate<f64>>>() as Vec<Coordinate<f64>>)
    ).collect();

    Ok(Polygon::new(outer_ring_line_string, inner_rings_line_strings))
}

fn map_svg_path_coords_to_coordinate(path: &str) -> Coordinate::<f64> {
    let trimmed = path.trim();
    let coords: Vec<&str> = trimmed.split(" ").collect();
    Coordinate::<f64>{x: coords[0].parse::<f64>().unwrap(), y: coords[1].parse::<f64>().unwrap()}
}

/** Tests */

#[cfg(test)]
mod tests {
    use super::*;
    use geo_types::{line_string, point, polygon};

    #[test]
    fn can_convert_svg_path() {
        let poly = polygon!(
        exterior: [
            (x: 0.0, y: 0.0),
            (x: 0.0, y: 60.0),
            (x: 60.0, y: 60.0),
            (x: 60.0, y: 0.0),
            (x: 0.0, y: 0.0),],
        interiors:[[
            (x: 10.0, y: 10.0),
            (x: 40.0, y: 1.0),
            (x: 40.0, y: 40.0),
            (x: 10.50, y: 40.0),
            (x: 10.0, y: 10.0),]
            ]
        );
        let svg_string = String::from("M0 0L0 60L60 60L60 0L0 0M10 10L40 1L40 40L10.5 40L10 10");
        let parsed_svg = svg_d_path_to_geometry(&svg_string);
        assert_eq!(parsed_svg.is_ok(), true);
        assert_eq!(parsed_svg.unwrap(), poly);
    }
}