#[derive(Debug, PartialEq)]
pub struct Vertex {
    x: f32,
    y: f32,
    z: f32,
    w: Option<f32>,
}

#[derive(Debug, PartialEq)]
pub struct TextureVertex {
    u: f32,
    v: Option<f32>,
    w: Option<f32>,
}

#[derive(Debug, PartialEq)]
pub struct VertexNormal {
    i: f32,
    j: f32,
    k: f32,
}

#[derive(Debug, PartialEq)]
pub struct Face {
    vertices: Vec<i32>,
    texture_vertices: Option<Vec<i32>>,
    vertex_normals: Option<Vec<i32>>,
}

#[derive(Debug, PartialEq)]
pub struct ObjData {
    vertices: Vec<Vertex>,
    texture_vertices: Vec<TextureVertex>,
    vertex_normals: Vec<VertexNormal>,
    faces: Vec<Face>,
}

pub fn parse(data: &str) -> Result<ObjData, &'static str> {
    let mut vertices = Vec::new();
    let mut texture_vertices = Vec::new();
    let mut vertex_normals = Vec::new();
    let mut faces = Vec::new();

    for line in data.lines() {
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("v") => {
                let x = parts
                    .next()
                    .and_then(|x| x.parse::<f32>().ok())
                    .ok_or("Error parsing x coordinate")?;
                let y = parts
                    .next()
                    .and_then(|y| y.parse::<f32>().ok())
                    .ok_or("Error parsing y coordinate")?;
                let z = parts
                    .next()
                    .and_then(|z| z.parse::<f32>().ok())
                    .ok_or("Error parsing z coordinate")?;
                let w = parts.next().and_then(|w| w.parse::<f32>().ok());
                vertices.push(Vertex { x, y, z, w });
            }
            Some("vt") => {
                let u = parts
                    .next()
                    .and_then(|u| u.parse::<f32>().ok())
                    .ok_or("Error parsing u coordinate")?;
                let v = parts.next().and_then(|v| v.parse::<f32>().ok());
                let w = parts.next().and_then(|w| w.parse::<f32>().ok());
                texture_vertices.push(TextureVertex { u, v, w });
            }
            Some("vn") => {
                let i = parts
                    .next()
                    .and_then(|i| i.parse::<f32>().ok())
                    .ok_or("Error parsing i component")?;
                let j = parts
                    .next()
                    .and_then(|j| j.parse::<f32>().ok())
                    .ok_or("Error parsing j component")?;
                let k = parts
                    .next()
                    .and_then(|k| k.parse::<f32>().ok())
                    .ok_or("Error parsing k component")?;
                vertex_normals.push(VertexNormal { i, j, k });
            }
            Some("f") => {
                let face_parts: Vec<Vec<i32>> = parts
                    .map(|part| {
                        part.split("/")
                            .filter_map(|s| s.parse::<i32>().ok())
                            .collect()
                    })
                    .collect();
                let vertices = face_parts.iter().map(|v| v[0]).collect();
                let texture_vertices = if face_parts[0].len() > 1 {
                    Some(face_parts.iter().map(|v| v[1]).collect())
                } else {
                    None
                };
                let vertex_normals = if face_parts[0].len() > 2 {
                    Some(face_parts.iter().map(|v| v[2]).collect())
                } else {
                    None
                };
                faces.push(Face {
                    vertices,
                    texture_vertices,
                    vertex_normals,
                });
            }
            _ => {} // ignore other data
        }
    }

    Ok(ObjData {
        vertices,
        texture_vertices,
        vertex_normals,
        faces,
    })
}
