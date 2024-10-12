#![no_std]

use spirv_std::glam::{Vec3, Vec4,Vec2, Mat4};
use spirv_std::spirv;
use spirv_std::Sampler;
use spirv_std::image::Image2d;
use spirv_std::num_traits::Float;
use shared::SceneConstants;

// Render Pass 1
#[spirv(vertex)]
pub fn main_vs(
    pos: Vec3,
    tex_coords: Vec2,
    normal: Vec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] camera_view_proj: &Mat4,
    #[spirv(position)] out_pos: &mut Vec4,
    out_tex_coord: &mut Vec2,
) {
    *out_pos = *camera_view_proj * pos.extend(1.0);
    *out_tex_coord = tex_coords;
}

#[spirv(fragment)]
pub fn main_fs(
    in_tex: Vec2,
    #[spirv(descriptor_set = 1, binding = 0)] texture: &Image2d,
    #[spirv(descriptor_set = 1, binding = 1)] sampler: &Sampler,
    output: &mut Vec4,
) {    
    *output = texture.sample(*sampler, in_tex);
}


// Render Pass 2
#[spirv(vertex)]
pub fn fullscreen_quad_gen_vs(
    #[spirv(vertex_index)] vertex_index: u32,
    #[spirv(position)] out_pos: &mut Vec4,
) {
    let outUV = Vec2::new(
        ((vertex_index << 1) & 2) as f32,
        (vertex_index & 2) as f32,
    );
    *out_pos = Vec4::new(outUV.x * 2.0 - 1.0,outUV.y * 2.0 - 1.0, 0.0, 1.0);
}

#[spirv(fragment)]
pub fn quad_copy_fs(
    in_pos: Vec4,
    output: &mut Vec4,
) {    
    *output = Vec4::new(1.0,0.2,0.9,1.0);
}

fn intensity(col: Vec4) -> f32 {
    (col.x * col.x + col.y * col.y + col.z * col.z).sqrt()
}


    //let frag_coord = Vec2::new(in_frag_coord.x,in_frag_coord.y);
    //
    //let tleft = intensity(framebuf.sample(*sampler, frag_coord + Vec2::new(-1.0, 1.0)));
    //let left = intensity(framebuf.sample(*sampler, frag_coord + Vec2::new(-1.0, 0.0)));
    //let bleft = intensity(framebuf.sample(*sampler, frag_coord + Vec2::new(-1.0, -1.0)));
    //let tright = intensity(framebuf.sample(*sampler, frag_coord + Vec2::new(1.0, 1.0)));
    //let right = intensity(framebuf.sample(*sampler, frag_coord + Vec2::new(1.0, 0.0)));
    //let bright = intensity(framebuf.sample(*sampler, frag_coord + Vec2::new(1.0, -1.0)));
    //let top = intensity(framebuf.sample(*sampler, frag_coord + Vec2::new(0.0, 1.0)));
    //let bottom = intensity(framebuf.sample(*sampler, frag_coord + Vec2::new(0.0, -1.0)));

    //let g_x = tleft + 2.0 * left + bleft - tright - 2.0 * right - bright;
    //let g_y = -tleft - 2.0 * top - tright + bleft + 2.0 * bottom + bright;
    //let magnitude = (g_x * g_x + g_y * g_y).sqrt();
