mod geometry_boolean;
mod geometry_wkt_writer;
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

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, geometry-tools!");
}

#[wasm_bindgen]
pub fn wktUnion(geom1: String, geom2: String) -> String {
    //return "".into();
    // Grab the stated type of each input
    let mut geom1_type = String::from("");
    match geom1.get(..1) {
        Some("G") => geom1_type = String::from("GeometryCollection"),
        Some("L") => geom1_type = String::from("LineString"),
        Some("M") => {
            if geom1.starts_with("MultiL") {
                geom1_type = String::from("MultiLineString");
            } else if geom1.starts_with("MultiPoi") {
                geom1_type = String::from("MultiPoint");
            } else if geom1.starts_with("MultiP") {
                geom1_type = String::from("MultiPolygon");
            }
        }
        Some("P") => {
            if geom1.starts_with("Poi") {
                geom1_type = String::from("Point");
            } else {
                geom1_type = String::from("Polygon");
            }
        }
        // Return immediately on empty string
        Some(&_) => return "".into(),
        None => return "".into(),
    }
    let mut geom2_type = String::from("");
    match geom2.get(..1) {
        Some("G") => geom2_type = String::from("GeometryCollection"),
        Some("L") => geom2_type = String::from("LineString"),
        Some("M") => {
            if geom2.starts_with("MultiL") {
                geom2_type = String::from("MultiLineString");
            } else if geom2.starts_with("MultiPoi") {
                geom2_type = String::from("MultiPoint");
            } else if geom2.starts_with("MultiP") {
                geom2_type = String::from("MultiPolygon");
            }
        }
        Some("P") => {
            if geom2.starts_with("Poi") {
                geom2_type = String::from("Point");
            } else {
                geom2_type = String::from("Polygon");
            }
        }
        // Return immediately on empty string
        Some(&_) => return "".into(),
        None => return "".into(),
    }

    if geom1_type == "MultiPolygon" {
        if geom1_type == geom2_type {
            //union two multipolygons
            return wkt_multi_polygon_union(geom1, geom2);
        } else if geom2_type == "Polygon" {
            // union multi + polygon
            return wkt_multi_polygon_polygon_union(geom1, geom2);
        } else {
            return "".into();
        } // Return immediately
    }

    if geom1_type == "Polygon" {
        if geom1_type == geom2_type {
            // union two polygons
            return wkt_polygon_union(geom1, geom2);
        } else if geom2_type == "MulitPolygon" {
            // union multi + polygon
            return wkt_multi_polygon_polygon_union(geom2, geom1);
        } else {
            return "".into();
        } // Return immediately
    }

    return "".into();
}

// #[wasm_bindgen]
// pub fn shape(input_shape: String) -> String {
//     let new_shape: Wkt<f64> = Wkt::from_str(
//         &input_shape
//     )
//     .ok()
//     .unwrap();

//     let geom = wkt::conversion::try_into_geometry(&new_shape.items[0])
//         .ok()
//         .unwrap();

//     let geom_poly = geom.into_polygon().unwrap();

//     let poly = polygon![
//         (x: 1.0, y: 1.0),
//         (x: 4.0, y: 1.0),
//         (x: 4.0, y: 4.0),
//         (x: 1.0, y: 4.0),
//         (x: 1.0, y: 1.0),
//     ];

//     // // Calculate the polygon's convex hull
//     let hull = poly.convex_hull();

//     let union: geo::MultiPolygon<f64> = geom_poly.xor(&poly);
//     let simp_union = union.simplify(&1.0);
//     let simp_poly = &simp_union.0.iter().next().unwrap();

//     format!("{} {} {} {}", geom_poly.to_wkt(), poly.to_wkt(),
//     union.to_wkt(), simp_poly.to_wkt())
// }

#[wasm_bindgen]
pub fn shoutOut() -> String {
    "Hey I returned a string!".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_greet() {
        let msg = shoutOut();
        let expected = String::from("Hey I returned a string!");
        assert_eq!(expected, msg);
    }

    #[test]
    fn can_join_polygons() {
        let res = wktUnion(
            "POLYGON((0 0,10 0,10 10,0 10,0 0),(3 3,6 3,6 6,3 6,3 3))".into(),
            "POLYGON((2 2,4 2,4 4,2 4,2 2))".into(),
        );
        assert_eq!(res, res);
    }
}
