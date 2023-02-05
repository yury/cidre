use cidre::{cg, ci, mtl, ns, objc::autoreleasepool, simd};

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
    autoreleasepool(|| {
        let device = mtl::Device::default().unwrap();

        let source = ns::String::with_str(LIB_SRC);
        let lib = device.library_with_source(&source, None).unwrap();

        let vertex_fn_name = ns::String::with_str("passthrough");
        let vertex_fn = lib.new_function_with_name(&vertex_fn_name).unwrap();

        let fragment_fn_name = ns::String::with_str("pass_color");
        let fragment_fn = lib.new_function_with_name(&fragment_fn_name).unwrap();

        let mut desc = mtl::RenderPipelineDescriptor::new();
        desc.set_raster_sample_count(4);
        desc.set_vertex_fn(&vertex_fn);
        desc.set_fragment_fn(&fragment_fn);
        desc.color_attachments_mut()[0].set_pixel_format(mtl::PixelFormat::RGBA8Unorm);

        let pipeline_state = device.render_pipeline_state_with_descriptor(&desc).unwrap();

        let render_texture_desc = mtl::TextureDescriptor::new_2d_with_pixel_format(
            mtl::PixelFormat::RGBA8Unorm,
            1920,
            1080,
            false,
        );

        let rgba_texture = device
            .texture_with_descriptor(&render_texture_desc)
            .unwrap();

        let mut render_pass_desc = mtl::RenderPassDescriptor::new();
        let ca = &mut render_pass_desc.color_attachments_mut()[0];
        ca.set_clear_color(mtl::ClearColor::red());
        ca.set_load_action(mtl::LoadAction::Clear);
        ca.set_store_action(mtl::StoreAction::Store);
        ca.set_texture(&rgba_texture);

        let triangle = [
            Vertex::with((-1.0, 1.0), (1.0, 0.0, 0.0)),
            Vertex::with((0.0, -1.0), (0.0, 1.0, 0.0)),
            Vertex::with((1.0, 1.0), (0.0, 0.0, 1.0)),
        ];

        let vertex_buffer = device
            .buffer_with_slice(&triangle, Default::default())
            .unwrap();

        let command_queue = device.command_queue().unwrap();
        let command_buffer = command_queue.command_buffer().unwrap();

        let mut encoder = command_buffer
            .render_command_encoder_with_descriptor(&render_pass_desc)
            .unwrap();

        encoder.set_render_pipeline_state(&pipeline_state);

        encoder.set_vertex_buffer(Some(&vertex_buffer), 0, 0);

        encoder.draw_primitives(mtl::PrimitiveType::Triangle, 0, triangle.len());

        encoder.end_encoding();

        command_buffer.commit();

        let image = ci::Image::with_mtl_texture(&rgba_texture, None).unwrap();

        let context = ci::Context::new();

        let color_space = cg::ColorSpace::device_rgb().unwrap();
        let url = ns::URL::with_str("file:///tmp/image.png").unwrap();
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
