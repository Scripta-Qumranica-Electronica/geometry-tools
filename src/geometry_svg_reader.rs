use crate::geometry_normalize::Normalized;
use geo_types::{Coordinate, Geometry, Line, LineString, Polygon, Rect};
use std::convert::From;
use svgtypes::{PathParser, PathSegment, PointsParser};
use wasm_bindgen::JsValue;

use xml::reader::{EventReader, XmlEvent};

pub fn to_geometry(svg: &str) -> Result<Geometry<f64>, JsValue> {
    let parser = EventReader::new(svg.as_bytes());
    for e in parser {
        if let Ok(XmlEvent::StartElement {
            name, attributes, ..
        }) = e
        {
            // An SVG path element
            if name.local_name == "path" {
                for attr in attributes {
                    if attr.name.local_name == "d" {
                        let res = svg_d_path_to_geometry(&attr.value);
                        return Ok(res.unwrap().into());
                    }
                }
            }
            // An SVG polygon
            else if name.local_name == "polygon" {
                for attr in attributes {
                    if attr.name.local_name == "points" {
                        let res = svg_polygon_to_geometry(&attr.value);
                        return Ok(res.unwrap().into());
                    }
                }
            }
            // An SVG polyline
            else if name.local_name == "polyline" {
                for attr in attributes {
                    if attr.name.local_name == "points" {
                        let res = svg_polyline_to_geometry(&attr.value);
                        return Ok(res.unwrap().into());
                    }
                }
            }
            // An SVG rect
            else if name.local_name == "rect" {
                let mut x: Option<f64> = None;
                let mut y: Option<f64> = None;
                let mut width: Option<f64> = None;
                let mut height: Option<f64> = None;

                for attr in attributes {
                    if attr.name.local_name == "x" {
                        let x_val = attr.value.parse::<f64>();
                        if x_val.is_err() {
                            return Err(JsValue::from_str("no valid x value found in SVG rect"));
                        }
                        x = Some(x_val.unwrap());
                    } else if attr.name.local_name == "y" {
                        let y_val = attr.value.parse::<f64>();
                        if y_val.is_err() {
                            return Err(JsValue::from_str("no valid y value found in SVG rect"));
                        }
                        y = Some(y_val.unwrap());
                    } else if attr.name.local_name == "width" {
                        let width_val = attr.value.parse::<f64>();
                        if width_val.is_err() {
                            return Err(JsValue::from_str(
                                "no valid width value found in SVG rect",
                            ));
                        }
                        width = Some(width_val.unwrap());
                    } else if attr.name.local_name == "height" {
                        let height_val = attr.value.parse::<f64>();
                        if height_val.is_err() {
                            return Err(JsValue::from_str(
                                "no valid height value found in SVG rect",
                            ));
                        }
                        height = Some(height_val.unwrap());
                    }
                }

                if x.is_none() {
                    return Err(JsValue::from_str("no x value found in SVG rect"));
                }
                if y.is_none() {
                    return Err(JsValue::from_str("no y value found in SVG rect"));
                }
                if width.is_none() {
                    return Err(JsValue::from_str("no width value found in SVG rect"));
                }
                if height.is_none() {
                    return Err(JsValue::from_str("no height value found in SVG rect"));
                }

                return Ok(svg_rect_to_geometry(
                    x.unwrap(),
                    y.unwrap(),
                    width.unwrap(),
                    height.unwrap(),
                )
                .unwrap()
                .into());
            }
            // An SVG line
            else if name.local_name == "line" {
                let mut start_x: Option<f64> = None;
                let mut start_y: Option<f64> = None;
                let mut end_x: Option<f64> = None;
                let mut end_y: Option<f64> = None;

                for attr in attributes {
                    if attr.name.local_name == "x1" {
                        let start_x_val = attr.value.parse::<f64>();
                        if start_x_val.is_err() {
                            return Err(JsValue::from_str("no valid x1 value found in SVG rect"));
                        }
                        start_x = Some(start_x_val.unwrap());
                    } else if attr.name.local_name == "y1" {
                        let start_y_val = attr.value.parse::<f64>();
                        if start_y_val.is_err() {
                            return Err(JsValue::from_str("no valid y1 value found in SVG rect"));
                        }
                        start_y = Some(start_y_val.unwrap());
                    } else if attr.name.local_name == "x2" {
                        let end_x_val = attr.value.parse::<f64>();
                        if end_x_val.is_err() {
                            return Err(JsValue::from_str("no valid x2 value found in SVG rect"));
                        }
                        end_x = Some(end_x_val.unwrap());
                    } else if attr.name.local_name == "y2" {
                        let end_y_val = attr.value.parse::<f64>();
                        if end_y_val.is_err() {
                            return Err(JsValue::from_str("no valid y2 value found in SVG rect"));
                        }
                        end_y = Some(end_y_val.unwrap());
                    }
                }

                if start_x.is_none() {
                    return Err(JsValue::from_str("no x1 value found in SVG rect"));
                }
                if start_y.is_none() {
                    return Err(JsValue::from_str("no y1 value found in SVG rect"));
                }
                if end_x.is_none() {
                    return Err(JsValue::from_str("no x2 value found in SVG rect"));
                }
                if end_y.is_none() {
                    return Err(JsValue::from_str("no y2 value found in SVG rect"));
                }

                return Ok(svg_line_to_geometry(
                    &start_x.unwrap(),
                    &start_y.unwrap(),
                    &end_x.unwrap(),
                    &end_y.unwrap(),
                )
                .into());
            }
        }
    }

    Err(JsValue::from_str("no svg shape found"))
}

fn svg_polygon_to_geometry(point_string: &str) -> Result<Polygon<f64>, JsValue> {
    let points = PointsParser::from(point_string);
    let polygon = Polygon::new(
        LineString(
            points
                .map(|(x, y)| Coordinate { x, y })
                .collect::<Vec<Coordinate<f64>>>(),
        ),
        vec![],
    );

    if polygon.exterior().num_coords() == 0 {
        return Err(JsValue::from_str("no points found in linestring"));
    }
    Ok(polygon.normalized())
}

fn svg_polyline_to_geometry(point_string: &str) -> Result<LineString<f64>, JsValue> {
    let points = PointsParser::from(point_string);
    let linestring = LineString(
        points
            .map(|(x, y)| Coordinate { x, y })
            .collect::<Vec<Coordinate<f64>>>(),
    );

    if linestring.num_coords() == 0 {
        return Err(JsValue::from_str("no points found in linestring"));
    }
    Ok(linestring)
}

fn svg_rect_to_geometry(x: f64, y: f64, width: f64, height: f64) -> Result<Polygon<f64>, JsValue> {
    let max_x = x + width;
    let max_y = y + height;
    if x > max_x {
        return Err(JsValue::from_str("x + width is less than x"));
    }
    if y > max_y {
        return Err(JsValue::from_str("y + height is less than y"));
    }

    // geo_types::Rect is not part of the enum Geometry, so we cast it to Polygon upon return
    Ok(Polygon::from(Rect::new(
        Coordinate::<f64> { x, y },
        Coordinate::<f64> { x: max_x, y: max_y },
    ))
    .normalized())
}

fn svg_line_to_geometry(start_x: &f64, start_y: &f64, end_x: &f64, end_y: &f64) -> Line<f64> {
    Line::new(
        Coordinate::<f64> {
            x: *start_x,
            y: *start_y,
        },
        Coordinate::<f64> {
            x: *end_x,
            y: *end_y,
        },
    )
}

fn svg_d_path_to_geometry(svg: &str) -> Result<Polygon<f64>, JsValue> {
    // Store the Vec<Coordinate> for each ring, the first one will be the outer ring
    // TODO: find out if it is possible for any other ring in the SVG to be the outer, or only the first one
    let mut rings = vec![] as Vec<Vec<Coordinate<f64>>>;
    let mut ring_count = 0;
    let mut first_ring = true;
    let zero_coord = Coordinate { x: 0_f64, y: 0_f64 }; // Default values to be added to relative coords
    let mut last_point: Option<Coordinate<f64>> = None; // Store last point for relative coordinates
    let p = PathParser::from(svg);
    // TODO: implement curves as well
    for token in p {
        let t = token.unwrap();
        match t {
            PathSegment::MoveTo { .. } => {
                rings.push(vec![] as Vec<Coordinate<f64>>);
                if !first_ring {
                    ring_count += 1;
                } else {
                    first_ring = false;
                }
                let coord = Coordinate {
                    x: if t.is_relative() {
                        t.x().unwrap() + last_point.unwrap_or(zero_coord).x
                    } else {
                        t.x().unwrap()
                    },
                    y: if t.is_relative() {
                        t.y().unwrap() + last_point.unwrap_or(zero_coord).y
                    } else {
                        t.y().unwrap()
                    },
                };
                last_point = Some(coord);
                rings[ring_count].push(coord);
            }
            PathSegment::LineTo { .. } => {
                let coord = Coordinate {
                    x: if t.is_relative() {
                        t.x().unwrap() + last_point.unwrap_or(zero_coord).x
                    } else {
                        t.x().unwrap()
                    },
                    y: if t.is_relative() {
                        t.y().unwrap() + last_point.unwrap_or(zero_coord).y
                    } else {
                        t.y().unwrap()
                    },
                };
                last_point = Some(coord);
                rings[ring_count].push(coord);
            }
            PathSegment::HorizontalLineTo { .. } => {
                let coord = Coordinate {
                    x: if t.is_relative() {
                        t.x().unwrap() + last_point.unwrap_or(zero_coord).x
                    } else {
                        t.x().unwrap()
                    },
                    y: last_point.unwrap_or(zero_coord).y,
                };
                last_point = Some(coord);
                rings[ring_count].push(coord);
            }
            PathSegment::VerticalLineTo { .. } => {
                let coord = Coordinate {
                    x: last_point.unwrap_or(zero_coord).x,
                    y: if t.is_relative() {
                        t.y().unwrap() + last_point.unwrap_or(zero_coord).y
                    } else {
                        t.y().unwrap()
                    },
                };
                last_point = Some(coord);
                rings[ring_count].push(coord);
            }
            PathSegment::ClosePath { .. } => {
                let coord = Coordinate {
                    x: rings[ring_count][0].x,
                    y: rings[ring_count][0].y,
                };
                last_point = Some(coord);
                rings[ring_count].push(coord);
            }
            _ => last_point = None,
        }
    }
    if rings.is_empty() {
        return Err(JsValue::from_str("no coordinates found in the path"));
    }

    let mut rings_iter = rings.iter();
    let outer_ring: LineString<f64> = LineString::from(rings_iter.next().unwrap().clone());
    let inner_rings = rings_iter.map(|x| LineString::from(x.clone())).collect();
    Ok(Polygon::new(outer_ring, inner_rings).normalized())
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
        let svg_string = String::from("M0 0l0 60L60 60L60 0L0 0M10 10L40 1L40 40L10.5 40L10 10");
        let parsed_svg = svg_d_path_to_geometry(&svg_string);
        assert_eq!(parsed_svg.is_ok(), true);
        assert_eq!(parsed_svg.unwrap(), poly);
    }

    #[test]
    fn can_convert_svg_path_test() {
        let poly: Geometry<f64> = polygon!(
        exterior: [
            (x: 0.0_f64, y: 0.0),
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
        )
        .into();
        let svg_string =
            String::from(r#"<path d="M0 0L0 60L60 60L60 0L0 0M10 10L40 1L40 40L10.5 40L10 10"/>"#);
        let parsed_svg = to_geometry(&svg_string);
        assert_eq!(parsed_svg.is_ok(), true);
        assert_eq!(parsed_svg.unwrap(), poly);
    }

    #[test]
    fn can_convert_svg_polygon_test() {
        let poly: Geometry<f64> = polygon!(
        exterior: [
            (x: 0.0_f64, y: 0.0),
            (x: 0.0, y: 60.0),
            (x: 60.0, y: 60.0),
            (x: 60.0, y: 0.0),
            (x: 0.0, y: 0.0),],
        interiors:[]
        )
        .into();
        let svg_string = String::from(r#"<polygon points="0, 0 60, 0 60, 60 0, 60 0, 0"/>"#);
        let parsed_svg = to_geometry(&svg_string);
        assert_eq!(parsed_svg.is_ok(), true);
        assert_eq!(parsed_svg.unwrap(), poly);
    }

    #[test]
    fn can_convert_svg_polyline_test() {
        let line: Geometry<f64> = line_string![
            (x: 0.0_f64, y: 0.0),
            (x: 0.0, y: 60.0),
            (x: 60.0, y: 60.0),
            (x: 60.0, y: 0.0),]
        .into();
        let svg_string = String::from(r#"<polyline points="0, 0 0, 60 60, 60 60, 0"/>"#);
        let parsed_svg = to_geometry(&svg_string);
        assert_eq!(parsed_svg.is_ok(), true);
        assert_eq!(parsed_svg.unwrap(), line);
    }

    #[test]
    fn can_convert_svg_rect_test() {
        let poly: Geometry<f64> = polygon!(
        exterior: [
            (x: 0.0_f64, y: 0.0),
            (x: 0.0, y: 60.0),
            (x: 60.0, y: 60.0),
            (x: 60.0, y: 0.0),
            (x: 0.0, y: 0.0),],
        interiors:[]
        )
        .into();
        let svg_string = String::from(r#"<rect x="0" y="0" width="60" height="60"/>"#);
        let parsed_svg = to_geometry(&svg_string);
        assert_eq!(parsed_svg.is_ok(), true);
        assert_eq!(parsed_svg.unwrap(), poly);
    }
}
