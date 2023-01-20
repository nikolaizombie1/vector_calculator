pub mod two_dimension {
    #[derive(PartialEq, Debug)]
    pub struct Point {
        pub x: f32,
        pub y: f32,
    }
    #[derive(PartialEq, Clone, Debug)]
    pub struct ComponentVector {
        pub i: f32,
        pub j: f32,
    }
    #[derive(PartialEq, Debug)]
    pub struct DistanceVector {
        pub p1: Point,
        pub p2: Point,
        pub distance_vector: ComponentVector,
        pub magnitude: f32,
        pub unit_vector: ComponentVector,
    }
    impl DistanceVector {
        pub fn new(p1: Point, p2: Point) -> DistanceVector {
            let distance_vector = ComponentVector {
                i: p2.y - p1.y,
                j: p2.x - p1.x,
            };
            let magnitude = f32::sqrt(distance_vector.i.powi(2) + distance_vector.j.powi(2));
            let unit_vector = ComponentVector {
                i: distance_vector.i / magnitude,
                j: distance_vector.j / magnitude,
            };
            DistanceVector {
                p1,
                p2,
                distance_vector,
                magnitude,
                unit_vector,
            }
        }
    }
    #[derive(PartialEq, Debug)]
    pub struct MagnitudeVector {
        pub magnitude: f32,
        pub theta: f32,
    }
    #[derive(PartialEq, Debug)]
    pub struct Vector {
        pub component_vector: ComponentVector,
        pub magnitude_vector: MagnitudeVector,
        pub distance_vector: DistanceVector,
    }
    impl Vector {
        pub fn new_with_components(component_vector: ComponentVector) -> Vector {
            let magnitude = f32::sqrt(component_vector.i.powi(2) + component_vector.j.powi(2));
            let theta = f32::atan(component_vector.j / component_vector.i);
            let p1 = Point { x: 0.0, y: 0.0 };
            let p2 = Point {
                x: component_vector.i,
                y: component_vector.j,
            };
            let distance_vector = ComponentVector {
                i: p2.y - p1.y,
                j: p2.x - p1.x,
            };
            let distance_vector_magnitude =
                f32::sqrt(distance_vector.i.powi(2) + distance_vector.j.powi(2));
            let unit_vector = ComponentVector {
                i: distance_vector.i / distance_vector_magnitude,
                j: distance_vector.j / distance_vector_magnitude,
            };
            Vector {
                component_vector,
                magnitude_vector: MagnitudeVector { magnitude, theta },
                distance_vector: DistanceVector {
                    p1,
                    p2,
                    distance_vector,
                    magnitude: distance_vector_magnitude,
                    unit_vector,
                },
            }
        }
        pub fn new_with_magnitude(magnitude_vector: MagnitudeVector) -> Vector {
            let i = magnitude_vector.magnitude * f32::cos(magnitude_vector.theta);
            let j = magnitude_vector.magnitude * f32::sin(magnitude_vector.theta);
            let p1 = Point { x: 0.0, y: 0.0 };
            let p2 = Point { x: i, y: j };
            let distance_vector = ComponentVector {
                i: p2.y - p1.y,
                j: p2.x - p1.x,
            };
            let distance_vector_magnitude =
                f32::sqrt(distance_vector.i.powi(2) + distance_vector.j.powi(2));
            let unit_vector = ComponentVector {
                i: distance_vector.i / distance_vector_magnitude,
                j: distance_vector.j / distance_vector_magnitude,
            };
            Vector {
                component_vector: ComponentVector { i, j },
                magnitude_vector,
                distance_vector: DistanceVector {
                    p1,
                    p2,
                    distance_vector,
                    magnitude: distance_vector_magnitude,
                    unit_vector,
                },
            }
        }
        pub fn new_with_distance(distance_vector: DistanceVector) -> Vector {
            let theta =
                f32::atan(distance_vector.distance_vector.j / distance_vector.distance_vector.i);
            Vector {
                component_vector: distance_vector.distance_vector.clone(),
                magnitude_vector: MagnitudeVector {
                    magnitude: distance_vector.magnitude,
                    theta,
                },
                distance_vector,
            }
        }
    }

    use std::ops::Add;
    impl Add for Vector {
        type Output = Vector;
        fn add(self, _rhs: Vector) -> Vector {
            let i = self.component_vector.i + _rhs.component_vector.i;
            let j = self.component_vector.j + _rhs.component_vector.j;
            let magnitude = f32::sqrt(i.powi(2) + j.powi(2));
            let theta = f32::atan(j / i);
            let p1 = Point {
                x: self.distance_vector.p1.x + _rhs.distance_vector.p1.x,
                y: self.distance_vector.p1.y + _rhs.distance_vector.p1.y,
            };
            let p2 = Point {
                x: self.distance_vector.p2.x + _rhs.distance_vector.p2.x,
                y: self.distance_vector.p2.y + _rhs.distance_vector.p2.y,
            };
            Vector {
                component_vector: ComponentVector { i, j },
                magnitude_vector: MagnitudeVector { magnitude, theta },
                distance_vector: DistanceVector::new(p1, p2),
            }
        }
    }
}
