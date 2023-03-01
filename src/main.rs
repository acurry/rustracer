use vec3D::Vec3D;

use crate::scene::{Shape, Object};

pub mod scene;

fn main() {
    println!("Hello, world!");

    let s: Object = Object { shape: Shape::Sphere, radius: 10.0, shininess: 100, reflection: 100.0, center: Vec3D { x: 10.0, y: 10.0, z: 30.0 } };

    println!("{:?}", s);
}
