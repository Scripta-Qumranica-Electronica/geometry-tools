extern crate geo;

use self::geo::algorithm::winding_order::Winding;
use geo_types::{Coordinate, Geometry, GeometryCollection, LineString, MultiPolygon, Polygon};

pub trait Normalized<T: num_traits::Float> {
    fn normalized(&self) -> Self;
}

/** Geometry Collections */

impl<T: num_traits::Float> Normalized<T> for GeometryCollection<T> {
    fn normalized(&self) -> Self {
        GeometryCollection(
            self.0
                .iter()
                .map(|p| match p {
                    Geometry::Polygon { .. } => {
                        Geometry::Polygon(p.clone().into_polygon().unwrap().normalized())
                    }
                    Geometry::MultiPolygon { .. } => {
                        Geometry::MultiPolygon(p.clone().into_multi_polygon().unwrap().normalized())
                    }
                    _ => p.clone(),
                })
                .collect::<Vec<Geometry<T>>>(),
        )
    }
}

/** Polygons */

impl<T: num_traits::Float> Normalized<T> for MultiPolygon<T> {
    fn normalized(&self) -> Self {
        MultiPolygon::from(
            self.0
                .iter()
                .map(|x| x.normalized())
                .collect::<Vec<Polygon<T>>>(),
        )
    }
}

impl<T: num_traits::Float> Normalized<T> for Polygon<T> {
    fn normalized(&self) -> Self {
        normalized_polygon(self)
    }
}

/// Return a new polygon where the exterior ring points are clockwise and interior ring points are
/// counter-clockwise
///
fn normalized_polygon<T: num_traits::Float>(poly: &Polygon<T>) -> Polygon<T> {
    Polygon::new(
        LineString::from(
            poly.exterior()
                .points_cw()
                .map(|x| x.0)
                .collect::<Vec<Coordinate<T>>>(),
        ),
        poly.interiors()
            .iter()
            .map(|ring| {
                LineString::from(
                    ring.clone()
                        .points_ccw()
                        .map(|x| x.0)
                        .collect::<Vec<Coordinate<T>>>(),
                )
            })
            .collect(),
    )
}

/** Tests */

#[cfg(test)]
mod tests {
    use super::*;
    use geo_types::{line_string, point, polygon};

    #[test]
    fn does_not_change_good_polygon() {
        let (good, _) = get_bad_outer_poly();
        let norm = good.normalized();
        assert_eq!(norm, good);
    }

    #[test]
    fn can_normalize_bad_outer_polygon() {
        let (good, bad) = get_bad_outer_poly();
        let norm = bad.normalized();
        assert_eq!(norm, good);
    }

    #[test]
    fn can_normalize_good_outer_bad_inner_polygon() {
        let (good, bad) = get_good_outer_bad_inner_poly();
        let norm = bad.normalized();
        assert_eq!(norm, good);
    }

    #[test]
    fn can_normalize_bad_outer_bad_inner_polygon() {
        let (good, bad) = get_bad_outer_bad_inner_poly();
        let norm = bad.normalized();
        assert_eq!(norm, good);
    }

    #[test]
    fn can_normalize_bad_outer_good_inner_polygon() {
        let (good, bad) = get_bad_outer_good_inner_poly();
        let norm = bad.normalized();
        assert_eq!(norm, good);
    }

    #[test]
    fn can_process_multi_polygon() {
        let (good, bad) = get_bad_outer_good_inner_poly();
        let mp = MultiPolygon(vec![good.clone(), bad]);
        let norm = mp.normalized();
        for poly in norm {
            assert_eq!(good, poly);
        }
    }

    fn get_bad_outer_poly() -> (Polygon<f64>, Polygon<f64>) {
        let bad = polygon![
        (x: 1.0, y: 1.0),
        (x: 4.0, y: 1.0),
        (x: 4.0, y: 4.0),
        (x: 1.0, y: 4.0),
        (x: 1.0, y: 1.0),
        ];
        let good = polygon![
        (x: 1.0, y: 1.0),
        (x: 1.0, y: 4.0),
        (x: 4.0, y: 4.0),
        (x: 4.0, y: 1.0),
        (x: 1.0, y: 1.0),
        ];
        (good, bad)
    }

    fn get_good_outer_bad_inner_poly() -> (Polygon<f64>, Polygon<f64>) {
        let bad = polygon!(
            exterior: [
                (x: 0., y: 0.),
                (x: 0., y: 50.),
                (x: 50., y: 50.),
                (x: 50., y: 0.),
            ],
            interiors: [
                [
                    (x: 10., y: 10.),
                    (x: 10., y: 20.),
                    (x: 20., y: 20.),
                    (x: 20., y: 10.),
                ],
            ],
        );
        let good = polygon!(
            exterior: [
                (x: 0., y: 0.),
                (x: 0., y: 50.),
                (x: 50., y: 50.),
                (x: 50., y: 0.),
            ],
            interiors: [
                [
                    (x: 10., y: 10.),
                    (x: 20., y: 10.),
                    (x: 20., y: 20.),
                    (x: 10., y: 20.),
                ],
            ],
        );
        (good, bad)
    }

    fn get_bad_outer_bad_inner_poly() -> (Polygon<f64>, Polygon<f64>) {
        let bad = polygon!(
            exterior: [
                (x: 0., y: 0.),
                (x: 50., y: 0.),
                (x: 50., y: 50.),
                (x: 0., y: 50.),
            ],
            interiors: [
                [
                    (x: 10., y: 10.),
                    (x: 10., y: 20.),
                    (x: 20., y: 20.),
                    (x: 20., y: 10.),
                ],
            ],
        );
        let good = polygon!(
            exterior: [
                (x: 0., y: 0.),
                (x: 0., y: 50.),
                (x: 50., y: 50.),
                (x: 50., y: 0.),
            ],
            interiors: [
                [
                    (x: 10., y: 10.),
                    (x: 20., y: 10.),
                    (x: 20., y: 20.),
                    (x: 10., y: 20.),
                ],
            ],
        );
        (good, bad)
    }

    fn get_bad_outer_good_inner_poly() -> (Polygon<f64>, Polygon<f64>) {
        let bad = polygon!(
            exterior: [
                (x: 0., y: 0.),
                (x: 50., y: 0.),
                (x: 50., y: 50.),
                (x: 0., y: 50.),
            ],
            interiors: [
                [
                    (x: 10., y: 10.),
                    (x: 20., y: 10.),
                    (x: 20., y: 20.),
                    (x: 10., y: 20.),
                ],
            ],
        );
        let good = polygon!(
            exterior: [
                (x: 0., y: 0.),
                (x: 0., y: 50.),
                (x: 50., y: 50.),
                (x: 50., y: 0.),
            ],
            interiors: [
                [
                    (x: 10., y: 10.),
                    (x: 20., y: 10.),
                    (x: 20., y: 20.),
                    (x: 10., y: 20.),
                ],
            ],
        );
        (good, bad)
    }
}
