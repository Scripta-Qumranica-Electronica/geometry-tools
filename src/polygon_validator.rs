extern crate geo;

use geo_types::{
    Coordinate, Geometry, GeometryCollection, Line, LineString, MultiLineString, MultiPoint,
    MultiPolygon, Point, Polygon,
};
use std::fmt;
use std::iter::Chain;
use std::hash::{Hash, Hasher};
use num_traits;
use linked_hash_map::LinkedHashMap;
use geo::algorithm::intersects::Intersects;
use byteorder::{ByteOrder, NativeEndian};

/// Violations of the OCG rules for polygon validity
/// This also includes usage of NaN or Infinite floating point values
pub struct ValidationErrors<T>
where T: num_traits::Float
{
    /// Whether the polygon is valid or not
    pub valid: bool,
    /// Whether the polygon has less than three points
    pub has_less_than_three_points: bool,
    /// NaN/Infinite floating point values
    pub unsupported_floating_point_values: Vec<T>,
    /// Rings of the polygon that have not been closed
    pub open_rings: Vec<LineString<T>>,
    /// Holes in the polygon that intersect the outer ring of another inner ring
    /// (they may, however, share points)
    pub ring_intersects_other_ring: Vec<Coordinate<T>>,
    /// Coordinates where self intersection occurs
    pub self_intersections: Vec<Coordinate<T>>,
    /// Points that touch a line
    pub point_touching_line: Vec<Coordinate<T>>,
    /// Points repeated in a single ring
    pub repeated_points: Vec<Coordinate<T>>,
}

pub trait Validate<T>
    where T: num_traits::Float
{
    fn validate(&self) -> bool;
    fn validate_detailed(&self) -> ValidationErrors<T>;
}

/** Geometries */

impl<T> Validate<T> for Polygon<T>
    where T: num_traits::Float
{
    fn validate(&self) -> bool {
        validate_polygon(self, true).valid
    }

    fn validate_detailed(&self) -> ValidationErrors<T> {
        validate_polygon(self, false)
    }
}

/// Check a polygon for validity.
/// This function is rather long, because it tries to be as quick as possible
/// and to waste as few resources as possible. Setting the second parameter `quick`
/// to true will cause the function to bail out at the very first error (without)
/// providing any detailed information about the error.
fn validate_polygon<T>(poly: &Polygon<T>, quick: bool) -> ValidationErrors<T>
    where   T: num_traits::Float,
{
    let mut validation_errors = ValidationErrors::<T> {
        valid: true,
        has_less_than_three_points: false,
        unsupported_floating_point_values: vec![] as Vec<T>,
        open_rings: vec![] as Vec<LineString<T>>,
        ring_intersects_other_ring: vec![] as Vec<Coordinate<T>>,
        self_intersections: vec![] as Vec<Coordinate<T>>,
        point_touching_line: vec![] as Vec<Coordinate<T>>,
        repeated_points: vec![] as Vec<Coordinate<T>>,
    };

    let mut poly_lines = vec![] as Vec<Line<T>>;
    let mut rings = vec![poly.exterior()];
    rings.extend(poly.interiors());
    let mut ring_start_idx = 0; // This is used together with poly_lines to determine if intersection is with self
    for ring in rings.into_iter() {
        // Check for poly with less than 3 points
        let ring_points_count = ring.0.len();
        if  ring_points_count < 3 {
            validation_errors.valid = false;
            if quick { return validation_errors; }
            validation_errors.has_less_than_three_points = true;
        }

        // Check for open ring
        // Note: this check is pointless with the current geo crate, since it automatically closes
        // any open rings. It is computationally cheap though, so keep it in case of future design
        // changes.
        if !ring.0[0].x.eq(&ring.0[ring_points_count - 1].x)
            && !ring.0[0].y.eq(&ring.0[ring_points_count - 1].y) {
                validation_errors.valid = false;
                if quick { return validation_errors; }
                validation_errors.open_rings.push(ring.clone());
        }

        // Check for unsupported floating point value
        let mut prev_point = ring.0[0];
        if !prev_point.x.is_finite() {
            validation_errors.valid = false;
            if quick { return validation_errors; }
            validation_errors.unsupported_floating_point_values.push(prev_point.x);
        }
        if !prev_point.y.is_finite() {
            validation_errors.valid = false;
            if quick { return validation_errors; }
            validation_errors.unsupported_floating_point_values.push(prev_point.y);
        }

        // let mut ring_points_map = LinkedHashMap::<[u8;16], Coordinate<T>>::new();
        let mut ring_points_map = LinkedHashMap::<CompCoord<T>, Coordinate<T>>::new();
        for i in 1..(ring_points_count) {
            let point = ring.0[i];

            // Check for unsupported floating point value
            if !point.x.is_finite() {
                validation_errors.valid = false;
                if quick { return validation_errors; }
                validation_errors.unsupported_floating_point_values.push(point.x);
            }
            if !point.y.is_finite() {
                validation_errors.valid = false;
                if quick { return validation_errors; }
                validation_errors.unsupported_floating_point_values.push(point.y);
            }

            // Check for repeated points (don't check the last point, since that should == first point)
            let pp_comp = CompCoord { 0: Coordinate { x: prev_point.x, y: prev_point.y } };
            if ring_points_map.contains_key(&pp_comp) {
                validation_errors.valid = false;
                if quick { return validation_errors; }
                validation_errors.repeated_points.push(prev_point);
            }
            // Check for intersections
            let current_line = Line::<T>::new(prev_point, point);
            for (line_idx, line) in poly_lines.iter().enumerate() {
                if !line.end.eq(&current_line.start) && !line.start.eq(&current_line.end) {
                    if line.intersects(&current_line) {
                        validation_errors.valid = false;
                        if quick { return validation_errors; }
                        if line_idx > ring_start_idx {
                            validation_errors.self_intersections.push(line.intersection_point(&current_line));
                        } else {
                            validation_errors.ring_intersects_other_ring.push(line.intersection_point(&current_line));
                        }
                    }

                    // Check if any points intersect any lines
                    let start_point: Point<T> = current_line.start.into();
                    if line.intersects(&start_point) {
                        validation_errors.valid = false;
                        if quick { return validation_errors; }
                        validation_errors.point_touching_line.push(current_line.start);
                    }
                }
            }
            poly_lines.push(current_line);
            prev_point = point;
            ring_points_map.insert(pp_comp, point);
        }
        ring_start_idx = poly_lines.len();
    }

    validation_errors
}

struct CompCoord<T: num_traits::Float>(Coordinate<T>);

impl<T: num_traits::Float> PartialEq for CompCoord<T> {
    fn eq(&self, other: &CompCoord<T>) -> bool {
        // Note this function has no idea about the history of the float coordinates
        // only the current state.  This is a strict byte-equality check and does not
        // try to account in any way for the deviation of a float from its expected
        // value due to imprecision caused by floating point operations.
        transform_coord_to_array_of_u8(self) ==
            transform_coord_to_array_of_u8(other)
    }
}

impl<T: num_traits::Float> Eq for CompCoord<T> {}

impl<T: num_traits::Float> Hash for CompCoord<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        transform_coord_to_array_of_u8(self).hash(state);
    }
}
/// Transform a coordinate into a 128byte array by concatenating the
/// byte representation of its position on the 2 axes (as f64)
/// 
fn transform_coord_to_array_of_u8<T>(coord: &CompCoord<T>) -> [u8;16]
where T: num_traits::Float
{
    let mut buf1 = [0; 8];
    NativeEndian::write_f64(&mut buf1, T::to_f64(&coord.0.x).unwrap());
    let mut buf2 = [0; 8];
    NativeEndian::write_f64(&mut buf2, T::to_f64(&coord.0.y).unwrap());
    
    [ buf1[0], buf1[1], buf1[2], buf1[3], buf1[4], buf1[5], buf1[6], buf1[7], 
        buf2[0], buf2[1], buf2[2], buf2[3], buf2[4], buf2[5], buf2[6], buf2[7] ]
}

/// Transform a 128byte array into a geometry coordinate
/// 
fn transform_array_of_u8_to_coord(byte_arr: &[u8;16]) -> CompCoord<f64>
{
    let x = NativeEndian::read_f64(&[byte_arr[0], byte_arr[1], byte_arr[2], byte_arr[3], byte_arr[4], 
        byte_arr[5], byte_arr[6], byte_arr[7]]);
    let y = NativeEndian::read_f64(&[byte_arr[8], byte_arr[9], byte_arr[10], byte_arr[11], byte_arr[12], 
        byte_arr[13], byte_arr[14], byte_arr[15]]);

    CompCoord { 0: Coordinate { x, y } }
}

/// Returns the coordinate at which two geometries intersect
pub trait IntersectionPoint<T>
where T: num_traits::Float,
{
    fn intersection_point(&self, line: &Line<T>) -> Coordinate<T>;
}

impl<T> IntersectionPoint<T> for Line<T>
    where
        T: num_traits::Float,
{
    // See https://www.geeksforgeeks.org/program-for-point-of-intersection-of-two-lines/
    fn intersection_point(&self, line: &Line<T>) -> Coordinate<T> {
        // Line AB represented as a1x + b1y = c1
        let a1 = self.end.y - self.start.y;
        let b1 = self.start.x - self.end.x;
        let c1 = a1 * (self.start.x) + b1 * (self.start.y);

        // Line CD represented as a2x + b2y = c2
        let a2 = line.end.y - line.start.y;
        let b2 = line.start.x - line.end.x;
        let c2 = a2 * (line.start.x) + b2 * (line.start.y);

        let determinant = a1 * b2 - a2 * b1;
        return if determinant.is_normal() // Will this be problematic in cases where determinant is subnormal?
        {
            let x = (b2 * c1 - b1 * c2) / determinant;
            let y = (a1 * c2 - a2 * c1) / determinant;
            Coordinate { x, y }
        } else {
            // Parallel lines never intersect (hence infinity)
            Coordinate {
                x: T::infinity(),
                y: T::infinity()
            }
        }
    }
}

/** Tests */

#[cfg(test)]
mod tests {
    use super::*;
    use geo_types::{line_string, point, polygon};
    use std::fmt::Debug;

    #[test]
    fn can_validate_polygon() {
        let poly = polygon![
            (x: 1.0, y: 1.0),
            (x: 4.000000007, y: 1.0),
            (x: 4.0, y: 4.0),
            (x: 1.0, y: 4.0),
            (x: 1.0, y: 1.0),
        ];

        let valid = validate_polygon(&poly, false);
        assert_eq!(valid.valid, true);
    }

    #[test]
    fn can_validate_complex_polygon() {
        let poly = polygon!(
            exterior: [
                (x: 0., y: 0.),
                (x: 0., y: 20.),
                (x: 20., y: 20.),
                (x: 20., y: 0.),
            ],
            interiors: [
                [
                    (x: 10., y: 10.),
                    (x: 15., y: 10.),
                    (x: 15., y: 15.),
                    (x: 10., y: 15.),
                ],
            ],
        );

        let valid = validate_polygon(&poly, true);
        assert_eq!(valid.valid, true);
    }

    #[test]
    fn can_find_multiple_errors_in_complex_polygon() {
        let poly = polygon!(
            exterior: [
                (x: 0., y: 0.),
                (x: 0., y: 200.),
                (x: 200., y: 0.),
                (x: 200., y: 200.),
            ],
            interiors: [
                [
                    (x: 10., y: 20.),
                    (x: 50., y: 20.),
                    (x: 20., y: 50.),
                    (x: 50., y: 50.),
                ],
            ],
        );

        let valid = validate_polygon(&poly, false);
        assert_eq!(valid.valid, false);
        assert_eq!(valid.ring_intersects_other_ring.len(), 4);
        assert_eq!(valid.self_intersections.len(), 2);
        assert_eq!(valid.point_touching_line.len(), 1);

        assert_eq!(valid.ring_intersects_other_ring[0].x, 20_f64);
        assert_eq!(valid.ring_intersects_other_ring[0].y, 20_f64);
        assert_eq!(valid.ring_intersects_other_ring[1].x, 35_f64);
        assert_eq!(valid.ring_intersects_other_ring[1].y, 35_f64);
        assert_eq!(valid.ring_intersects_other_ring[2].x, 50_f64);
        assert_eq!(valid.ring_intersects_other_ring[2].y, 50_f64);
        assert_eq!(valid.ring_intersects_other_ring[3].x, 50_f64);
        assert_eq!(valid.ring_intersects_other_ring[3].y, 50_f64);

        assert_eq!(valid.self_intersections[0].x, 100_f64);
        assert_eq!(valid.self_intersections[0].y, 100_f64);
        assert_eq!(valid.self_intersections[1].x, 32.857142857142854_f64);
        assert_eq!(valid.self_intersections[1].y, 37.142857142857146_f64);

        assert_eq!(valid.point_touching_line[0].x, 50_f64);
        assert_eq!(valid.point_touching_line[0].y, 50_f64);

    }

    #[test]
    fn can_recognize_self_intersecting_polygon() {
        let poly = polygon![
            (x: 1.0, y: 1.0),
            (x: 4.0, y: 1.0),
            (x: 1.0, y: 4.0),
            (x: 4.0, y: 4.0),
            (x: 1.0, y: 1.0),
        ];

        let valid = validate_polygon(&poly, false);
        assert_eq!(valid.valid, false);
        assert_eq!(valid.self_intersections.len(), 1);
        assert_eq!(valid.self_intersections[0].x, 2.5);
        assert_eq!(valid.self_intersections[0].y, 2.5);
    }
}
