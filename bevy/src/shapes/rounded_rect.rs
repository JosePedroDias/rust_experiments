use bevy::{
    math::Vec2,
    render::mesh::{Indices, Mesh},
    render::pipeline::PrimitiveTopology,
};

pub fn build_rounded_rect(dims: Vec2, _radius: f32, _verts_per_corner: usize) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let w2 = dims.x * 0.5;
    let h2 = dims.y * 0.5;
    //let W2 = w2 + radius;
    //let H2 = h2 + radius;

    let ctl = [-w2, h2, 0.];
    let ctr = [w2, h2, 0.];
    let cbl = [-w2, -h2, 0.];
    let cbr = [w2, -h2, 0.];

    let normal = [0., 0., 1.];

    let n_vertices = 4;
    let n_indices = 6;

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(n_vertices);

    // vertices...

    // rect in the center (2 tris)

    positions.push(ctr); // 0
    normals.push(normal);
    uvs.push([0., 0.]);

    positions.push(ctl); // 1
    normals.push(normal);
    uvs.push([0., 0.]);

    positions.push(cbl); // 2
    normals.push(normal);
    uvs.push([0., 0.]);

    positions.push(cbr); // 3
    normals.push(normal);
    uvs.push([0., 0.]);

    // assign vertices
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    // indices...
    let mut indices: Vec<u32> = Vec::with_capacity(n_indices);

    // rect in the center
    indices.push(0);
    indices.push(1);
    indices.push(2);

    indices.push(0);
    indices.push(2);
    indices.push(3);

    // assign indices
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}
