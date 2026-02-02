// port of https://github.com/evanw/theta/blob/master/src/core/font.sk
// https://nilcoalescing.com/blog/RenderingQuadraticBezierCurvesWithMetal/

use cidre::{cf, cg, ci, ct, mtl, ns, simd};

enum TriangleKind {
    Solid,
    QuadraticCurve,
}

#[derive(Default, Debug, Clone, Copy)]
struct Glyph {
    // code_point: UniChar,
    bounds: cg::Rect,
}

struct RectBuilder {
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32,
}

impl RectBuilder {
    pub fn include(&mut self, x: f32, y: f32) {
        self.min_x = f32::min(self.min_x, x);
        self.min_y = f32::min(self.min_y, y);
        self.max_x = f32::max(self.max_x, x);
        self.max_y = f32::max(self.max_y, y);
    }

    pub fn new() -> Self {
        Self {
            min_x: f32::INFINITY,
            min_y: f32::INFINITY,
            max_x: f32::NEG_INFINITY,
            max_y: f32::NEG_INFINITY,
        }
    }

    pub fn build(&self) -> cg::Rect {
        cg::Rect::new(
            self.min_x as f64,
            self.min_y as f64,
            (self.max_x - self.min_x) as f64,
            (self.max_y - self.min_y) as f64,
        )
    }
}

impl Default for RectBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default)]
struct GlyphCompiler {
    vertices: Vec<f32>,
    first_x: f32,
    first_y: f32,
    current_x: f32,
    current_y: f32,
    countour_count: usize,
    glyph: Option<Glyph>,
    builder: RectBuilder,
}

impl GlyphCompiler {
    pub fn begin(&mut self, glyph: Glyph) {
        self.glyph = Some(glyph);
        self.builder = RectBuilder::new();
        self.vertices.clear();
    }

    pub fn move_to<F: Into<f32>>(&mut self, x: F, y: F) {
        let x = x.into();
        let y = y.into();
        self.first_x = x;
        self.first_y = y;
        self.current_x = x;
        self.current_y = y;
        self.countour_count = 0;
    }

    pub fn line_to<F: Into<f32>>(&mut self, x: F, y: F) {
        let x = x.into();
        let y = y.into();
        self.countour_count += 1;
        if self.countour_count >= 2 {
            self.push_triangle(
                self.first_x,
                self.first_y,
                self.current_x,
                self.current_y,
                x,
                y,
                TriangleKind::Solid,
            );
        }
        self.current_x = x;
        self.current_y = y;
    }

    pub fn curve_to<F: Into<f32>>(&mut self, cx: F, cy: F, x: F, y: F) {
        let x = x.into();
        let y = y.into();
        self.countour_count += 1;
        if self.countour_count >= 2 {
            self.push_triangle(
                self.first_x,
                self.first_y,
                self.current_x,
                self.current_y,
                x,
                y,
                TriangleKind::Solid,
            );
        }

        self.push_triangle(
            self.current_x,
            self.current_y,
            cx.into(),
            cy.into(),
            x,
            y,
            TriangleKind::QuadraticCurve,
        );

        self.current_x = x;
        self.current_y = y;
    }

    pub fn close(&mut self) {
        self.current_x = self.first_x;
        self.current_y = self.first_y;
        self.countour_count = 0;
    }

    pub fn end(&mut self) {
        let g = self.glyph.as_mut().unwrap();
        g.bounds = self.builder.build();
    }

    fn push_triangle<F: Into<f32>>(
        &mut self,
        ax: F,
        ay: F,
        bx: F,
        by: F,
        cx: F,
        cy: F,
        kind: TriangleKind,
    ) {
        match kind {
            TriangleKind::Solid => {
                self.push_vertex(ax.into(), ay.into(), 0.0, 1.0);
                self.push_vertex(bx.into(), by.into(), 0.0, 1.0);
                self.push_vertex(cx.into(), cy.into(), 0.0, 1.0);
            }
            TriangleKind::QuadraticCurve => {
                self.push_vertex(ax.into(), ay.into(), 0.0, 0.0);
                self.push_vertex(bx.into(), by.into(), 0.5, 0.0);
                self.push_vertex(cx.into(), cy.into(), 1.0, 1.0);
            }
        }
    }

    fn push_vertex<F: Into<f32>>(&mut self, x: F, y: F, s: F, t: F) {
        let x = x.into();
        let y = y.into();
        self.builder.include(x, y);
        self.vertices.extend_from_slice(&[x, y, s.into(), t.into()]);
    }
}

extern "C" fn apply(compiler: *mut GlyphCompiler, element: *mut cg::PathElement) {
    let compiler = unsafe { &mut *compiler };
    let element = unsafe { &*element };
    use cg::PathElementType as pet;
    match element.type_ {
        pet::MoveToPoint => {
            let point = element.points()[0];
            compiler.move_to(point.x as f32, point.y as f32);
        }
        pet::AddLineToPoint => {
            let point = element.points()[0];
            compiler.line_to(point.x as f32, point.y as f32);
        }
        pet::AddQuadCurveToPoint => {
            let points = element.points();
            let c = points[0];
            let p = points[1];
            compiler.curve_to(c.x as f32, c.y as f32, p.x as f32, p.y as f32);
        }
        pet::AddCurveToPoint => panic!("unexpected curve to"),
        pet::CloseSubpath => compiler.close(),
    }
}

static LIB_SRC: &str = r###"

using namespace metal;
           
typedef struct {
    float4 position;
} Vertex;

typedef struct {
    float4 position [[position]];
    float2 uv [[sample_no_perspective]];
} Varyings;

vertex Varyings glyph_vertex(
    unsigned short vid [[ vertex_id ]],
    constant Vertex *verticies [[buffer(0)]],
    constant float3x3 &matrix [[buffer(1)]]
) {
    Varyings out;
    constant Vertex &v = verticies[vid];
    out.position = float4(matrix * float3(float2(v.position.xy), 1.0), 1.0);
    out.uv = float2(v.position.zw);

    return out;
}

[[early_fragment_tests]]
fragment float4 glyph_fragment(
    Varyings in [[stage_in]],
    bool is_front_face [[front_facing]],
    constant float4 &color [[buffer(0)]]
) {
    if (in.uv.x * in.uv.x - in.uv.y > 0.0) {
        discard_fragment();
    }
    // Upper 4 bits: front faces
    // Lower 4 bits: back faces
    return color * (is_front_face ? 16.0 / 255.0 : 1.0 / 255.0);
}

"###;

/// 6x subpixel AA pattern
///
///   R = (f(x - 2/3, y) + f(x - 1/3, y) + f(x, y)) / 3
///   G = (f(x - 1/3, y) + f(x, y) + f(x + 1/3, y)) / 3
///   B = (f(x, y) + f(x + 1/3, y) + f(x + 2/3, y)) / 3
///
/// The shader would require three texture lookups if the texture format
/// stored data for offsets -1/3, 0, and +1/3 since the shader also needs
/// data for offsets -2/3 and +2/3. To avoid this, the texture format stores
/// data for offsets 0, +1/3, and +2/3 instead. That way the shader can get
/// data for offsets -2/3 and -1/3 with only one additional texture lookup.
///
// const JITTER_PATTERN: [(f32, f32); 6] = [
//     (-1.0 / 12.0, -5.0 / 12.0),
//     (1.0 / 12.0, 1.0 / 12.0),
//     (3.0 / 12.0, -1.0 / 12.0),
//     (5.0 / 12.0, 5.0 / 12.0),
//     (7.0 / 12.0, -3.0 / 12.0),
//     (9.0 / 12.0, 3.0 / 12.0),
// ];

fn main() {
    let mut verticies = Vec::<f32>::new();
    let mut nverticies = Vec::<usize>::new();
    let mut byte_offsets = Vec::<usize>::new();
    let font = ct::Font::with_name_size(cf::str!(c"Verdana"), 28.0);
    let utf16 = "1234567890-=~!@#$%^&*()_qwertyuiop[]QWERTYUIOP{}|\\sasdfghjkl;`'ASDFGHJKL:\"zxcvbnm,./ZXCVBNM<>?".encode_utf16().collect::<Vec<u16>>();
    let mut glyphs = vec![cg::Glyph::new(0); utf16.len()];
    font.glyphs_for_characters(&utf16, &mut glyphs).unwrap();
    let scale = cg::AffineTransform::new_scale(1.0 / (1920.0 * 0.05), -1.0 / (1080.0 * 0.05));
    let mut compiler = GlyphCompiler::default();
    for g in glyphs {
        if let Some(path) = font.path_for_glyph(g, Some(&scale)) {
            let gg = Glyph::default();
            compiler.begin(gg);
            path.apply(&mut compiler, apply);
            compiler.end();
            byte_offsets.push(verticies.len() * 4 * 2);
            nverticies.push(compiler.vertices.len() / 4);
            verticies.extend_from_slice(&compiler.vertices);
        } else {
            eprintln!("no path for {:?}", g);
        }
    }

    let device = mtl::Device::sys_default().unwrap();
    let buf = device
        .new_buf_from_vec(verticies, mtl::ResOpts::default())
        .unwrap();

    let source = ns::String::with_str(LIB_SRC);
    let lib = device.new_lib_with_src_blocking(&source, None).unwrap();

    let vertex_fn_name = ns::String::with_str("glyph_vertex");
    let vertex_fn = lib.new_fn(&vertex_fn_name).unwrap();

    let fragment_fn_name = ns::String::with_str("glyph_fragment");
    let fragment_fn = lib.new_fn(&fragment_fn_name).unwrap();

    let desc = mtl::RenderPipelineDesc::new().with_fns(&vertex_fn, &fragment_fn);

    let mut ca = desc.color_attaches().get(0);
    ca.set_pixel_format(mtl::PixelFormat::Rgba8UNorm);
    ca.set_blending_enabled(true);
    ca.set_rgb_blend_op(mtl::BlendOp::Add);
    ca.set_alpha_blend_op(mtl::BlendOp::Add);
    ca.set_src_rgb_blend_factor(mtl::BlendFactor::One);
    ca.set_src_alpha_blend_factor(mtl::BlendFactor::One);
    ca.set_dst_rgb_blend_factor(mtl::BlendFactor::One);
    ca.set_dst_alpha_blend_factor(mtl::BlendFactor::One);

    let render_ps = device.new_render_ps(&desc).unwrap();

    let render_texture_desc =
        mtl::TextureDesc::new_2d(mtl::PixelFormat::Rgba8UNorm, 1920, 1080, false);

    let rgba_texture = device.new_texture(&render_texture_desc).unwrap();

    let render_pass_desc = mtl::RenderPassDesc::new();
    let mut ca = render_pass_desc.color_attaches().get(0);
    ca.set_clear_color(mtl::ClearColor::clear());
    ca.set_load_action(mtl::LoadAction::Clear);
    ca.set_store_action(mtl::StoreAction::Store);
    ca.set_texture(Some(&rgba_texture));

    let depth_desc = mtl::DepthStencilDesc::new();
    let mut front_face_stencil = depth_desc.front_face_stencil();
    front_face_stencil.set_depth_stencil_op(mtl::StencilOp::Invert);
    front_face_stencil.set_compare_fn(mtl::CompareFn::Always);
    front_face_stencil.set_read_mask(0);

    let mut back_face_stencil = depth_desc.front_face_stencil();
    back_face_stencil.set_depth_stencil_op(mtl::StencilOp::Invert);
    back_face_stencil.set_compare_fn(mtl::CompareFn::Always);
    back_face_stencil.set_read_mask(0);

    let depth_state = device.new_depth_stencil_state(&depth_desc).unwrap();

    let cmd_queue = device.new_cmd_queue().unwrap();
    let mut cmd_buf = cmd_queue.new_cmd_buf_unretained_refs().unwrap();

    cmd_buf.render(&render_pass_desc, |enc| {
        enc.set_render_ps(&render_ps);
        enc.set_depth_stencil_state(Some(&depth_state));
        enc.set_vp(mtl::Viewport {
            x: 0.0,
            y: 0.0,
            w: 1920.0,
            h: 1080.0,
            z_near: 0.0,
            z_far: 1.0,
        });
        enc.set_ffw(mtl::Winding::Ccw);
        let t = simd::f32x3x3::translate(0.0, 0.0);
        enc.copy_to_vertex_at(&t, 1);
        enc.set_vertex_buf_at(Some(&buf), 0, 0);
        let color = simd::f32x4::with_rgba(1.0, 0.0, 0.0, 1.0);
        enc.copy_to_fragment_at(&color, 0);
        enc.draw_primitives(mtl::Primitive::Triangle, 0, nverticies[0]);
        // for j in 0..JITTER_PATTERN.len() {
        //     let (tx, ty) = JITTER_PATTERN[j];
        //     let t = simd::f32x3x3::translate(tx, ty);
        //     enc.set_vertex_arg_at(&t, 1);
        //     if j % 2 == 0 {
        //         let color = simd::f32x4::with_rgba(
        //             if j == 0 { 1.0 } else { 0.0 },
        //             if j == 2 { 1.0 } else { 0.0 },
        //             if j == 4 { 1.0 } else { 0.0 },
        //             1.0,
        //         );
        //         enc.set_fragment_arg_at(&color, 0);
        //     }
        //     enc.draw_primitives(mtl::Primitive::Triangle, 0, nverticies[0]);
        // }
    });

    cmd_buf.commit();

    let image = ci::Image::with_mtl_texture(&rgba_texture, None).unwrap();

    let context = ci::Context::new();

    let color_space = cg::ColorSpace::device_rgb().unwrap();
    let url = ns::Url::with_str("file:///tmp/image@2x.png").unwrap();
    context
        .write_png_to_url(
            &image,
            &url,
            ci::Format::rgba8(),
            &color_space,
            ns::Dictionary::new().as_ref(),
        )
        .unwrap();

    println!("image is written to {:?}", url.abs_string());
}
