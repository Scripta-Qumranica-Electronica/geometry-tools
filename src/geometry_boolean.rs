extern crate wkt;
extern crate geo_types;
extern crate geo_booleanop;

use wkt::Wkt;
use geo_types::{Geometry, Polygon};
use geo_booleanop::boolean::BooleanOp;
use crate::geometry_wkt_writer::ToWkt;
use std::convert::TryFrom;
use geo_types::CoordinateType;
use num_traits::Float;

struct BoolPair<T> 
where T: num_traits::Float + std::str::FromStr + std::default::Default
{
    geom1: Geometry<T>,
    geom2: Geometry<T>
}

pub fn wkt_polygon_union(geom1: String, geom2: String) -> String 
{
    let pair: BoolPair<f64> = geom_pair_from_wkt(geom1, geom2);
    let poly1 = pair.geom1.into_polygon().unwrap();
    let poly2 = pair.geom2.into_polygon().unwrap();

    let union = poly1.union(&poly2);
    union.to_wkt()
}

pub fn wkt_multi_polygon_polygon_union(geom1: String, geom2: String) -> String 
{
    let pair: BoolPair<f64> = geom_pair_from_wkt(geom1, geom2);
    let poly1 = pair.geom1.into_multi_polygon().unwrap();
    let poly2 = pair.geom2.into_polygon().unwrap();

    let union = poly1.union(&poly2);
    union.to_wkt()
}

pub fn wkt_multi_polygon_union(geom1: String, geom2: String) -> String 
{
    let pair: BoolPair<f64> = geom_pair_from_wkt(geom1, geom2);
    let poly1 = pair.geom1.into_multi_polygon().unwrap();
    let poly2 = pair.geom2.into_multi_polygon().unwrap();

    let union = poly1.union(&poly2);
    union.to_wkt()
}

fn geom_pair_from_wkt<T>(geom1: String, geom2: String) -> BoolPair<T> 
where T: num_traits::Float + std::str::FromStr + std::default::Default
{
    let wkt_geom1: Wkt<T> = Wkt::from_str(&geom1)
        .ok()
        .unwrap();
    let geo_geom1 = wkt::conversion::try_into_geometry(&wkt_geom1.items[0])
        .ok()
        .unwrap();
    
    let wkt_geom2: Wkt<T> = Wkt::from_str(&geom2)
        .ok()
        .unwrap();
    let geo_geom2 = wkt::conversion::try_into_geometry(&wkt_geom2.items[0])
        .ok()
        .unwrap();
    
    BoolPair {geom1: geo_geom1, geom2: geo_geom2}
}

/** Tests */

#[cfg(test)]
mod tests {
    use super::*;
    use geo_types::{Point, MultiPoint, LineString, MultiLineString, Polygon, MultiPolygon, GeometryCollection};

    #[test]
    fn can_wkt_union_polygons(){
        let poly1 = String::from("POLYGON((0 0,10 0,10 10,0 10,0 0),(2 2,6 2,6 6,2 6,2 2))");
        let poly2 = String::from("POLYGON((1 1,5 1,5 5,1 5,1 1))");
        let union = wkt_polygon_union(poly1, poly2);
        let expected = "MULTIPOLYGON(((0 0,10 0,10 10,0 10,0 0),(2 5,5 5,5 2,6 2,6 6,2 6,2 5)))";
        assert_eq!(expected, union);
    }
}