mod parser;
mod transformations;

use transformations::{Matrix4x4, Vector3};

fn main() {
    let vertex = Vector3::new(1.0, 2.0, 3.0);

    let translate = Matrix4x4::translation(5.0, 0.0, 0.0);
    let rotate = Matrix4x4::rotation_x(90.0);
    
    let combined = rotate.multiply(&translate);

    let transformed_vertex = combined.transform_vector(&vertex);

    println!("Original Vertex: {:?}", vertex);
    println!("Transformed Vertex: {:?}", transformed_vertex);
}

