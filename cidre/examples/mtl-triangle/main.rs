use cidre::{cg, ci, mtl, ns, objc::ar_pool, simd};

#[repr(C)]
struct Vertex {
    position: simd::packed::f32x2,
    color: simd::packed::f32x4,
}

impl Vertex {
    #[inline]
    pub fn with(xy: (f32, f32), rgb: (f32, f32, f32)) -> Self {
        Self {
            position: simd::packed::f32x2::with_xy(xy.0, xy.1),
            color: simd::packed::f32x4::with_xyzw(rgb.0, rgb.1, rgb.2, 1.0),
        }
    }
}

static LIB_SRC: &str = r###"

using namespace metal;

float circle(float2 st, float radius) {
    float2 dist = st;
	return 1.0 - smoothstep(radius-(radius*0.01),
                         radius+(radius*0.01),
                         dot(dist,dist)*4.0);
}
            
typedef struct {
    packed_float2 position;
    packed_float4 color;
} Vertex;

typedef struct {
    float4 position [[position]] [[center_no_perspective]];
    float4 color [[center_no_perspective]];
} Varyings;

vertex Varyings passthrough(
    unsigned short vid [[ vertex_id ]],
    constant Vertex *verticies [[buffer(0)]]
) {
    Varyings out;
    constant Vertex &v = verticies[vid];
    out.position = float4(float2(v.position), 0.0, 1.0);
    out.color = out.position;// v.color;

    return out;
}

fragment float4 pass_color(
    Varyings in [[stage_in]]
) {
    float4 color = float4(circle(in.color.xy, 0.9), 0, 0, 1);
    return color;
}

"###;

fn main() {
    ar_pool(|| {
        let device = mtl::Device::default().unwrap();

        let source = ns::String::with_str(LIB_SRC);
        let lib = device.new_lib_with_src(&source, None).unwrap();

        let vertex_fn_name = ns::String::with_str("passthrough");
        let vertex_fn = lib.new_fn(&vertex_fn_name).unwrap();

        let fragment_fn_name = ns::String::with_str("pass_color");
        let fragment_fn = lib.new_fn(&fragment_fn_name).unwrap();

        let mut desc = mtl::RenderPipelineDesc::new().with_fns(&vertex_fn, &fragment_fn);

        desc.set_raster_sample_count(4);
        desc.color_attaches_mut()[0].set_pixel_format(mtl::PixelFormat::Rgba8UNorm);

        let render_ps = device.new_render_ps(&desc).unwrap();

        let render_texture_desc = mtl::TextureDesc::new_2d_with_pixel_format(
            mtl::PixelFormat::Rgba8UNorm,
            1920,
            1080,
            false,
        );

        let rgba_texture = device.new_texture(&render_texture_desc).unwrap();

        let mut render_pass_desc = mtl::RenderPassDesc::new();
        let ca = &mut render_pass_desc.color_attaches_mut()[0];
        ca.set_clear_color(mtl::ClearColor::red());
        ca.set_load_action(mtl::LoadAction::Clear);
        ca.set_store_action(mtl::StoreAction::Store);
        ca.set_texture(Some(&rgba_texture));

        let triangle = [
            Vertex::with((-1.0, 1.0), (1.0, 0.0, 0.0)),
            Vertex::with((0.0, -1.0), (0.0, 1.0, 0.0)),
            Vertex::with((1.0, 1.0), (0.0, 0.0, 1.0)),
        ];

        let vertex_buffer = device
            .new_buf_with_slice(&triangle, Default::default())
            .unwrap();

        let cmd_queue = device.new_cmd_queue().unwrap();
        let mut cmd_buf = cmd_queue.new_cmd_buf_unretained_refs().unwrap();

        cmd_buf.render(&render_pass_desc, |enc| {
            enc.set_render_ps(&render_ps);
            enc.set_vp_rect(0, 0, 1920, 1080);
            enc.set_vertex_buf_at(Some(&vertex_buffer), 0, 0);
            enc.draw_primitives(mtl::PrimitiveType::Triangle, 0, triangle.len());
        });

        cmd_buf.commit();

        let image = ci::Image::with_mtl_texture(&rgba_texture, None).unwrap();

        let context = ci::Context::new();

        let color_space = cg::ColorSpace::device_rgb().unwrap();
        let url = ns::Url::with_str("file:///tmp/image.png").unwrap();
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
    });
}
