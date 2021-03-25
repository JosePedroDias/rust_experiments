use bevy::{
    render::mesh::{Indices, Mesh},
    render::pipeline::PrimitiveTopology,
};

const PI2: f32 = std::f32::consts::PI * 2.0;

pub fn generate_cylinder(half_height: f32, radius: f32, steps: usize) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let n_vertices = 2 + steps * 4;
    let n_triangles = (steps * 4) as u32;

    // TODO UVS

    let h2 = half_height;
    /*
    vertices:
        ctr -y
        ctr +y
        bottom circ
        top circ
        sides (2 circs)
    triangles:
        bottom
        top
        side (2 tris at a time)
     */

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(n_vertices);
    let mut indices: Vec<u32> = Vec::with_capacity((n_triangles as usize) * 3);

    positions.push([0., -h2, 0.]);
    normals.push([0., -1., 0.]);
    uvs.push([0., 0.]);

    positions.push([0., h2, 0.]);
    normals.push([0., 1., 0.]);
    uvs.push([0., 0.]);

    for nth in 0..2 {
        let y = if nth == 0 { -h2 } else { h2 };
        let ny = if nth == 0 { -1. } else { 1. };
        for i in 0..steps {
            let angle = (i as f32) / (steps as f32) * PI2;
            let c = angle.cos();
            let s = angle.sin();
            let x = radius * c;
            let z = radius * s;
            positions.push([x, y, z]);
            normals.push([0., ny, 0.]);
            uvs.push([0., 0.]);
        }
    }

    for nth in 0..2 {
        let y = if nth == 0 { -h2 } else { h2 };
        for i in 0..steps {
            let angle = (i as f32) / (steps as f32) * PI2;
            let c = angle.cos();
            let s = angle.sin();
            let x = radius * c;
            let z = radius * s;
            positions.push([x, y, z]);
            normals.push([c, 0., s]);
            uvs.push([0., 0.]);
        }
    }

    let s0 = 2 as u32;
    let s1 = 2 + steps as u32;
    let s2 = 2 + (steps * 2) as u32;
    let s3 = 2 + (steps * 3) as u32;

    // bottom and top
    for i in 0..n_triangles {
        indices.push(i % n_triangles + s0);
        indices.push((i + 1) % n_triangles + s0);
        indices.push(0);

        indices.push(i % n_triangles + s1);
        indices.push((i + 1) % n_triangles + s1);
        indices.push(1);
    }

    // sides
    for i in 0..n_triangles {
        indices.push(i % n_triangles + s3);
        indices.push((i + 1) % n_triangles + s2);
        indices.push(i % n_triangles + s2);

        indices.push(i % n_triangles + s3);
        indices.push((i + 1) % n_triangles + s3);
        indices.push((i + 1) % n_triangles + s2);
    }

    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}
