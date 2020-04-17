extern crate geo_booleanop;
extern crate geo_types;
extern crate wkt;

use crate::json_errors;
use geo_booleanop::boolean::BooleanOp;
use geo_types::Geometry;
use geo_validator::Validate;
use geo_wkt_writer::ToWkt;
use wasm_bindgen::JsValue;
use wkt::Wkt;

struct BoolPair<T>
where
    T: num_traits::Float + std::str::FromStr + std::default::Default,
{
    geom1: Geometry<T>,
    geom2: Geometry<T>,
}

pub fn wkt_polygon_union(geom1: &str, geom2: &str) -> Result<String, JsValue> {
    let pair: BoolPair<f64> = geom_pair_from_wkt(geom1, geom2);
    let poly1 = pair
        .geom1
        .into_polygon()
        .unwrap_or_else(|| panic![format!("Could not convert geometry to polygon: {}", geom1)]);

    if !poly1.validate() {
        return Err(json_errors::boolean_geometry_errors::invalid_geom(
            &"invalid polygon",
        ));
    }

    let poly2 = pair
        .geom2
        .into_polygon()
        .unwrap_or_else(|| panic![format!("Could not convert geometry to polygon: {}", geom2)]);

    let valid_poly_2 = poly1.validate_detailed();
    if !valid_poly_2.valid {
        return Err(json_errors::boolean_geometry_errors::invalid_geom(
            &"invalid polygon",
        ));
    }

    let union = poly1.union(&poly2);
    Ok(union.to_wkt())
}

pub fn wkt_multi_polygon_polygon_union(geom1: &str, geom2: &str) -> String {
    let pair: BoolPair<f64> = geom_pair_from_wkt(geom1, geom2);
    let poly1 = pair.geom1.into_multi_polygon().unwrap_or_else(|| {
        panic![format!(
            "Could not convert geometry to multipolygon: {}",
            geom1
        )]
    });
    let poly2 = pair
        .geom2
        .into_polygon()
        .unwrap_or_else(|| panic![format!("Could not convert geometry to polygon: {}", geom2)]);

    let union = poly1.union(&poly2);
    union.to_wkt()
}

pub fn wkt_multi_polygon_union(geom1: &str, geom2: &str) -> String {
    let pair: BoolPair<f64> = geom_pair_from_wkt(geom1, geom2);
    let poly1 = pair.geom1.into_multi_polygon().unwrap_or_else(|| {
        panic![format!(
            "Could not convert geometry to multipolygon: {}",
            geom1
        )]
    });
    let poly2 = pair.geom2.into_multi_polygon().unwrap_or_else(|| {
        panic![format!(
            "Could not convert geometry to multipolygon: {}",
            geom2
        )]
    });

    let union = poly1.union(&poly2);
    union.to_wkt()
}

fn geom_pair_from_wkt<T>(geom1: &str, geom2: &str) -> BoolPair<T>
where
    T: num_traits::Float + std::str::FromStr + std::default::Default,
{
    let wkt_geom1: Wkt<T> = Wkt::from_str(geom1)
        .ok()
        .unwrap_or_else(|| panic![format!("WKT geometry not recognized: {}", geom1)]);
    let geo_geom1 = wkt::conversion::try_into_geometry(&wkt_geom1.items[0])
        .ok()
        .unwrap_or_else(|| panic![format!("WKT geometry could not be parsed: {}", geom1)]);

    let wkt_geom2: Wkt<T> = Wkt::from_str(geom2)
        .ok()
        .unwrap_or_else(|| panic![format!("WKT geometry not recognized: {}", geom1)]);
    let geo_geom2 = wkt::conversion::try_into_geometry(&wkt_geom2.items[0])
        .ok()
        .unwrap_or_else(|| panic![format!("WKT geometry could not be parsed: {}", geom1)]);

    BoolPair {
        geom1: geo_geom1,
        geom2: geo_geom2,
    }
}

/** Tests */

#[cfg(test)]
mod tests {
    use super::*;
    use geo_types::{LineString, MultiLineString, MultiPoint, MultiPolygon, Point, Polygon};

    #[test]
    fn can_wkt_union_polygons() {
        let poly1 = String::from("POLYGON((0 0,10 0,10 10,0 10,0 0),(2 2,6 2,6 6,2 6,2 2))");
        let poly2 = String::from("POLYGON((1 1,5 1,5 5,1 5,1 1))");
        let union = wkt_polygon_union(&poly1, &poly2);
        let expected = "MULTIPOLYGON(((0 0,0 10,10 10,10 0,0 0),(2 5,5 5,5 2,6 2,6 6,2 6,2 5)))";
        assert_eq!(expected, union.unwrap());
    }

    #[test]
    #[should_panic(
        expected = "WKT geometry not recognized: BAD((0 0,10 0,10 10,0 10,0 0),(2 2,6 2,6 6,2 6,2 2))"
    )]
    fn errors_on_unknown_geom_type() {
        let poly1 = String::from("BAD((0 0,10 0,10 10,0 10,0 0),(2 2,6 2,6 6,2 6,2 2))");
        let poly2 = String::from("POLYGON((1 1,5 1,5 5,1 5,1 1))");
        let union = wkt_polygon_union(&poly1, &poly2);
    }

    #[test]
    fn automatically_closes_polygons() {
        let poly1 = String::from("POLYGON((0 0,10 0,10 10,0 10),(2 2,6 2,6 6,2 6,2 2))");
        let poly2 = String::from("POLYGON((1 1,5 1,5 5,1 5))");
        let union = wkt_polygon_union(&poly1, &poly2);
        let expected = "MULTIPOLYGON(((0 0,0 10,10 10,10 0,0 0),(2 5,5 5,5 2,6 2,6 6,2 6,2 5)))";
        assert_eq!(expected, union.unwrap());
    }

    #[test]
    #[should_panic(
        expected = "WKT geometry not recognized: POLYGON((0 0,10 0,10 10,0 10),2 2,6 2,6 6,2 6,2 2))"
    )]
    fn errors_on_malformed_wkt() {
        let poly1 = String::from("POLYGON((0 0,10 0,10 10,0 10),2 2,6 2,6 6,2 6,2 2))");
        let poly2 = String::from("POLYGON((1 1,5 1,5 5,1 5))");
        let union = wkt_polygon_union(&poly1, &poly2);
        let expected = "MULTIPOLYGON(((0 0,10 0,10 10,0 10,0 0),(2 5,5 5,5 2,6 2,6 6,2 6,2 5)))";
        assert_eq!(expected, union.unwrap());
    }

    /*#[test]
    #[should_panic(expected = "WKT geometry not recognized: POLYGON((0 0,10 0,10 10,0 10),2 2,6 2,6 6,2 6,2 2))")]
    fn errors_on_invalid_geometry() {
        let poly1 = String::from("POLYGON((0 0,10 0,0 10,10 10,0 0),(2 2,6 2,6 6,2 6,2 2))");
        let poly2 = String::from("POLYGON((1 1,5 1,5 5,1 5))");
        let union = wkt_polygon_union(&poly1, &poly2);
        let expected = "MULTIPOLYGON(((0 0,10 0,10 10,0 10,0 0),(2 5,5 5,5 2,6 2,6 6,2 6,2 5)))";
        assert_eq!(expected, union);
    }*/
}
