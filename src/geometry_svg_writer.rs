extern crate geo;

use geo_types::{
    Coordinate, Geometry, GeometryCollection, Line, LineString, MultiLineString, MultiPoint,
    MultiPolygon, Point, Polygon, Rect, Triangle
};
use std::fmt;

pub trait ToSvg {
    fn to_svg(&self) -> String;
}

/** Geometries */

impl<T> ToSvg for GeometryCollection<T>
    where
        T: num_traits::Float + fmt::Display,
{
    fn to_svg(&self) -> String {
        if self.is_empty() {
            "".into()
        } else {
            self.0
                .iter()
                .map(|p| p.to_svg())
                .collect::<Vec<String>>()
                .join("\n")
        }
    }
}

impl<T> ToSvg for Geometry<T>
    where
        T: num_traits::Float + fmt::Display,
{
    fn to_svg(&self) -> String {
        let multi_polygon = self.clone().into_multi_polygon();
        if multi_polygon.is_some() {
            return multi_polygon.unwrap().to_svg();
        }
        let polygon = self.clone().into_polygon();
        if polygon.is_some() {
            return polygon.unwrap().to_svg();
        }
        let multi_line_string = self.clone().into_multi_line_string();
        if multi_line_string.is_some() {
            return multi_line_string.unwrap().to_svg();
        }
        let line_string = self.clone().into_line_string();
        if line_string.is_some() {
            return line_string.unwrap().to_svg();
        }

        "".into()
    }
}

/** Polygons */

impl<T> ToSvg for MultiPolygon<T>
    where
        T: num_traits::Float + fmt::Display,
{
    fn to_svg(&self) -> String {
        multi_polygon_to_svg(self)
    }
}

fn multi_polygon_to_svg<T>(poly: &MultiPolygon<T>) -> String
    where
        T: num_traits::Float + fmt::Display,
{
    if poly.0.is_empty() {
        "".into()
    } else {
        poly.0
            .iter()
            .map(|p| polygon_to_svg(&p))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl<T> ToSvg for Polygon<T>
    where
        T: num_traits::Float + fmt::Display,
{
    fn to_svg(&self) -> String {
        polygon_to_svg(self)
    }
}

fn polygon_to_svg<T>(poly: &Polygon<T>) -> String
    where
        T: num_traits::Float + fmt::Display,
{
    if poly.exterior().0.is_empty() {
        "".into()
    } else {
        format!("<path d=\"M{}\"/>", polygon_rings_to_svg(poly))
    }
}

fn polygon_rings_to_svg<T>(poly: &Polygon<T>) -> String
    where
        T: num_traits::Float + fmt::Display,
{
    let mut lines: Vec<LineString<T>> = poly.interiors().into();
    let exterior: &LineString<T> = poly.exterior().into();
    lines.insert(0, exterior.clone());

    lines
        .iter()
        .map(|l| poly_ring_to_svg(&l))
        .collect::<Vec<String>>()
        .join("M")
}

fn poly_ring_to_svg<T>(line: &LineString<T>) -> String
    where
        T: num_traits::Float + fmt::Display,
{
    line.0
        .iter()
        .map(|c| coord_to_svg(&c))
        .collect::<Vec<String>>()
        .join("L")
}

/** Rect */

impl<T> ToSvg for Rect<T>
    where
        T: num_traits::Float + fmt::Display,
{
    fn to_svg(&self) -> String {
        rect_to_svg(self)
    }
}

fn rect_to_svg<T>(rect: &Rect<T>) -> String
    where
        T: num_traits::Float + fmt::Display,
{
    format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/>", rect.min.x, rect.min.y,
            rect.width(), rect.height())
}

/** Triangle */

impl<T> ToSvg for Triangle<T>
    where
        T: num_traits::Float + fmt::Display,
{
    fn to_svg(&self) -> String {
        triangle_to_svg(self)
    }
}

fn triangle_to_svg<T>(triangle: &Triangle<T>) -> String
    where
        T: num_traits::Float + fmt::Display,
{
    format!("<polygon points=\"{},{} {},{} {},{}\"/>", triangle.0.x, triangle.0.y, triangle.1.x, triangle.1.y,
        triangle.2.x, triangle.2.y)
}

/** Lines */

impl<T> ToSvg for MultiLineString<T>
    where
        T: num_traits::Float + fmt::Display,
{
    fn to_svg(&self) -> String {
        multi_linestring_to_svg(self)
    }
}

fn multi_linestring_to_svg<T>(multi_line: &MultiLineString<T>) -> String
    where
        T: num_traits::Float + fmt::Display,
{
    if multi_line.0.is_empty() {
        "".into()
    } else {
        multi_line
            .0
            .iter()
            .map(|l| linestring_to_svg(&l))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl<T> ToSvg for LineString<T>
    where
        T: num_traits::Float + fmt::Display,
{
    fn to_svg(&self) -> String {
        linestring_to_svg(self)
    }
}

fn linestring_to_svg<T>(line: &LineString<T>) -> String
    where
        T: num_traits::Float + fmt::Display,
{
    if line.0.is_empty() {
        "".into()
    } else {
        format!("<polyline points=\"{}\"/>", line_to_svg(line))
    }
}

fn line_to_svg<T>(line: &LineString<T>) -> String
    where
        T: num_traits::Float + fmt::Display,
{
    line.0
        .iter()
        .map(|c| coord_to_svg_point(&c))
        .collect::<Vec<String>>()
        .join(" ")
}

/** Line */

impl<T> ToSvg for Line<T>
    where
        T: num_traits::Float + fmt::Display,
{
    fn to_svg(&self) -> String {
        single_line_to_svg(self)
    }
}

fn single_line_to_svg<T>(line: &Line<T>) -> String
    where
        T: num_traits::Float + fmt::Display,
{
    format!("<line x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/>",
            line.start.x, line.end.x, line.start.y, line.end.y)
}

/** Points */

fn coord_to_svg<T>(coord: &Coordinate<T>) -> String
    where
        T: num_traits::Float + fmt::Display,
{
    format!("{} {}", coord.x, coord.y)
}

fn coord_to_svg_point<T>(coord: &Coordinate<T>) -> String
    where
        T: num_traits::Float + fmt::Display,
{
    format!("{},{}", coord.x, coord.y)
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
        let line = Geometry::LineString(line_string![
            (x: 11.0, y: 21.0),
            (x: 34.0, y: 21.0),
            (x: 24.0, y: 54.0),
            (x: 31.50, y: 34.0),
        ]);
        let gc = GeometryCollection(vec![line, poly]);
        let wkt_out = gc.to_svg();
        let expected =
            String::from("<polyline points=\"11,21 34,21 24,54 31.5,34\"/>\n<path d=\"M1 1L4 1L4 4L1 4L1 1\"/>");
        assert_eq!(wkt_out, expected);
    }

    #[test]
    fn can_format_empty_geom_collection() {
        let gc = GeometryCollection(vec![] as Vec<Geometry<f64>>);
        let wkt_out = gc.to_svg();
        let expected = String::from("");
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
        let wkt_out = mp.to_svg();
        let expected = String::from(
            "<path d=\"M1 1L4 1L4 4L1 4L1 1\"/>\n<path d=\"M0 0L6 0L6 6L0 6L0 0M1 1L4 1L4 4L1.5 4L1 1\"/>",
        );
        assert_eq!(wkt_out, expected);
    }

    #[test]
    fn can_format_empty_multi_polygon() {
        let mp = MultiPolygon(vec![] as Vec<Polygon<f64>>);
        let wkt_out = mp.to_svg();
        let expected = String::from("");
        assert_eq!(wkt_out, expected);
    }

    #[test]
    fn can_format_polygon() {
        let poly = polygon![
            (x: 1.0, y: 1.0),
            (x: 40.0, y: 1.0),
            (x: 40.0, y: 40.0),
            (x: 1.0, y: 40.0),
            (x: 1.0, y: 1.0),
        ];
        let wkt_out = poly.to_svg();
        let expected = String::from("<path d=\"M1 1L40 1L40 40L1 40L1 1\"/>");
        assert_eq!(wkt_out, expected);
    }

    #[test]
    fn can_format_empty_polygon() {
        let poly: Polygon<f64> =
            Polygon::new(LineString::from(vec![] as Vec<Coordinate<f64>>), vec![]);
        let wkt_out = poly.to_svg();
        let expected = String::from("");
        assert_eq!(wkt_out, expected);
    }

    #[test]
    fn can_format_polygon_with_hole() {
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
        let wkt_out = poly.to_svg();
        let expected = String::from("<path d=\"M0 0L0 60L60 60L60 0L0 0M10 10L40 1L40 40L10.5 40L10 10\"/>");
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
        let wkt_out = ml.to_svg();
        let expected =
            String::from("<polyline points=\"1,1 4,1 4,4 1.5,4\"/>\n<polyline points=\"11,21 34,21 24,54 31.5,34\"/>");
        assert_eq!(wkt_out, expected);
    }

    #[test]
    fn can_format_empty_multi_line_string() {
        let ml = MultiLineString(vec![] as Vec<LineString<f64>>);
        let wkt_out = ml.to_svg();
        let expected = String::from("");
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
        let wkt_out = line.to_svg();
        let expected = String::from("<polyline points=\"1,1 4,1 4,4 1.5,4 1,1\"/>");
        assert_eq!(wkt_out, expected);
    }

    #[test]
    fn can_format_empty_line_string() {
        let line = LineString::from(vec![] as Vec<Coordinate<f64>>);
        let wkt_out = line.to_svg();
        let expected = String::from("");
        assert_eq!(wkt_out, expected);
    }

    //TODO: add tests for Line, Triangle, and Rect
}
