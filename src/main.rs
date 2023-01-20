mod two_dimension;
use std::f32::consts::PI;

use two_dimension::two_dimension::{Vector, MagnitudeVector, ComponentVector};

fn main() {
    let m1 = MagnitudeVector  {magnitude: 10.0,theta: PI/4.0};
    let v1 = Vector::new_with_magnitude(m1);
    let c1 = ComponentVector {i: 5.0,j: 5.0};
    let v2 = Vector::new_with_components(c1);
    println!("{:?}",v1+v2);
}
