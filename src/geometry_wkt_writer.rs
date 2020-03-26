extern crate geo;

use geo_types::{
    Coordinate, 
    Point, 
    MultiPoint, 
    LineString, 
    MultiLineString, 
    Polygon, 
    MultiPolygon, 
    Geometry, 
    GeometryCollection};
use std::fmt;

pub trait ToWkt {
    fn to_wkt(&self) -> String;
}

/** Geometries */

impl<T> ToWkt for GeometryCollection<T> where 
T: num_traits::Float + fmt::Display {
    fn to_wkt(&self) -> String 
    {
        if self.is_empty() {
            "GEOMETRYCOLLECTION EMPTY".into()
        } else {
            format!(
                "GEOMETRYCOLLECTION({})", 
                self.0
                    .iter()
                    .map(|p| p.to_wkt())
                    .collect::<Vec<String>>()
                    .join(",")
            )
        }
    }
}

impl<T> ToWkt for Geometry<T> where 
T: num_traits::Float + fmt::Display {
    fn to_wkt(&self) -> String 
    {
        let multi_polygon = self.clone().into_multi_polygon();
        if multi_polygon.is_some() {
            return multi_polygon.unwrap().to_wkt();
        }
        let polygon = self.clone().into_polygon();
        if polygon.is_some() {
            return polygon.unwrap().to_wkt();
        }
        let multi_line_string = self.clone().into_multi_line_string();
        if multi_line_string.is_some() {
            return multi_line_string.unwrap().to_wkt();
        }
        let line_string = self.clone().into_line_string();
        if line_string.is_some() {
            return line_string.unwrap().to_wkt();
        }
        let multi_point = self.clone().into_multi_point();
        if multi_point.is_some() {
            return multi_point.unwrap().to_wkt();
        }
        let point = self.clone().into_point();
        if point.is_some() {
            return point.unwrap().to_wkt();
        }
        
        "GEOMETRYCOLLECTION EMPTY".into()
    }
}

/** Polygons */

impl<T> ToWkt for MultiPolygon<T> where 
T: num_traits::Float + fmt::Display {
    fn to_wkt(&self) -> String 
    {
        multi_polygon_to_wkt(self)
    }
}

fn multi_polygon_to_wkt<T>(poly: &MultiPolygon<T>) -> String where 
T: num_traits::Float + fmt::Display
{
    if poly.0.is_empty() {
        "MULTIPOLYGON EMPTY".into()
    } else {
        format!(
            "MULTIPOLYGON((({})))", 
            poly
                .0
                .iter()
                .map(|p| polygon_linestrings_to_wkt(&p))
                .collect::<Vec<String>>()
                .join(")),((")
        )
    }
}

impl<T> ToWkt for Polygon<T> where 
T: num_traits::Float + fmt::Display {
    fn to_wkt(&self) -> String 
    {
        polygon_to_wkt(self)
    }
}

fn polygon_to_wkt<T>(poly: &Polygon<T>) -> String where 
T: num_traits::Float + fmt::Display
{
    if poly.exterior().0.is_empty() {
        "POLYGON EMPTY".into()
    } else {
        format!(
            "POLYGON(({}))", 
            polygon_linestrings_to_wkt(poly)
        )
    }
}

fn polygon_linestrings_to_wkt<T>(poly: &Polygon<T>) -> String where 
T: num_traits::Float + fmt::Display
{
    let mut lines: Vec<LineString<T>> = poly.interiors().into();
    let exterior: &LineString<T> = poly.exterior().into();
    lines.insert(0, exterior.clone());

    lines.iter()
        .map(|l| line_to_wkt(&l))
        .collect::<Vec<String>>()
        .join("),(")
}

/** Lines */

impl<T> ToWkt for MultiLineString<T> where 
T: num_traits::Float + fmt::Display {
    fn to_wkt(&self) -> String 
    {
        multi_linestring_to_wkt(self)
    }
}

fn multi_linestring_to_wkt<T>(multi_line: &MultiLineString<T>) -> String where 
T: num_traits::Float + fmt::Display
{
    if multi_line.0.is_empty() {
        "MULTILINESTRING EMPTY".into()
    } else {
        format!(
            "MULTILINESTRING(({}))", 
            multi_line
                .0
                .iter()
                .map(|l| line_to_wkt(&l))
                .collect::<Vec<String>>()
                .join("),(")
        )
    }
}

impl<T> ToWkt for LineString<T> where 
T: num_traits::Float + fmt::Display {
    fn to_wkt(&self) -> String 
    {
        linestring_to_wkt(self)
    }
}

fn linestring_to_wkt<T>(line: &LineString<T>) -> String where 
T: num_traits::Float + fmt::Display
{
    if line.0.is_empty() {
        "LINESTRING EMPTY".into()
    } else {
        format!("LINESTRING({})", line_to_wkt(line))
    }
}

fn line_to_wkt<T>(line: &LineString<T>) -> String where 
T: num_traits::Float + fmt::Display
{
    line.0.iter()
        .map(|c| coord_to_wkt(&c))
        .collect::<Vec<String>>()
        .join(",")
}

/** Points */

impl<T> ToWkt for MultiPoint<T> where 
T: num_traits::Float + fmt::Display {
    fn to_wkt(&self) -> String 
    {
        multi_point_to_wkt(self)
    }
}

fn multi_point_to_wkt<T>(multi_point: &MultiPoint<T>) -> String where 
T: num_traits::Float + fmt::Display
{
    if multi_point.0.is_empty() {
        "MULTIPOINT EMPTY".into()
    } else {
        format!(
            "MULTIPOINT({})", 
            multi_point
                .0
                .iter()
                .map(|p| point_to_string(&p))
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl<T> ToWkt for Point<T> where 
T: num_traits::Float + fmt::Display {
    fn to_wkt(&self) -> String 
    {
        point_to_wkt(self)
    }
}

fn point_to_wkt<T>(point: &Point<T>) -> String where 
T: num_traits::Float + fmt::Display
{
    format!("POINT({})", point_to_string(point))
}

fn point_to_string<T>(point: &Point<T>) -> String where 
T: num_traits::Float + fmt::Display
{
    coord_to_wkt(&point.0) 
}

fn coord_to_wkt<T>(coord: &Coordinate<T>) -> String where 
T: num_traits::Float + fmt::Display
{
    format!("{} {}", coord.x, coord.y)
}

/** Tests */

#[cfg(test)]
mod tests {
    use super::*;
    use geo_types::{line_string, point, polygon};

    
    #[test]
    fn can_format_geom_collection() {
        let poly = Geometry::Polygon(polygon![
            (x: 1.0, y: 1.0),
            (x: 4.0, y: 1.0),
            (x: 4.0, y: 4.0),
            (x: 1.0, y: 4.0),
            (x: 1.0, y: 1.0),
        ]);
        let pe = Geometry::Point(point!(x: 1.0, y: 1.0));
        let gc = GeometryCollection(vec![pe, poly]);
        let wkt_out = gc.to_wkt();
        let expected = String::from("GEOMETRYCOLLECTION(POINT(1 1),POLYGON((1 1,4 1,4 4,1 4,1 1)))");
        assert_eq!(wkt_out, expected);
    }

    #[test]
    fn can_format_empty_geom_collection() {
        let gc = GeometryCollection(vec![] as Vec<Geometry<f64>>);
        let wkt_out = gc.to_wkt();
        let expected = String::from("GEOMETRYCOLLECTION EMPTY");
        assert_eq!(wkt_out, expected);
    }

    #[test]
    fn can_format_multi_polygon() {
        let poly1 = polygon![
            (x: 1.0, y: 1.0),
            (x: 4.0, y: 1.0),
            (x: 4.0, y: 4.0),
            (x: 1.0, y: 4.0),
            (x: 1.0, y: 1.0),
        ];
        let poly2 = polygon!(
            exterior: [
                (x: 0.0, y: 0.0),
                (x: 6.0, y: 0.0),
                (x: 6.0, y: 6.0),
                (x: 0.0, y: 6.0),
                (x: 0.0, y: 0.0),],
            interiors:[[
                (x: 1.0, y: 1.0),
                (x: 4.0, y: 1.0),
                (x: 4.0, y: 4.0),
                (x: 1.50, y: 4.0),
                (x: 1.0, y: 1.0),]
                ]
            );
        let mp = MultiPolygon(vec![poly1, poly2]);
        let wkt_out = mp.to_wkt();
        let expected = String::from("MULTIPOLYGON(((1 1,4 1,4 4,1 4,1 1)),((0 0,6 0,6 6,0 6,0 0),(1 1,4 1,4 4,1.5 4,1 1)))");
        assert_eq!(wkt_out, expected);
    }

    #[test]
    fn can_format_empty_multi_polygon() {
        let mp = MultiPolygon(vec![] as Vec<Polygon<f64>>);
        let wkt_out = mp.to_wkt();
        let expected = String::from("MULTIPOLYGON EMPTY");
        assert_eq!(wkt_out, expected);
    }

    #[test]
    fn can_format_polygon() {
        let poly = polygon![
            (x: 1.0, y: 1.0),
            (x: 4.0, y: 1.0),
            (x: 4.0, y: 4.0),
            (x: 1.0, y: 4.0),
            (x: 1.0, y: 1.0),
        ];
        let wkt_out = poly.to_wkt();
        let expected = String::from("POLYGON((1 1,4 1,4 4,1 4,1 1))");
        assert_eq!(wkt_out, expected);
    }

    #[test]
    fn can_format_empty_polygon() {
        let poly: Polygon<f64> = Polygon::new(
            LineString::from(vec![] as Vec<Coordinate<f64>>),
            vec![],
        );
        let wkt_out = poly.to_wkt();
        let expected = String::from("POLYGON EMPTY");
        assert_eq!(wkt_out, expected);
    }

    #[test]
    fn can_format_polygon_with_hole() {
        let poly = polygon!(
            exterior: [
                (x: 0.0, y: 0.0),
                (x: 6.0, y: 0.0),
                (x: 6.0, y: 6.0),
                (x: 0.0, y: 6.0),
                (x: 0.0, y: 0.0),],
            interiors:[[
                (x: 1.0, y: 1.0),
                (x: 4.0, y: 1.0),
                (x: 4.0, y: 4.0),
                (x: 1.50, y: 4.0),
                (x: 1.0, y: 1.0),]
                ]
            );
        let wkt_out = poly.to_wkt();
        let expected = String::from("POLYGON((0 0,6 0,6 6,0 6,0 0),(1 1,4 1,4 4,1.5 4,1 1))");
        assert_eq!(wkt_out, expected);
    }

    #[test]
    fn can_format_multi_line_string() {
        let line1 = line_string![
            (x: 1.0, y: 1.0),
            (x: 4.0, y: 1.0),
            (x: 4.0, y: 4.0),
            (x: 1.50, y: 4.0),
        ];
        let line2 = line_string![
            (x: 11.0, y: 21.0),
            (x: 34.0, y: 21.0),
            (x: 24.0, y: 54.0),
            (x: 31.50, y: 34.0),
        ];
        let ml = MultiLineString(vec![line1, line2]);
        let wkt_out = ml.to_wkt();
        let expected = String::from("MULTILINESTRING((1 1,4 1,4 4,1.5 4),(11 21,34 21,24 54,31.5 34))");
        assert_eq!(wkt_out, expected);
    }

    #[test]
    fn can_format_empty_multi_line_string() {
        let ml = MultiLineString(vec![] as Vec<LineString<f64>>);
        let wkt_out = ml.to_wkt();
        let expected = String::from("MULTILINESTRING EMPTY");
        assert_eq!(wkt_out, expected);
    }

    #[test]
    fn can_format_line_string() {
        let line = line_string![
            (x: 1.0, y: 1.0),
            (x: 4.0, y: 1.0),
            (x: 4.0, y: 4.0),
            (x: 1.50, y: 4.0),
            (x: 1.0, y: 1.0),
        ];
        let wkt_out = line.to_wkt();
        let expected = String::from("LINESTRING(1 1,4 1,4 4,1.5 4,1 1)");
        assert_eq!(wkt_out, expected);
    }

    #[test]
    fn can_format_empty_line_string() {
        let line = LineString::from(vec![] as Vec<Coordinate<f64>>);
        let wkt_out = line.to_wkt();
        let expected = String::from("LINESTRING EMPTY");
        assert_eq!(wkt_out, expected);
    }

    #[test]
    fn can_format_multi_point() {
        let point1 = point!(x: 22.200, y: 31.0);
        let point2 = point!(x: 4356.0, y: 1002.345);
        let mp = MultiPoint(vec![point1, point2]);
        let wkt_out = mp.to_wkt();
        let expected = String::from("MULTIPOINT(22.2 31,4356 1002.345)");
        assert_eq!(wkt_out, expected);
    }

    #[test]
    fn can_format_empty_multi_point() {
        let mp = MultiPoint(vec![] as Vec<Point<f64>>);
        let wkt_out = mp.to_wkt();
        let expected = String::from("MULTIPOINT EMPTY");
        assert_eq!(wkt_out, expected);
    }

    #[test]
    fn can_format_point() {
        let point = point!(x: 22.200, y: 31.0);
        let wkt_out = point.to_wkt();
        let expected = String::from("POINT(22.2 31)");
        assert_eq!(wkt_out, expected);
    }
}