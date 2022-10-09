use cidre::{cf, mtl, objc::autoreleasepool, simd};

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
            
typedef struct {
  packed_float2 position;
  packed_float4 color;
} Vertex;

typedef struct {
  float4 position [[position]];
  float4 color;
} Varyings;

vertex Varyings vertexPassthrough(
  unsigned short vid [[ vertex_id ]],
  constant Vertex *verticies [[buffer(0)]]
) {
  Varyings out;
  constant Vertex &v = verticies[vid];
  out.position = float4(float2(v.position), 0.0, 1.0);
  out.color = v.color;

  return out;
}

fragment float4 fragmentColorConversion (
  Varyings in [[stage_in]]
) {
  return in.color;
}


"###;

fn main() {
    autoreleasepool(|| {
        let device = mtl::Device::default().unwrap();

        let command_queue = device.command_queue().unwrap();
        let command_buffer = command_queue.command_buffer().unwrap();

        let cmd_queue = command_buffer.command_queue();
        let dev = cmd_queue.device();

        assert!(dev.as_type_ref().equal(device.as_type_ref()));
        assert!(cmd_queue.as_type_ref().equal(command_queue.as_type_ref()));

        let source = cf::String::from_str(LIB_SRC);

        let lib = device.library_with_source(&source, None).unwrap();

        let vertex_fn_name = cf::String::from_str("vertexPassthrough");
        let vertex_fn = lib.new_function_with_name(&vertex_fn_name).unwrap();

        let fragment_fn_name = cf::String::from_str("fragmentColorConversion");
        let fragment_fn = lib.new_function_with_name(&fragment_fn_name).unwrap();

        let mut desc = mtl::RenderPipelineDescriptor::new();
        desc.set_vertex_function(Some(&vertex_fn));
        desc.set_fragment_function(Some(&fragment_fn));
        let attachments = desc.color_attachments_mut();
        let at = &mut attachments[0];
        at.set_pixel_format(mtl::PixelFormat::RGBA8Unorm);

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
        render_pass_desc.set_render_target_height(1080);
        render_pass_desc.set_render_target_width(1920);
        let attachments = render_pass_desc.color_attachments_mut();
        let first = &mut attachments[0];
        first.set_load_action(mtl::LoadAction::Clear);
        first.set_store_action(mtl::StoreAction::Store);
        first.set_texture(Some(&rgba_texture));

        let triangle = [
            Vertex::with((0.0, 1.0), (1.0, 1.0, 1.0)),
            Vertex::with((0.0, 1.0), (1.0, 1.0, 1.0)),
            Vertex::with((0.0, 1.0), (1.0, 1.0, 1.0)),
        ];

        let mem_size = std::mem::size_of::<Vertex>() * triangle.len();
        let buffer = device
            .buffer_with_bytes_length_and_options(
                triangle.as_ptr() as _,
                mem_size,
                Default::default(),
            )
            .unwrap();

        let encoder = command_buffer
            .render_command_encoder_with_descriptor(&render_pass_desc)
            .unwrap();

        encoder.set_render_pipeline_state(&pipeline_state);

        encoder.set_vertex_buffer(Some(&buffer), 0, 0);

        encoder.draw_primitives(mtl::PrimitiveType::Triangle, 0, 3);
        encoder.end_encoding();

        command_buffer.commit();

        command_buffer.wait_until_completed();

        println!("{:?}", rgba_texture);
        println!("done");
    });
}
