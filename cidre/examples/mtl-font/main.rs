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

    pub fn end(&mut self) -> Glyph {
        let g = self.glyph.as_mut().unwrap();
        g.bounds = self.builder.build();
        *g
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
    uint vid [[ vertex_id ]],
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
    Varyings in [[stage_in]]
) {
    if (in.uv.x * in.uv.x - in.uv.y > 0.0) {
        discard_fragment();
    }
    return float4(0.0);
}

[[early_fragment_tests]]
fragment float4 fill_fragment(
    Varyings in [[stage_in]],
    constant float4 &color [[buffer(0)]]
) {
    return color;
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

fn bounds_rect_vertices(bounds: cg::Rect) -> [f32; 24] {
    let x0 = bounds.origin.x as f32;
    let y0 = bounds.origin.y as f32;
    let x1 = (bounds.origin.x + bounds.size.width) as f32;
    let y1 = (bounds.origin.y + bounds.size.height) as f32;

    [
        x0, y0, 0.0, 0.0, // tri 1
        x1, y0, 0.0, 0.0, x0, y1, 0.0, 0.0, x1, y0, 0.0, 0.0, // tri 2
        x1, y1, 0.0, 0.0, x0, y1, 0.0, 0.0,
    ]
}

fn main() {
    let mut verticies = Vec::<f32>::new();
    let mut nverticies = Vec::<usize>::new();
    let mut byte_offsets = Vec::<usize>::new();
    let mut glyph_bounds = Vec::<cg::Rect>::new();
    let font = ct::Font::with_name_size(cf::str!(c"Verdana"), 28.0);
    let utf16 = "1234567890-=~!@#$%^&*()_qwertyuiop[]QWERTYUIOP{}|\\sasdfghjkl;`'ASDFGHJKL:\"zxcvbnm,./ZXCVBNM<>?".encode_utf16().collect::<Vec<u16>>();
    let mut glyphs = vec![cg::Glyph::new(0); utf16.len()];
    font.glyphs_for_characters(&utf16, &mut glyphs).unwrap();
    let mut advances = vec![cg::Size::zero(); glyphs.len()];
    font.advances_for_glyphs(ct::FontOrientation::Horizontal, &glyphs, &mut advances);
    let mut compiler = GlyphCompiler::default();
    let ascent = font.ascent() as f64;
    let descent = font.descent() as f64;
    let leading = font.leading() as f64;
    let cell_h = ascent + descent + leading;
    let mut cell_w = 0.0f64;
    for advance in &advances {
        cell_w = cell_w.max(advance.width as f64);
    }
    if cell_w == 0.0 {
        cell_w = 1.0;
    }
    let cols = (glyphs.len() as f64).sqrt().ceil() as usize;
    let rows = (glyphs.len() + cols - 1) / cols;
    let grid_w = cell_w * cols as f64;
    let grid_h = cell_h * rows as f64;
    let fit_scale = 0.9 * 2.0 / grid_w.max(grid_h);
    let scale = cg::AffineTransform::new_scale(fit_scale, -fit_scale);
    let origin_x = -grid_w / 2.0;
    let origin_y = -grid_h / 2.0 + ascent;
    for (idx, g) in glyphs.iter().enumerate() {
        let col = (idx % cols) as f64;
        let row = (idx / cols) as f64;
        let pen_x = origin_x + col * cell_w;
        let pen_y = origin_y + row * cell_h;
        let transform = scale.translate(pen_x, pen_y);
        if let Some(path) = font.path_for_glyph(*g, Some(&transform)) {
            compiler.begin(Glyph::default());
            path.apply(&mut compiler, apply);
            let compiled = compiler.end();
            glyph_bounds.push(compiled.bounds);
            byte_offsets.push(verticies.len() * std::mem::size_of::<f32>());
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

    let stencil_fragment_fn_name = ns::String::with_str("glyph_fragment");
    let stencil_fragment_fn = lib.new_fn(&stencil_fragment_fn_name).unwrap();

    let fill_fragment_fn_name = ns::String::with_str("fill_fragment");
    let fill_fragment_fn = lib.new_fn(&fill_fragment_fn_name).unwrap();

    let mut stencil_desc =
        mtl::RenderPipelineDesc::new().with_fns(&vertex_fn, &stencil_fragment_fn);
    stencil_desc.set_stencil_attach_pixel_format(mtl::PixelFormat::Stencil8);
    let mut stencil_ca = stencil_desc.color_attaches().get(0);
    stencil_ca.set_pixel_format(mtl::PixelFormat::Rgba8UNorm);
    stencil_ca.set_write_mask(mtl::ColorWriteMask::None);

    let mut fill_desc = mtl::RenderPipelineDesc::new().with_fns(&vertex_fn, &fill_fragment_fn);
    fill_desc.set_stencil_attach_pixel_format(mtl::PixelFormat::Stencil8);
    let mut fill_ca = fill_desc.color_attaches().get(0);
    fill_ca.set_pixel_format(mtl::PixelFormat::Rgba8UNorm);
    fill_ca.set_blending_enabled(true);
    fill_ca.set_rgb_blend_op(mtl::BlendOp::Add);
    fill_ca.set_alpha_blend_op(mtl::BlendOp::Add);
    fill_ca.set_src_rgb_blend_factor(mtl::BlendFactor::One);
    fill_ca.set_src_alpha_blend_factor(mtl::BlendFactor::One);
    fill_ca.set_dst_rgb_blend_factor(mtl::BlendFactor::One);
    fill_ca.set_dst_alpha_blend_factor(mtl::BlendFactor::One);

    let stencil_ps = device.new_render_ps(&stencil_desc).unwrap();
    let fill_ps = device.new_render_ps(&fill_desc).unwrap();

    let render_texture_desc =
        mtl::TextureDesc::new_2d(mtl::PixelFormat::Rgba8UNorm, 1920, 1080, false);

    let rgba_texture = device.new_texture(&render_texture_desc).unwrap();

    let stencil_texture_desc =
        mtl::TextureDesc::new_2d(mtl::PixelFormat::Stencil8, 1920, 1080, false);
    let stencil_texture = device.new_texture(&stencil_texture_desc).unwrap();

    let render_pass_desc = mtl::RenderPassDesc::new();
    let mut ca = render_pass_desc.color_attaches().get(0);
    ca.set_clear_color(mtl::ClearColor::clear());
    ca.set_load_action(mtl::LoadAction::Clear);
    ca.set_store_action(mtl::StoreAction::Store);
    ca.set_texture(Some(&rgba_texture));

    let mut sa = render_pass_desc.stencil_attach();
    sa.set_clear_stencil(0);
    sa.set_load_action(mtl::LoadAction::Clear);
    sa.set_store_action(mtl::StoreAction::Store);
    sa.set_texture(Some(&stencil_texture));

    let stencil_desc = mtl::DepthStencilDesc::new();
    let mut front_face_stencil = stencil_desc.front_face_stencil();
    front_face_stencil.set_compare_fn(mtl::CompareFn::Always);
    front_face_stencil.set_failure_op(mtl::StencilOp::Keep);
    front_face_stencil.set_depth_failure_op(mtl::StencilOp::Keep);
    front_face_stencil.set_depth_stencil_op(mtl::StencilOp::Invert);
    front_face_stencil.set_read_mask(0);
    front_face_stencil.set_write_mask(0xff);

    let mut back_face_stencil = stencil_desc.back_face_stencil();
    back_face_stencil.set_compare_fn(mtl::CompareFn::Always);
    back_face_stencil.set_failure_op(mtl::StencilOp::Keep);
    back_face_stencil.set_depth_failure_op(mtl::StencilOp::Keep);
    back_face_stencil.set_depth_stencil_op(mtl::StencilOp::Invert);
    back_face_stencil.set_read_mask(0);
    back_face_stencil.set_write_mask(0xff);

    let stencil_state = device.new_depth_stencil_state(&stencil_desc).unwrap();

    let fill_stencil_desc = mtl::DepthStencilDesc::new();
    let mut fill_front = fill_stencil_desc.front_face_stencil();
    fill_front.set_compare_fn(mtl::CompareFn::NotEqual);
    fill_front.set_failure_op(mtl::StencilOp::Keep);
    fill_front.set_depth_failure_op(mtl::StencilOp::Keep);
    fill_front.set_depth_stencil_op(mtl::StencilOp::Zero);
    fill_front.set_read_mask(0xff);
    fill_front.set_write_mask(0xff);

    let mut fill_back = fill_stencil_desc.back_face_stencil();
    fill_back.set_compare_fn(mtl::CompareFn::NotEqual);
    fill_back.set_failure_op(mtl::StencilOp::Keep);
    fill_back.set_depth_failure_op(mtl::StencilOp::Keep);
    fill_back.set_depth_stencil_op(mtl::StencilOp::Zero);
    fill_back.set_read_mask(0xff);
    fill_back.set_write_mask(0xff);

    let fill_stencil_state = device.new_depth_stencil_state(&fill_stencil_desc).unwrap();

    let cmd_queue = device.new_cmd_queue().unwrap();
    let mut cmd_buf = cmd_queue.new_cmd_buf_unretained_refs().unwrap();

    cmd_buf.render(&render_pass_desc, |enc| {
        enc.set_render_ps(&stencil_ps);
        enc.set_depth_stencil_state(Some(&stencil_state));
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
        let color = simd::f32x4::with_rgba(1.0, 0.0, 0.0, 1.0);

        for (idx, bounds) in glyph_bounds.iter().enumerate() {
            enc.set_render_ps(&stencil_ps);
            enc.set_depth_stencil_state(Some(&stencil_state));
            enc.set_vertex_buf_at(Some(&buf), byte_offsets[idx], 0);
            enc.draw_primitives(mtl::Primitive::Triangle, 0, nverticies[idx]);

            let rect_vertices = bounds_rect_vertices(*bounds);
            enc.set_render_ps(&fill_ps);
            enc.set_depth_stencil_state(Some(&fill_stencil_state));
            enc.set_stencil_reference_value(0);
            enc.copy_slice_to_vertex_at(&rect_vertices, 0);
            enc.copy_to_fragment_at(&color, 0);
            enc.draw_primitives(mtl::Primitive::Triangle, 0, 6);
        }
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
