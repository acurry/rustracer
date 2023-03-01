use vec3D::Vec3D;

#[derive(Debug)]
pub enum Shape {
    Sphere
}

#[derive(Debug)]
pub struct Object {
    pub(crate) shape: Shape,
    pub(crate) radius: f32,
    pub(crate) shininess: i32,
    pub(crate) reflection: f32,
    pub(crate) center: Vec3D
}