extern crate geo_booleanop;
extern crate geo_types;
extern crate wkt;

use self::geo_types::Polygon;
use crate::information::type_of;
use crate::json_errors;
use geo_booleanop::boolean::BooleanOp;
use geo_repair_polygon::join::Join;
use geo_types::Geometry;
use geo_validator::Validate;
use geo_wkt_writer::ToWkt;
use wasm_bindgen::JsValue;
use wkt::Wkt;

/// Apply the operation function to the two geometries if possible
///
pub fn wkt_boolean(
    geom1: &str,
    geom2: &str,
    operation: geo_booleanop::boolean::Operation,
) -> Result<String, JsValue> {
    let wkt_geom1: Wkt<f64> = match Wkt::from_str(geom1) {
        Ok(g1) => g1,
        Err(err) => {
            return Err(json_errors::geometry_processing_error::invalid_geom(
                &err.to_string(),
            ))
        }
    };

    let geo_geom1 = match wkt::conversion::try_into_geometry(&wkt_geom1.items[0]) {
        Ok(g1) => g1,
        Err(err) => {
            return Err(json_errors::geometry_processing_error::invalid_geom(
                &err.to_string(),
            ))
        }
    };

    let wkt_geom2: Wkt<f64> = match Wkt::from_str(geom2) {
        Ok(g2) => g2,
        Err(err) => {
            return Err(json_errors::geometry_processing_error::invalid_geom(
                &err.to_string(),
            ))
        }
    };
    let geo_geom2 = match wkt::conversion::try_into_geometry(&wkt_geom2.items[0]) {
        Ok(g2) => g2,
        Err(err) => {
            return Err(json_errors::geometry_processing_error::invalid_geom(
                &err.to_string(),
            ))
        }
    };

    match geometry_boolean(&geo_geom1, &geo_geom2, operation) {
        Ok(g) => Ok(g.to_wkt()),
        Err(e) => Err(e),
    }
}

pub fn geometry_boolean(
    geo_geom1: &Geometry<f64>,
    geo_geom2: &Geometry<f64>,
    operation: geo_booleanop::boolean::Operation,
) -> Result<Polygon<f64>, JsValue> {
    match geo_geom1 {
        Geometry::MultiPolygon { .. } => {
            let g1 = geo_geom1.clone().into_multi_polygon().unwrap();
            if !g1.validate() {
                return Err(json_errors::wkt_errors::invalid_geometry(
                    &geo_geom1.to_wkt(),
                ));
            }
            match geo_geom2 {
                Geometry::MultiPolygon { .. } => {
                    let g2 = geo_geom2.clone().into_multi_polygon().unwrap();
                    if !g2.validate() {
                        return Err(json_errors::wkt_errors::invalid_geometry(
                            &geo_geom2.to_wkt(),
                        ));
                    }
                    Ok(g1.boolean(&g2, operation).join())
                }
                Geometry::Polygon { .. } => {
                    let g2 = geo_geom2.clone().into_polygon().unwrap();
                    if !g2.validate() {
                        return Err(json_errors::wkt_errors::invalid_geometry(
                            &geo_geom2.to_wkt(),
                        ));
                    }
                    Ok(g1.boolean(&g2, operation).join())
                }
                _ => Err(
                    json_errors::geometry_processing_error::invalid_boolean_geom_pair(
                        &type_of(&geo_geom1),
                        &type_of(&geo_geom2),
                    ),
                ),
            }
        }
        Geometry::Polygon { .. } => {
            let g1 = geo_geom1.clone().into_polygon().unwrap();
            if !g1.validate() {
                return Err(json_errors::wkt_errors::invalid_geometry(
                    &geo_geom1.to_wkt(),
                ));
            }
            match geo_geom2 {
                Geometry::MultiPolygon { .. } => {
                    let g2 = geo_geom2.clone().into_multi_polygon().unwrap();
                    if !g2.validate() {
                        return Err(json_errors::wkt_errors::invalid_geometry(
                            &geo_geom2.to_wkt(),
                        ));
                    }
                    Ok(g1.boolean(&g2, operation).join())
                }
                Geometry::Polygon { .. } => {
                    let g2 = geo_geom2.clone().into_polygon().unwrap();
                    if !g2.validate() {
                        return Err(json_errors::wkt_errors::invalid_geometry(
                            &geo_geom2.to_wkt(),
                        ));
                    }
                    Ok(g1.boolean(&g2, operation).join())
                }
                _ => Err(
                    json_errors::geometry_processing_error::invalid_boolean_geom_pair(
                        &type_of(&geo_geom1),
                        &type_of(&geo_geom2),
                    ),
                ),
            }
        }
        _ => Err(
            json_errors::geometry_processing_error::invalid_boolean_geom_pair(
                &type_of(&geo_geom1),
                &type_of(&geo_geom2),
            ),
        ),
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
        let union = wkt_boolean(&poly1, &poly2, geo_booleanop::boolean::Operation::Union);
        let expected = "POLYGON((0 0,0 10,10 10,10 0,0 0),(2 5,5 5,5 2,6 2,6 6,2 6,2 5))";
        assert!(union.is_ok());
        assert_eq!(expected, union.unwrap());
    }

    // #[test]
    // fn can_not_wkt_union_complex_polygons() {
    //     let poly1 = String::from("POLYGON((0 0,10 0,0 10,10 10,0 0),(3 3,6 3,6 6,3 6,3 3))");
    //     let poly2 = String::from("POLYGON((2 2,4 2,4 4,2 4,2 2))");
    //     let union = wkt_boolean(&poly1, &poly2, geo_booleanop::boolean::Operation::Union);
    //     let expected = "POLYGON((0 0,0 10,10 10,10 0,0 0),(2 5,5 5,5 2,6 2,6 6,2 6,2 5))";
    //     assert!(union.is_err());
    // }

    // #[test]
    // #[should_panic(
    //     expected = "WKT geometry not recognized: BAD((0 0,10 0,10 10,0 10,0 0),(2 2,6 2,6 6,2 6,2 2))"
    // )]
    // fn errors_on_unknown_geom_type() {
    //     let poly1 = String::from("BAD((0 0,10 0,10 10,0 10,0 0),(2 2,6 2,6 6,2 6,2 2))");
    //     let poly2 = String::from("POLYGON((1 1,5 1,5 5,1 5,1 1))");
    //     let union = wkt_union(&poly1, &poly2);
    //     assert!(!union.is_err());
    // }

    #[test]
    fn automatically_closes_polygons() {
        let poly1 = String::from("POLYGON((0 0,10 0,10 10,0 10),(2 2,6 2,6 6,2 6,2 2))");
        let poly2 = String::from("POLYGON((1 1,5 1,5 5,1 5))");
        let union = wkt_boolean(&poly1, &poly2, geo_booleanop::boolean::Operation::Union);
        let expected = "POLYGON((0 0,0 10,10 10,10 0,0 0),(2 5,5 5,5 2,6 2,6 6,2 6,2 5))";
        assert!(union.is_ok());
        assert_eq!(expected, union.unwrap());
    }

    // #[test]
    // fn errors_on_malformed_wkt() {
    //     let poly1 = String::from("POLYGON((0 0,10 0,10 10,0 10),2 2,6 2,6 6,2 6,2 2))");
    //     let poly2 = String::from("POLYGON((1 1,5 1,5 5,1 5))");
    //     let union = wkt_union(&poly1, &poly2);
    //     assert!(union.is_err());
    // }

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
