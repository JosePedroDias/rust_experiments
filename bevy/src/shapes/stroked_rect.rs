use bevy::{
    math::Vec2,
    render::mesh::{Indices, Mesh},
    render::pipeline::PrimitiveTopology,
};

pub fn build_stroked_rect(dims: Vec2, outer_border: f32, inner_border: f32) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let w2 = dims.x * 0.5 - inner_border;
    let h2 = dims.y * 0.5 - inner_border;
    let w2_ = dims.x * 0.5 + outer_border;
    let h2_ = dims.y * 0.5 + outer_border;

    let pos_2_uv = |p: &[f32; 3]| [(p[0] + w2_) / (w2_ * 2.), 1. - (p[1] + h2_) / (h2_ * 2.)];

    let normal = [0., 0., 1.];

    let n_vertices = 8; // 4 inner verts + 4 outer verts
    let n_indices = 24; // 8 triangles -> 24 indices

    let tr = [w2, h2, 0.];
    let tl = [-w2, h2, 0.];
    let bl = [-w2, -h2, 0.];
    let br = [w2, -h2, 0.];
    let tr_ = [w2_, h2_, 0.];
    let tl_ = [-w2_, h2_, 0.];
    let bl_ = [-w2_, -h2_, 0.];
    let br_ = [w2_, -h2_, 0.];

    // allocate stuff
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(n_vertices);
    let mut indices: Vec<u32> = Vec::with_capacity(n_indices);

    // vertices...
    positions.push(tr); // 0
    normals.push(normal);
    uvs.push(pos_2_uv(&tr));

    positions.push(tl); // 1
    normals.push(normal);
    uvs.push(pos_2_uv(&tl));

    positions.push(bl); // 2
    normals.push(normal);
    uvs.push(pos_2_uv(&bl));

    positions.push(br); // 3
    normals.push(normal);
    uvs.push(pos_2_uv(&br));

    positions.push(tr_); // 4
    normals.push(normal);
    uvs.push(pos_2_uv(&tr_));

    positions.push(tl_); // 5
    normals.push(normal);
    uvs.push(pos_2_uv(&tl_));

    positions.push(bl_); // 6
    normals.push(normal);
    uvs.push(pos_2_uv(&bl_));

    positions.push(br_); // 7
    normals.push(normal);
    uvs.push(pos_2_uv(&br_));

    // assign vertices
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    // indices...
    indices.push(0); // top
    indices.push(4);
    indices.push(1);

    indices.push(1);
    indices.push(4);
    indices.push(5);

    indices.push(5); // left
    indices.push(6);
    indices.push(1);

    indices.push(6);
    indices.push(2);
    indices.push(1);

    indices.push(6); // bottom
    indices.push(7);
    indices.push(3);

    indices.push(3);
    indices.push(2);
    indices.push(6);

    indices.push(7); // right
    indices.push(4);
    indices.push(3);

    indices.push(3);
    indices.push(4);
    indices.push(0);

    // assign indices
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}
