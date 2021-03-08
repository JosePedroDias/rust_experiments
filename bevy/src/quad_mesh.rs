use bevy::{
    math::Vec2,
    render::mesh::{Indices, Mesh},
    render::pipeline::PrimitiveTopology,
};

/*
+Y
   0     1


   2     3
             +X
*/
pub fn build_quad(dims: Vec2) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let n_vertices = 4;
    let n_indices = 6;

    let w2 = dims[0] / 2.0;
    let h2 = dims[1] / 2.0;

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(n_vertices);

    // #0 tl
    positions.push([-w2, h2, 0.]);
    normals.push([0., 0., 1.]);
    uvs.push([0., 0.]);

    // #1 tr
    positions.push([w2, h2, 0.]);
    normals.push([0., 0., 1.]);
    uvs.push([1., 0.]);

    // #2 bl
    positions.push([-w2, -h2, 0.]);
    normals.push([0., 0., 1.]);
    uvs.push([0., 1.]);

    // #3 br
    positions.push([w2, -h2, 0.]);
    normals.push([0., 0., 1.]);
    uvs.push([1., 1.]);

    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    let mut indices: Vec<u32> = Vec::with_capacity(n_indices);
    indices.push(1);
    indices.push(0);
    indices.push(2);

    indices.push(1);
    indices.push(2);
    indices.push(3);
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}

pub fn build_quad_uvs(dims: Vec2, uvs: (f32, f32, f32, f32)) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let n_vertices = 4;
    let n_indices = 6;

    let w2 = dims[0] / 2.0;
    let h2 = dims[1] / 2.0;

    let (u0, u1, v0, v1) = uvs;

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(n_vertices);

    // #0 tl
    positions.push([-w2, h2, 0.]);
    normals.push([0., 0., 1.]);
    uvs.push([u0, v0]);

    // #1 tr
    positions.push([w2, h2, 0.]);
    normals.push([0., 0., 1.]);
    uvs.push([u1, v0]);

    // #2 bl
    positions.push([-w2, -h2, 0.]);
    normals.push([0., 0., 1.]);
    uvs.push([u0, v1]);

    // #3 br
    positions.push([w2, -h2, 0.]);
    normals.push([0., 0., 1.]);
    uvs.push([u1, v1]);

    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    let mut indices: Vec<u32> = Vec::with_capacity(n_indices);
    indices.push(1);
    indices.push(0);
    indices.push(2);

    indices.push(1);
    indices.push(2);
    indices.push(3);
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}
