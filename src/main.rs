use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Vertex {
    x: f32,
    y: f32,
    z: f32,
    w: Option<f32>,
}

struct TextureCoordinate {
    u: f32,
    v: Option<f32>,
    w: Option<f32>,
}

struct Normal {
    i: f32,
    j: f32,
    k: f32,
}

struct FaceVertex {
    v: i32,         
    vt: Option<i32>,
    vn: Option<i32>,
}

fn parse_vertex(parts: &[&str]) -> Vertex {
    let x = parts[1].parse().unwrap();
    let y = parts[2].parse().unwrap();
    let z = parts[3].parse().unwrap();
    let w = if parts.len() > 4 {
        Some(parts[4].parse().unwrap())
    } else {
        None
    };
    Vertex { x, y, z, w }
}

fn parse_texture_coordinate(parts: &[&str]) -> TextureCoordinate {
    let u = parts[1].parse().unwrap();
    let v = if parts.len() > 2 {
        Some(parts[2].parse().unwrap())
    } else {
        None
    };
    let w = if parts.len() > 3 {
        Some(parts[3].parse().unwrap())
    } else {
        None
    };
    TextureCoordinate { u, v, w }
}

fn parse_normal(parts: &[&str]) -> Normal {
    let i = parts[1].parse().unwrap();
    let j = parts[2].parse().unwrap();
    let k = parts[3].parse().unwrap();
    Normal { i, j, k }
}

fn parse_face_vertex(part: &str) -> FaceVertex {
    let indices: Vec<&str> = part.split('/').collect();
    let v = indices[0].parse().unwrap();
    let vt = if indices.len() > 1 && !indices[1].is_empty() {
        Some(indices[1].parse().unwrap())
    } else {
        None
    };
    let vn = if indices.len() > 2 {
        Some(indices[2].parse().unwrap())
    } else {
        None
    };
    FaceVertex { v, vt, vn }
}

fn main() -> io::Result<()> {
    let file = File::open("model.obj")?;
    let reader = io::BufReader::new(file);

    let mut vertices: Vec<Vertex> = Vec::new();
    let mut texture_coordinates: Vec<TextureCoordinate> = Vec::new();
    let mut normals: Vec<Normal> = Vec::new();
    let mut faces: Vec<Vec<FaceVertex>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts[0] {
            "v" => vertices.push(parse_vertex(&parts)),
            "vt" => texture_coordinates.push(parse_texture_coordinate(&parts)),
            "vn" => normals.push(parse_normal(&parts)),
            "f" => {
                let face = parts[1..]
                    .iter()
                    .map(|&p| parse_face_vertex(p))
                    .collect();
                faces.push(face);
            }
            _ => {} // Ignore lines that don't start with v, vt, vn, or f
        }
    }

    Ok(())
}

