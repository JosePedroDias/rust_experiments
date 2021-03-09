use bevy::{
    math::Vec2,
    render::mesh::{Indices, Mesh},
    render::pipeline::PrimitiveTopology,
};

//const PI2: f32 = std::f32::consts::PI * 2.0;
const PI_OVER_2: f32 = std::f32::consts::PI * 0.5;

pub fn build_rounded_rect(dims: Vec2, radius: f32, verts_per_corner: usize) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let w2_ = dims.x * 0.5;
    let h2_ = dims.y * 0.5;
    let w2 = w2_ - radius;
    let h2 = h2_ - radius;

    let pos_2_uv = |p: &[f32; 3]| [(p[0] + w2_) / (w2_ * 2.), 1. - (p[1] + h2_) / (h2_ * 2.)];
    let tr = [w2, h2, 0.];
    let tl = [-w2, h2, 0.];
    let bl = [-w2, -h2, 0.];
    let br = [w2, -h2, 0.];

    let normal = [0., 0., 1.];

    let n_vertices = (verts_per_corner + 2) * 4;
    let n_indices = (2 + 8 + 4 * verts_per_corner) * 3;

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(n_vertices);

    // vertices...

    // rect in the center (2 tris)
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

    // TR quarter circle
    let mut start_angle: f32 = 0.;

    for i in 0..verts_per_corner {
        let angle = (i as f32) / ((verts_per_corner - 1) as f32) * PI_OVER_2 + start_angle;
        let p = [
            tr[0] + radius * angle.cos(),
            tr[1] + radius * angle.sin(),
            tr[2],
        ];
        positions.push(p);
        normals.push(normal);
        uvs.push(pos_2_uv(&p));
    }

    // TL quarter circle
    start_angle += PI_OVER_2;
    for i in 0..verts_per_corner {
        let angle = (i as f32) / ((verts_per_corner - 1) as f32) * PI_OVER_2 + start_angle;
        let p = [
            tl[0] + radius * angle.cos(),
            tl[1] + radius * angle.sin(),
            tl[2],
        ];
        positions.push(p);
        normals.push(normal);
        uvs.push(pos_2_uv(&p));
    }

    // BL quarter circle
    start_angle += PI_OVER_2;
    for i in 0..verts_per_corner {
        let angle = (i as f32) / ((verts_per_corner - 1) as f32) * PI_OVER_2 + start_angle;
        let p = [
            bl[0] + radius * angle.cos(),
            bl[1] + radius * angle.sin(),
            bl[2],
        ];
        positions.push(p);
        normals.push(normal);
        uvs.push(pos_2_uv(&p));
    }

    // BR quarter circle
    start_angle += PI_OVER_2;
    for i in 0..verts_per_corner {
        let angle = (i as f32) / ((verts_per_corner - 1) as f32) * PI_OVER_2 + start_angle;
        let p = [
            br[0] + radius * angle.cos(),
            br[1] + radius * angle.sin(),
            br[2],
        ];
        positions.push(p);
        normals.push(normal);
        uvs.push(pos_2_uv(&p));
    }

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

    let ipc = verts_per_corner as u32;

    // TR quarter circle
    let mut start_i: u32 = 4;
    for i in 0..ipc {
        indices.push(0);
        indices.push(i % ipc + start_i);
        indices.push((i + 1) % ipc + start_i);
    }

    // TL quarter circle
    start_i += ipc;
    for i in 0..ipc {
        indices.push(1);
        indices.push(i % ipc + start_i);
        indices.push((i + 1) % ipc + start_i);
    }

    // BL quarter circle
    start_i += ipc;
    for i in 0..ipc {
        indices.push(2);
        indices.push(i % ipc + start_i);
        indices.push((i + 1) % ipc + start_i);
    }

    // BR quarter circle
    start_i += ipc;
    for i in 0..ipc {
        indices.push(3);
        indices.push(i % ipc + start_i);
        indices.push((i + 1) % ipc + start_i);
    }

    // 4 side rects
    let mut a: u32 = 0;
    let mut b = 4 + verts_per_corner as u32 - 1;
    for _ in 0..3 {
        indices.push(a);
        indices.push(b);
        indices.push(b + 1);
        indices.push(a);
        indices.push(b + 1);
        indices.push(a + 1);
        a += 1;
        b += verts_per_corner as u32;
    }
    indices.push(a);
    indices.push(b);
    indices.push(4);
    indices.push(a);
    indices.push(4);
    indices.push(0);

    // assign indices
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}
