use cidre::{cf, mtl, ns};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

struct BenchState {
    y_texture: cf::Retained<mtl::Texture>,
    cbcr_texture: cf::Retained<mtl::Texture>,
    bgra_texture: cf::Retained<mtl::Texture>,
    macro_state: cf::Retained<mtl::ComputePipelineState>,
    matrix_state: cf::Retained<mtl::ComputePipelineState>,
    render_state: cf::Retained<mtl::RenderPipelineState>,
    render_pass_desc: cf::Retained<mtl::RenderPassDescriptor>,
    vertex_buf: cf::Retained<mtl::Buffer>,
}

fn foo() -> BenchState {
    let device = mtl::Device::default().unwrap();

    let mut y_texture_desc = mtl::TextureDescriptor::new_2d_with_pixel_format(
        mtl::PixelFormat::R8Unorm,
        1920,
        1080,
        false,
    );
    y_texture_desc.set_usage(mtl::TextureUsage::SHADER_READ);
    let mut cbcr_texture_desc = mtl::TextureDescriptor::new_2d_with_pixel_format(
        mtl::PixelFormat::RG8Unorm,
        1920 / 2,
        1080 / 2,
        false,
    );
    cbcr_texture_desc.set_usage(mtl::TextureUsage::SHADER_READ);
    let mut bgra_texture_desc = mtl::TextureDescriptor::new_2d_with_pixel_format(
        mtl::PixelFormat::BGRA8Unorm,
        1920,
        1080,
        false,
    );
    bgra_texture_desc.set_usage(mtl::TextureUsage::SHADER_WRITE);

    let y_texture = device.texture_with_descriptor(&y_texture_desc).unwrap();
    let cbcr_texture = device.texture_with_descriptor(&cbcr_texture_desc).unwrap();
    let bgra_texture = device.texture_with_descriptor(&bgra_texture_desc).unwrap();

    let source = r#"
    using namespace metal;

    #define ycbcr2rgba(y, cbcr) half4( \
        y + 1.402h * cbcr.g, \
        y - (0.114h * 1.772h * cbcr.r + 0.299h * 1.402h * cbcr.g) / 0.587h, \
        y + 1.772h * cbcr.r, \
        1.0h \
    )

    typedef struct {
        float4 position [[position]];
        float2 texcoord;
    } Varyings;

    typedef struct {
        packed_float2 position;
        packed_float2 texcoord;
    } Vertex;

    vertex float4 vertex_passthrough(
        const device packed_float3* vertex_array [[ buffer(0) ]],
        unsigned int vid [[ vertex_id ]]) {
        return float4(vertex_array[vid], 1.0);
    }
    vertex Varyings vertex_passthrough2(
        unsigned short vid [[ vertex_id ]],
        constant Vertex *verticies [[buffer(0)]]
    )
    {
        Varyings out;
        constant Vertex &v = verticies[vid];
        out.position = float4(float2(v.position), 0.0, 1.0);
        out.texcoord = v.texcoord;
        
        return out;
    }

    fragment half4 fragment_y_cbcr(
        Varyings in [[stage_in]],
        texture2d<half, access::sample> yt [[texture(0)]],
        texture2d<half, access::sample> cbcrt [[texture(1)]]
    ) {
        constexpr sampler s = sampler(coord::normalized, address::clamp_to_edge, filter::nearest);
        half y = yt.sample(s, in.texcoord).r;
        half2 cbcr = cbcrt.sample(s, in.texcoord).rg - 0.5h;
        return ycbcr2rgba(y, cbcr);
    }

    kernel void matrix_transform(
        texture2d<half, access::read> yt [[texture(0)]],
        texture2d<half, access::read> cbcrt [[texture(1)]],
        texture2d<half, access::write> rgbat [[texture(2)]],
        ushort2 tid [[thread_position_in_grid]]
    ) {
        ushort2 p00 = tid * ushort(2);
        ushort2 p01 = p00 + ushort2(0, 1);
        ushort2 p10 = p00 + ushort2(1, 0);
        ushort2 p11 = p00 + ushort2(1, 1);

        half y00 = yt.read(p00).r;
        half y01 = yt.read(p01).r;
        half y10 = yt.read(p10).r;
        half y11 = yt.read(p11).r;

        half2 cbcr = cbcrt.read(p00).rg;

        const half4x4 ycbcrToRGBTransform = half4x4(
            half4(+1.0000h, +1.0000h, +1.0000h, +0.0000h),
            half4(+0.0000h, -0.3441h, +1.7720h, +0.0000h),
            half4(+1.4020h, -0.7141h, +0.0000h, +0.0000h),
            half4(-0.7010h, +0.5291h, -0.8860h, +1.0000h)
        );

        half4 rgba00 = ycbcrToRGBTransform * half4(y00, cbcr, 1.0h);
        half4 rgba01 = ycbcrToRGBTransform * half4(y01, cbcr, 1.0h);
        half4 rgba10 = ycbcrToRGBTransform * half4(y10, cbcr, 1.0h);
        half4 rgba11 = ycbcrToRGBTransform * half4(y11, cbcr, 1.0h);

        rgbat.write(rgba00, p00);
        rgbat.write(rgba01, p01);
        rgbat.write(rgba10, p10);
        rgbat.write(rgba11, p11);

    }

    kernel void macro_tranform(
        texture2d<half, access::read> yt [[texture(0)]],
        texture2d<half, access::read> cbcrt [[texture(1)]],
        texture2d<half, access::write> rgbat [[texture(2)]],
        ushort2 tid [[thread_position_in_grid]]
    ) {
        ushort2 p00 = tid * ushort(2);
        ushort2 p01 = p00 + ushort2(0, 1);
        ushort2 p10 = p00 + ushort2(1, 0);
        ushort2 p11 = p00 + ushort2(1, 1);

        half y00 = yt.read(p00).r;
        half y01 = yt.read(p01).r;
        half y10 = yt.read(p10).r;
        half y11 = yt.read(p11).r;

        half2 cbcr = cbcrt.read(p00).rg;

        half4 rgba00 = ycbcr2rgba(y00, cbcr);
        half4 rgba01 = ycbcr2rgba(y01, cbcr);
        half4 rgba10 = ycbcr2rgba(y10, cbcr);
        half4 rgba11 = ycbcr2rgba(y11, cbcr);

        rgbat.write(rgba00, p00);
        rgbat.write(rgba01, p01);
        rgbat.write(rgba10, p10);
        rgbat.write(rgba11, p11);

    }
"#;

    let source = cf::String::from_str(source);

    let lib = device.library_with_source(&source, None).unwrap();

    let matrix_transfrom_fn = lib
        .new_function_with_name(&cf::String::from_str("matrix_transform"))
        .unwrap();
    let macro_transfrom_fn = lib
        .new_function_with_name(&cf::String::from_str("macro_tranform"))
        .unwrap();

    let matrix_state = device
        .compute_pipeline_state_with_function(&matrix_transfrom_fn)
        .unwrap();
    let macro_state = device
        .compute_pipeline_state_with_function(&macro_transfrom_fn)
        .unwrap();

    let vert_fn = lib
        .new_function_with_name(&cf::String::from_str("vertex_passthrough2"))
        .unwrap();
    let frag_fn = lib
        .new_function_with_name(&cf::String::from_str("fragment_y_cbcr"))
        .unwrap();

    let mut render_desc = mtl::RenderPipelineDescriptor::new();

    render_desc.set_fragment_function(Some(&frag_fn));
    render_desc.set_vertex_function(Some(&vert_fn));

    let render_state = device
        .render_pipeline_state_with_descriptor(&render_desc)
        .unwrap();

    let render_pass_desc = mtl::RenderPassDescriptor::default();
    let arr = render_pass_desc.color_attachments();
    let foo = arr.get_mut_at(0);
    foo.set_load_action(mtl::LoadAction::DontCare);
    foo.set_store_action(mtl::StoreAction::DontCare);

    let mut desc = mtl::TextureDescriptor::new_2d_with_pixel_format(
        mtl::PixelFormat::BGRA8Unorm,
        1920,
        1080,
        false,
    );
    desc.set_usage(mtl::TextureUsage::RENDER_TARGET);
    desc.set_storage_mode(mtl::StorageMode::MemoryLess);

    let text = device.texture_with_descriptor(&desc).unwrap();

    foo.set_texture(Some(&text));

    let vertex_data = [
        -1.0f32, -1.0, 0.0, 1.0, // -
        -1.0, 1.0, 0.0, 0.0, // -
        1.0, -1.0, 1.0, 1.0, // -
        1.0, 1.0, 1.0, 0.0,
    ];

    let vertex_buf = device
        .buffer_with_bytes_length_and_options(
            vertex_data.as_ptr() as _,
            (vertex_data.len() * std::mem::size_of::<f32>()) as _,
            mtl::ResourceOptions::CPU_CACHE_MODE_DEFAULT,
        )
        .unwrap();

    // foo.set_texture(value)

    BenchState {
        y_texture,
        cbcr_texture,
        bgra_texture,
        macro_state,
        matrix_state,
        render_state,
        render_pass_desc: render_pass_desc.retained(),
        vertex_buf,
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let state = foo();

    let queue = state.bgra_texture.device().command_queue().unwrap();

    let depth = 1;
    let width = state.matrix_state.thread_execution_width();
    let height = state.matrix_state.max_total_threads_per_threadgroup() / width;
    let width_1 = state.macro_state.thread_execution_width();
    let height_1 = state.macro_state.max_total_threads_per_threadgroup() / width_1;

    assert_eq!(height_1, height);
    assert_eq!(width_1, width);

    let threads_per_group = mtl::Size {
        width,
        height,
        depth,
    };
    let grid_size = mtl::Size {
        width: 1920 / 2,
        height: 1080 / 2,
        depth,
    };

    c.bench_function("matrix", |b| {
        b.iter(|| {
            let cmd_buf = queue.command_buffer().unwrap();
            let encoder = cmd_buf.compute_command_encoder().unwrap();
            encoder.set_compute_pipeline_state(&state.matrix_state);
            encoder.set_texture_at_index(Some(&state.y_texture), 0);
            encoder.set_texture_at_index(Some(&state.cbcr_texture), 1);
            encoder.set_texture_at_index(Some(&state.bgra_texture), 2);
            encoder.dispatch_threads(grid_size, threads_per_group);
            encoder.end_encoding();
            cmd_buf.commit();
            cmd_buf.wait_until_completed();
        })
    });

    c.bench_function("macro", |b| {
        b.iter(|| {
            let cmd_buf = queue.command_buffer().unwrap();
            let encoder = cmd_buf.compute_command_encoder().unwrap();
            encoder.set_compute_pipeline_state(&state.macro_state);
            encoder.set_texture_at_index(Some(&state.y_texture), 0);
            encoder.set_texture_at_index(Some(&state.cbcr_texture), 1);
            encoder.set_texture_at_index(Some(&state.bgra_texture), 2);
            encoder.dispatch_threads(grid_size, threads_per_group);
            encoder.end_encoding();
            cmd_buf.commit();
            cmd_buf.wait_until_completed();
        })
    });

    c.bench_function("macro untracked", |b| {
        b.iter(|| {
            let cmd_buf = queue.command_buffer_with_unretained_refs().unwrap();
            let encoder = cmd_buf.compute_command_encoder().unwrap();
            encoder.set_compute_pipeline_state(&state.macro_state);
            encoder.set_texture_at_index(Some(&state.y_texture), 0);
            encoder.set_texture_at_index(Some(&state.cbcr_texture), 1);
            encoder.set_texture_at_index(Some(&state.bgra_texture), 2);
            encoder.dispatch_threads(grid_size, threads_per_group);
            encoder.end_encoding();
            cmd_buf.commit();
            cmd_buf.wait_until_completed();
        })
    });

    let textures: [&mtl::Texture; 3] = [&state.y_texture, &state.cbcr_texture, &state.bgra_texture];
    let textures_ptr = textures.as_ptr();
    let range = ns::Range::new(0, 3);

    c.bench_function("macro with textures", |b| {
        b.iter(|| {
            let cmd_buf = queue.command_buffer().unwrap();
            let encoder = cmd_buf.compute_command_encoder().unwrap();
            encoder.set_compute_pipeline_state(&state.macro_state);
            encoder.set_textures_with_range(textures_ptr, range);
            encoder.dispatch_threads(grid_size, threads_per_group);
            encoder.end_encoding();
            cmd_buf.commit();
            cmd_buf.wait_until_completed();
        })
    });

    c.bench_function("render", |b| {
        b.iter(|| {
            let cmd_buf = queue.command_buffer().unwrap();
            let encoder = cmd_buf
                .render_command_encoder_with_descriptor(&state.render_pass_desc)
                .unwrap();
            encoder.set_render_pipeline_state(&state.render_state);
            encoder.set_vertex_buffer(Some(&state.vertex_buf), 0, 0);
            encoder.set_fragment_texture_at(Some(&state.y_texture), 0);
            encoder.set_fragment_texture_at(Some(&state.cbcr_texture), 1);
            encoder.draw_primitives(mtl::PrimitiveType::TriangleStrip, 0, 4);
            encoder.end_encoding();
            cmd_buf.commit();
            cmd_buf.wait_until_completed();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
