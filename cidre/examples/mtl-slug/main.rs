/// Port of https://github.com/metal-by-example/MetalSlug
/// but with buffers instead of textures
#[cfg(target_os = "macos")]
mod macos {
    use std::{collections::HashMap, env, mem};

    use cidre::{arc, cf, cg, ci, ct, mtl, ns, objc::ar_pool};
    use half::f16;

    const DEFAULT_OUTPUT_PATH: &str = "/tmp/mtl-slug.png";
    const DEFAULT_FONT_SIZE: f64 = 12.0;
    const DEMO_WIDTH: usize = 2048;
    const DEMO_HEIGHT: usize = 2048;
    const NUM_HORIZ_BANDS: usize = 8;
    const NUM_VERT_BANDS: usize = 8;
    const DILATION_MARGIN: f32 = 0.02;
    const BAND_EPSILON: f32 = 1.0 / 1024.0;
    const LAYOUT_LIMIT: f64 = 1_000_000.0;

    static SHADER_SRC: &str = r###"
#include <metal_stdlib>
using namespace metal;

struct ViewConstants {
    float4x4 modelViewProjectionMatrix;
    float2 viewportSize;
};

struct VertexIn {
    float4 posAndNorm [[attribute(0)]];
    float4 texAndAtlasOffsets [[attribute(1)]];
    float4 invJacobian [[attribute(2)]];
    float4 bandTransform [[attribute(3)]];
    float4 color [[attribute(4)]];
};

struct VertexOut {
    float4 position [[position]];
    float4 color;
    float2 texCoords;
    float4 bandTransform [[flat]];
    uint2 glyph [[flat]];
};

static float2 dilate(float2 position, float2 vertNorm, float2 tex,
                     float2x2 invJacobian, float4x4 mvpTranspose, float2 viewportSize,
                     thread float2 &outDilatedPosition)
{
    float4 m0 = mvpTranspose[0], m1 = mvpTranspose[1], m3 = mvpTranspose[3];

    float2 norm = normalize(vertNorm);
    float s = dot(m3.xy, position.xy) + m3.w;
    float t = dot(m3.xy, norm);

    float u = (s * dot(m0.xy, norm) - t * (dot(m0.xy, position.xy) + m0.w)) * viewportSize.x;
    float v = (s * dot(m1.xy, norm) - t * (dot(m1.xy, position.xy) + m1.w)) * viewportSize.y;

    float s2 = s * s;
    float st = s * t;
    float uv = u * u + v * v;
    float2 dir = vertNorm * (s2 * (st + sqrt(uv)) / (uv - st * st));

    outDilatedPosition = position + dir;
    return tex + invJacobian * dir;
}

static inline uint2 unpack(float2 packed) {
    return as_type<uint2>(packed);
}

[[vertex]]
VertexOut glyph_vertex(VertexIn in [[stage_in]], constant ViewConstants &view [[buffer(1)]]) {
    float2x2 invJacobian { in.invJacobian.xy, in.invJacobian.zw };
    float2 dilatedPosition{};
    float2 dilatedUV = dilate(in.posAndNorm.xy, in.posAndNorm.zw, in.texAndAtlasOffsets.xy, invJacobian,
                              transpose(view.modelViewProjectionMatrix), view.viewportSize,
                              dilatedPosition);
    VertexOut out {
        .position = view.modelViewProjectionMatrix * float4(dilatedPosition, 0, 1),
        .color = in.color,
        .texCoords = dilatedUV,
        .bandTransform = in.bandTransform,
        .glyph = unpack(in.texAndAtlasOffsets.zw),
    };
    return out;
}

static uint classify_roots(float y1, float y2, float y3) {
    uint i1 = as_type<uint>(y1) >> 31u;
    uint i2 = as_type<uint>(y2) >> 30u;
    uint i3 = as_type<uint>(y3) >> 29u;
    uint shift = (i2 & 2u) | (i1 & ~2u);
    shift = (i3 & 4u) | (shift & ~4u);
    return (0x2E74u >> shift) & 0x0101u;
}

static float2 solve_poly(float2 p1, float2 p2, float2 p3, int axis) {
    float2 a = p1 - p2 * 2.0f + p3;
    float2 b = p1 - p2;
    float disc = sqrt(max(b[1 - axis] * b[1 - axis] - a[1 - axis] * p1[1 - axis], 0.0f));
    float ra = 1.0f / a[1 - axis];
    float t1 = (b[1 - axis] - disc) * ra;
    float t2 = (b[1 - axis] + disc) * ra;
    if (abs(a[1 - axis]) < 1e-5) {
        float r2b = 1.0f / (b[1 - axis] * 2.0f);
        t1 = p1[1 - axis] * r2b;
        t2 = p1[1 - axis] * r2b;
    }
    return float2((a[axis] * t1 - b[axis] * 2.0f) * t1 + p1[axis],
                  (a[axis] * t2 - b[axis] * 2.0f) * t2 + p1[axis]);
}

[[fragment]]
float4 glyph_fragment(VertexOut in [[stage_in]],
                    device const half4 *curveData [[buffer(0)]],
                    device const uint  *bandData  [[buffer(1)]])
{
    float2 emUV = in.texCoords;
    float2 emsPerPixel = fwidth(emUV);
    float2 pixelsPerEm = 1.0f / emsPerPixel;

    uint bandStart = in.glyph.x;
    int2 bandMax = int2(int(in.glyph.y & 0xFFFFu), int(in.glyph.y >> 16u));

    float2 bandScale = in.bandTransform.xy;
    float2 bandOffset = in.bandTransform.zw;

    int2 bandIndex = clamp(int2(emUV * bandScale + bandOffset), int2(0, 0), bandMax);

    uint hHeader = bandStart + uint(bandIndex.y) * 2u;
    uint hCount = bandData[hHeader + 0u];
    uint hList = bandStart + bandData[hHeader + 1u];
    float xCoverage = 0.0f, xWeight = 0.0f;
    for (int ci = 0; ci < int(hCount); ci++) {
        uint cl = bandData[hList + uint(ci)];

        float4 p12 = float4(curveData[cl]) - float4(emUV, emUV);
        float2 p1 = p12.xy, p2 = p12.zw;
        float2 p3 = float2(curveData[cl + 1u].xy) - emUV;

        if (max(max(p1.x, p2.x), p3.x) * pixelsPerEm.x < -0.5f) {
            break;
        }

        uint code = classify_roots(p1.y, p2.y, p3.y);
        if (code != 0u) {
            float2 r = solve_poly(p12.xy, p12.zw, p3, 0) * pixelsPerEm.x;
            if ((code & 1u) != 0u) {
                xCoverage += saturate(r.x + 0.5f);
                xWeight = max(xWeight, saturate(1.0f - abs(r.x) * 2.0f));
            }
            if (code > 1u) {
                xCoverage -= saturate(r.y + 0.5f);
                xWeight  = max(xWeight, saturate(1.0f - abs(r.y) * 2.0f));
            }
        }
    }

    uint vHeader = bandStart + uint(bandMax.y + 1 + bandIndex.x) * 2u;
    uint vCount = bandData[vHeader + 0u];
    uint vList = bandStart + bandData[vHeader + 1u];
    float yCoverage = 0.0f, yWeight = 0.0f;
    for (int ci = 0; ci < int(vCount); ci++) {
        uint cl = bandData[vList + uint(ci)];

        float4 p12 = float4(curveData[cl]) - float4(emUV, emUV);
        float2 p1 = p12.xy, p2 = p12.zw;
        float2 p3  = float2(curveData[cl + 1u].xy) - emUV;

        if (max(max(p12.y, p12.w), p3.y) * pixelsPerEm.y < -0.5f) {
            break;
        }

        uint code = classify_roots(p1.x, p2.x, p3.x);
        if (code != 0u) {
            float2 r = solve_poly(p1, p2, p3, 1) * pixelsPerEm.y;
            if ((code & 1u) != 0u) {
                yCoverage -= saturate(r.x + 0.5f);
                yWeight  = max(yWeight, saturate(1.0f - abs(r.x) * 2.0f));
            }
            if (code > 1u) {
                yCoverage += saturate(r.y + 0.5f);
                yWeight  = max(yWeight, saturate(1.0f - abs(r.y) * 2.0f));
            }
        }
    }

    float denom = max(xWeight + yWeight, 1.0f / 65536.0f);
    float coverage = max(abs(xCoverage * xWeight + yCoverage * yWeight) / denom, min(abs(xCoverage), abs(yCoverage)));
    coverage = saturate(coverage);

    return in.color * coverage;
}
"###;

    static DEFAULT_PHRASES: &[&str] = &[
        "我能吞下玻璃而不伤身体。\n",
        "私はガラスを食べられます。それは私を傷つけません。\n",
        "Μπορῶ νὰ φάω σπασμένα γυαλιὰ χωρὶς νὰ πάθω τίποτα.\n",
        "ᛁᚳ᛫ᛗᚨᚷ᛫ᚷᛚᚨᛋ᛫ᛖᚩᛏᚪᚾ᛫ᚩᚾᛞ᛫ᚻᛁᛏ᛫ᚾᛖ᛫ᚻᛖᚪᚱᛗᛁᚪᚧ᛫ᛗᛖ᛬\n",
        "زه شيشه خوړلې شم، هغه ما نه خوږوي\n",
        ".من می توانم بدونِ احساس درد شيشه بخورم\n",
        "میں کانچ کھا سکتا ہوں اور مجھے تکلیف نہیں ہوتی ۔\n",
        "Я могу есть стекло, оно мне не вредит.\n",
        "𐌼𐌰𐌲 𐌲𐌻𐌴𐍃 𐌹̈𐍄𐌰𐌽, 𐌽𐌹 𐌼𐌹𐍃 𐍅𐌿 𐌽𐌳𐌰𐌽 𐌱𐍂𐌹𐌲𐌲𐌹𐌸.\n",
        "Կրնամ ապակի ուտել և ինծի անհանգիստ չըներ։\n",
        "എനിക്ക് ഗ്ലാസ് തിന്നാം. അതെന്നെ വേദനിപ്പിക്കില്ല.\n",
        "මට වීදුරු කෑමට හැකියි. එයින් මට කිසි හානියක් සිදු නොවේ.\n",
        "אני יכול לאכול זכוכית וזה לא מזיק לי.\n",
        "ကျွန်တော် ကျွန်မ မှန်စားနိုင်တယ်။ ၎င်းကြောင့် ထိခိုက်မှုမရှိပါ။\n",
        "ខ្ញុំអាចញុំកញ្ចក់បាន ដោយគ្មានបញ្ហារ\n",
        "ຂອ້ຍກິນແກ້ວໄດ້ໂດຍທີ່ມັນບໍ່ໄດ້ເຮັດໃຫ້ຂອ້ຍເຈັບ.\n",
        "ฉันกินกระจกได้ แต่มันไม่ทำให้ฉันเจ็บ\n",
        "ཤེལ་སྒོ་ཟ་ནས་ང་ན་གི་མ་རེད།\n",
    ];

    #[derive(Debug, Clone)]
    struct Args {
        output_path: String,
    }

    #[derive(Debug, Clone)]
    struct SpanSpec {
        text: String,
        font_name: &'static str,
        font_size: f64,
        color_linear: [f32; 4],
    }

    #[repr(C, align(16))]
    #[derive(Clone, Copy, Default)]
    struct GlyphVertex {
        pos_and_norm: [f32; 4],
        tex_and_atlas_offsets: [f32; 4],
        inv_jacobian: [f32; 4],
        band_transform: [f32; 4],
        color: [f32; 4],
    }

    #[repr(C, align(16))]
    #[derive(Clone, Copy)]
    struct ViewConstants {
        model_view_projection_matrix: [[f32; 4]; 4],
        viewport_size: [f32; 2],
        _padding: [f32; 2],
    }

    #[derive(Clone, Copy, Default)]
    struct QuadBezier {
        p0: [f32; 2],
        p1: [f32; 2],
        p2: [f32; 2],
    }

    impl QuadBezier {
        fn min_x(self) -> f32 {
            self.p0[0].min(self.p1[0]).min(self.p2[0])
        }

        fn max_x(self) -> f32 {
            self.p0[0].max(self.p1[0]).max(self.p2[0])
        }

        fn min_y(self) -> f32 {
            self.p0[1].min(self.p1[1]).min(self.p2[1])
        }

        fn max_y(self) -> f32 {
            self.p0[1].max(self.p1[1]).max(self.p2[1])
        }

        fn is_straight_horizontal(self) -> bool {
            (self.p0[1] - self.p2[1]).abs() < 1.0e-5
                && (self.p1[1] - (self.p0[1] + self.p2[1]) * 0.5).abs() < 1.0e-5
        }

        fn is_straight_vertical(self) -> bool {
            (self.p0[0] - self.p2[0]).abs() < 1.0e-5
                && (self.p1[0] - (self.p0[0] + self.p2[0]) * 0.5).abs() < 1.0e-5
        }
    }

    #[derive(Clone, Copy, Default)]
    struct GlyphInfo {
        x_min: f32,
        y_min: f32,
        x_max: f32,
        y_max: f32,
        curve_count: usize,
        band_start: u32,
        num_horiz_bands: usize,
        num_vert_bands: usize,
        band_scale_x: f32,
        band_scale_y: f32,
        band_offset_x: f32,
        band_offset_y: f32,
    }

    struct FontAtlas {
        font: arc::R<ct::Font>,
        glyph_cache: HashMap<u16, GlyphInfo>,
        curve_data: Vec<[f16; 4]>,
        band_data: Vec<u32>,
        curve_buffer: Option<arc::R<mtl::Buf>>,
        band_buffer: Option<arc::R<mtl::Buf>>,
    }

    #[derive(Clone)]
    struct TextSubmesh {
        font_key: String,
        index_buffer_offset: usize,
        index_count: usize,
    }

    struct PreparedRun {
        font_key: String,
        font_size: f32,
        color: [f32; 4],
        glyphs: Vec<cg::Glyph>,
        positions: Vec<cg::Point>,
        line_origin: cg::Point,
    }

    struct TextMesh {
        vertex_buffer: Option<arc::R<mtl::Buf>>,
        index_buffer: Option<arc::R<mtl::Buf>>,
        submeshes: Vec<TextSubmesh>,
        bounds: cg::Rect,
    }

    struct TextRenderer {
        device: arc::R<mtl::Device>,
        render_pipeline: arc::R<mtl::RenderPipelineState>,
        atlas_cache: HashMap<String, FontAtlas>,
    }

    struct AttrStringBuilder {
        text: arc::R<cf::AttrStringMut>,
        font_cache: HashMap<String, arc::R<ct::Font>>,
    }

    struct PathExtractionState {
        subpath_start: cg::Point,
        current: cg::Point,
        curves: Vec<QuadBezier>,
    }

    impl PathExtractionState {
        fn new() -> Self {
            Self {
                subpath_start: cg::Point::zero(),
                current: cg::Point::zero(),
                curves: Vec::new(),
            }
        }
    }

    impl FontAtlas {
        fn new(font_name: &str) -> Result<Self, String> {
            let name = cf::String::from_str(font_name);
            let font = ct::Font::with_name_size(&name, 1.0);
            Ok(Self {
                font,
                glyph_cache: HashMap::new(),
                curve_data: Vec::new(),
                band_data: Vec::new(),
                curve_buffer: None,
                band_buffer: None,
            })
        }

        fn glyph_info(&self, glyph: cg::Glyph) -> GlyphInfo {
            self.glyph_cache
                .get(&glyph.0.0)
                .copied()
                .unwrap_or_default()
        }

        fn insert_glyphs(
            &mut self,
            glyphs: &[cg::Glyph],
            device: &mtl::Device,
        ) -> Result<(), String> {
            let curve_len_before = self.curve_data.len();
            let band_len_before = self.band_data.len();
            let did_update = self.ensure_glyphs(glyphs)?;
            if did_update
                && (self.curve_data.len() != curve_len_before
                    || self.band_data.len() != band_len_before)
            {
                self.upload_buffers(device)?;
            }
            Ok(())
        }

        fn ensure_glyphs(&mut self, glyphs: &[cg::Glyph]) -> Result<bool, String> {
            let mut did_update = false;
            for &glyph in glyphs {
                if self.glyph_cache.contains_key(&glyph.0.0) {
                    continue;
                }
                self.ensure_glyph(glyph)?;
                did_update = true;
            }
            Ok(did_update)
        }

        fn ensure_glyph(&mut self, glyph: cg::Glyph) -> Result<(), String> {
            let Some(path) = self.font.path_for_glyph(glyph, None) else {
                self.glyph_cache.insert(glyph.0.0, GlyphInfo::default());
                return Ok(());
            };

            let curves = extract_quad_beziers_from_path(&path);
            if curves.is_empty() {
                self.glyph_cache.insert(glyph.0.0, GlyphInfo::default());
                return Ok(());
            }

            let mut x_min = f32::INFINITY;
            let mut x_max = f32::NEG_INFINITY;
            let mut y_min = f32::INFINITY;
            let mut y_max = f32::NEG_INFINITY;
            for &curve in &curves {
                x_min = x_min.min(curve.min_x());
                x_max = x_max.max(curve.max_x());
                y_min = y_min.min(curve.min_y());
                y_max = y_max.max(curve.max_y());
            }

            let mut curve_starts = Vec::with_capacity(curves.len());
            for &curve in &curves {
                let start = self.curve_data.len() as u32;
                curve_starts.push(start);
                self.curve_data.push([
                    f16::from_f32(curve.p0[0]),
                    f16::from_f32(curve.p0[1]),
                    f16::from_f32(curve.p1[0]),
                    f16::from_f32(curve.p1[1]),
                ]);
                self.curve_data.push([
                    f16::from_f32(curve.p2[0]),
                    f16::from_f32(curve.p2[1]),
                    f16::from_f32(0.0),
                    f16::from_f32(0.0),
                ]);
            }

            let h_band_height = (y_max - y_min) / NUM_HORIZ_BANDS as f32;
            let v_band_width = (x_max - x_min) / NUM_VERT_BANDS as f32;

            let mut horiz_bands = Vec::with_capacity(NUM_HORIZ_BANDS);
            for band in 0..NUM_HORIZ_BANDS {
                let band_y_min = y_min + band as f32 * h_band_height - BAND_EPSILON;
                let band_y_max = y_min + (band + 1) as f32 * h_band_height + BAND_EPSILON;
                let mut indices = Vec::new();
                for (index, curve) in curves.iter().enumerate() {
                    if curve.is_straight_horizontal() {
                        continue;
                    }
                    if curve.max_y() >= band_y_min && curve.min_y() <= band_y_max {
                        indices.push(index);
                    }
                }
                indices.sort_by(|&lhs, &rhs| {
                    curves[rhs]
                        .max_x()
                        .partial_cmp(&curves[lhs].max_x())
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
                horiz_bands.push(indices);
            }

            let mut vert_bands = Vec::with_capacity(NUM_VERT_BANDS);
            for band in 0..NUM_VERT_BANDS {
                let band_x_min = x_min + band as f32 * v_band_width - BAND_EPSILON;
                let band_x_max = x_min + (band + 1) as f32 * v_band_width + BAND_EPSILON;
                let mut indices = Vec::new();
                for (index, curve) in curves.iter().enumerate() {
                    if curve.is_straight_vertical() {
                        continue;
                    }
                    if curve.max_x() >= band_x_min && curve.min_x() <= band_x_max {
                        indices.push(index);
                    }
                }
                indices.sort_by(|&lhs, &rhs| {
                    curves[rhs]
                        .max_y()
                        .partial_cmp(&curves[lhs].max_y())
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
                vert_bands.push(indices);
            }

            let band_start = self.band_data.len() as u32;
            let header_offset = self.band_data.len();
            self.band_data
                .resize(header_offset + (NUM_HORIZ_BANDS + NUM_VERT_BANDS) * 2, 0);

            let mut horiz_offsets = Vec::with_capacity(NUM_HORIZ_BANDS);
            for band in &horiz_bands {
                let rel_offset = (self.band_data.len() as u32) - band_start;
                horiz_offsets.push(rel_offset);
                for &curve_index in band {
                    self.band_data.push(curve_starts[curve_index]);
                }
            }

            let mut vert_offsets = Vec::with_capacity(NUM_VERT_BANDS);
            for band in &vert_bands {
                let rel_offset = (self.band_data.len() as u32) - band_start;
                vert_offsets.push(rel_offset);
                for &curve_index in band {
                    self.band_data.push(curve_starts[curve_index]);
                }
            }

            let mut header_cursor = header_offset;
            for band in 0..NUM_HORIZ_BANDS {
                self.band_data[header_cursor] = horiz_bands[band].len() as u32;
                self.band_data[header_cursor + 1] = horiz_offsets[band];
                header_cursor += 2;
            }
            for band in 0..NUM_VERT_BANDS {
                self.band_data[header_cursor] = vert_bands[band].len() as u32;
                self.band_data[header_cursor + 1] = vert_offsets[band];
                header_cursor += 2;
            }

            let band_scale_x = NUM_VERT_BANDS as f32 / (x_max - x_min).max(1.0e-6);
            let band_scale_y = NUM_HORIZ_BANDS as f32 / (y_max - y_min).max(1.0e-6);
            let band_offset_x = -x_min * band_scale_x;
            let band_offset_y = -y_min * band_scale_y;

            self.glyph_cache.insert(
                glyph.0.0,
                GlyphInfo {
                    x_min,
                    y_min,
                    x_max,
                    y_max,
                    curve_count: curves.len(),
                    band_start,
                    num_horiz_bands: NUM_HORIZ_BANDS,
                    num_vert_bands: NUM_VERT_BANDS,
                    band_scale_x,
                    band_scale_y,
                    band_offset_x,
                    band_offset_y,
                },
            );

            Ok(())
        }

        fn upload_buffers(&mut self, device: &mtl::Device) -> Result<(), String> {
            if self.curve_data.is_empty() || self.band_data.is_empty() {
                return Ok(());
            }

            self.curve_buffer = Some(
                device
                    .new_buf_with_slice(&self.curve_data, Default::default())
                    .ok_or("failed to create curve buffer")?,
            );
            self.band_buffer = Some(
                device
                    .new_buf_with_slice(&self.band_data, Default::default())
                    .ok_or("failed to create band buffer")?,
            );
            Ok(())
        }
    }

    impl TextRenderer {
        fn new(
            device: arc::R<mtl::Device>,
            color_pixel_format: mtl::PixelFormat,
        ) -> Result<Self, String> {
            let source = ns::String::with_str(SHADER_SRC);
            let library = device
                .new_lib_with_src_blocking(&source, None)
                .map_err(|err| format!("failed to compile Metal source: {err:?}"))?;

            let vertex_fn = library
                .new_fn(ns::str!(c"glyph_vertex"))
                .ok_or("missing glyph_vertex function")?;
            let fragment_fn = library
                .new_fn(ns::str!(c"glyph_fragment"))
                .ok_or("missing glyph_fragment function")?;

            let vertex_desc = mtl::VertexDesc::new();
            for attr_index in 0..5 {
                let mut attr = vertex_desc.attrs().get(attr_index);
                attr.set_format(mtl::VertexFormat::F32x4);
                attr.set_offset(attr_index * 16);
                attr.set_buf_index(0);
            }
            let mut layout = vertex_desc.layouts().get(0);
            layout.set_stride(mem::size_of::<GlyphVertex>());
            layout.set_step_fn(mtl::VertexStepFn::PerVertex);

            let mut pipeline_desc = mtl::RenderPipelineDesc::new().with_fns_vertex_desc(
                &vertex_fn,
                &fragment_fn,
                &vertex_desc,
            );
            let mut color_attach = pipeline_desc.color_attaches().get(0);
            color_attach.set_pixel_format(color_pixel_format);
            color_attach.set_blending_enabled(true);
            color_attach.set_src_rgb_blend_factor(mtl::BlendFactor::One);
            color_attach.set_dst_rgb_blend_factor(mtl::BlendFactor::OneMinusSrcAlpha);
            color_attach.set_src_alpha_blend_factor(mtl::BlendFactor::One);
            color_attach.set_dst_alpha_blend_factor(mtl::BlendFactor::OneMinusSrcAlpha);
            pipeline_desc.set_raster_sample_count(1);

            let render_pipeline = device
                .new_render_ps(&pipeline_desc)
                .map_err(|err| format!("failed to create render pipeline: {err:?}"))?;

            Ok(Self {
                device,
                render_pipeline,
                atlas_cache: HashMap::new(),
            })
        }

        fn atlas_for_font_name_mut(&mut self, font_name: &str) -> Result<&mut FontAtlas, String> {
            if !self.atlas_cache.contains_key(font_name) {
                let atlas = FontAtlas::new(font_name)?;
                self.atlas_cache.insert(font_name.to_string(), atlas);
            }
            Ok(self
                .atlas_cache
                .get_mut(font_name)
                .expect("font atlas must exist"))
        }

        fn make_text_mesh(
            &mut self,
            attr_string: &cf::AttrString,
            max_size: cg::Size,
        ) -> Result<TextMesh, String> {
            if attr_string.is_empty() {
                return Ok(TextMesh {
                    vertex_buffer: None,
                    index_buffer: None,
                    submeshes: Vec::new(),
                    bounds: cg::Rect::zero(),
                });
            }

            let framesetter = ct::Framesetter::with_attr_string(attr_string);
            let suggested_size = framesetter.suggest_frame_size_with_constraints(
                cf::Range::new(0, 0),
                None,
                max_size,
                None,
            );
            let frame_path = cg::Path::with_rect(
                cg::Rect::new(0.0, 0.0, suggested_size.width, suggested_size.height),
                None,
            );
            let frame = framesetter.create_frame(cf::Range::new(0, 0), &frame_path, None);
            let lines = frame.lines();

            let line_count = lines.len();
            let mut line_origins = vec![cg::Point::zero(); line_count];
            if !line_origins.is_empty() {
                frame.line_origins(cf::Range::new(0, 0), &mut line_origins);
            }
            let first_line_y = line_origins.first().map_or(0.0, |p| p.y);

            let mut bounds_min_x = f32::INFINITY;
            let mut bounds_min_y = f32::INFINITY;
            let mut bounds_max_x = f32::NEG_INFINITY;
            let mut bounds_max_y = f32::NEG_INFINITY;

            let mut prepared_runs = Vec::<PreparedRun>::new();
            let mut glyphs_by_font = HashMap::<String, Vec<cg::Glyph>>::new();

            for (line_index, line) in lines.iter().enumerate() {
                let line_origin = line_origins[line_index];
                let runs = line.glyph_runs();

                for run in runs.iter() {
                    let glyph_count = run.glyph_count() as usize;
                    if glyph_count == 0 {
                        continue;
                    }

                    let attrs = run.attributes();
                    let font = attrs
                        .get(ct::StringAttrName::font())
                        .map(|value| unsafe { &*(value as *const cf::Type as *const ct::Font) })
                        .ok_or("missing run font")?;
                    let font_size = font.size() as f32;
                    let font_key = font_postscript_name(font);

                    let mut glyphs_storage = vec![cg::Glyph::new(0); glyph_count];
                    run.copy_glyphs(0, &mut glyphs_storage);

                    let mut positions = vec![cg::Point::zero(); glyph_count];
                    run.copy_positions(0, &mut positions);

                    let run_color = extract_run_color(attrs);
                    glyphs_by_font
                        .entry(font_key.clone())
                        .or_default()
                        .extend_from_slice(&glyphs_storage);
                    prepared_runs.push(PreparedRun {
                        font_key,
                        font_size,
                        color: run_color,
                        glyphs: glyphs_storage,
                        positions,
                        line_origin,
                    });
                }
            }

            if !glyphs_by_font.is_empty() {
                let device = self.device.retained();
                for (font_key, glyphs) in glyphs_by_font.iter_mut() {
                    glyphs.sort_unstable_by_key(|glyph| glyph.0.0);
                    glyphs.dedup_by_key(|glyph| glyph.0.0);
                    let atlas = self.atlas_for_font_name_mut(font_key)?;
                    atlas.insert_glyphs(glyphs, device.as_ref())?;
                }
            }

            let estimated_glyph_count: usize =
                prepared_runs.iter().map(|run| run.glyphs.len()).sum();
            let mut vertices = Vec::<GlyphVertex>::with_capacity(estimated_glyph_count * 4);
            let mut indices = Vec::<u16>::with_capacity(estimated_glyph_count * 6);
            let mut submeshes = Vec::<TextSubmesh>::with_capacity(prepared_runs.len());

            for run in prepared_runs {
                let PreparedRun {
                    font_key,
                    font_size,
                    color,
                    glyphs,
                    positions,
                    line_origin,
                } = run;
                let atlas = self
                    .atlas_cache
                    .get(&font_key)
                    .ok_or_else(|| format!("missing atlas for {}", font_key))?;

                let submesh_index_start = indices.len();
                let mut submesh_index_count = 0usize;

                for (glyph_index, &glyph) in glyphs.iter().enumerate() {
                    let info = atlas.glyph_info(glyph);
                    if info.curve_count == 0 {
                        continue;
                    }

                    let pos_x = (line_origin.x + positions[glyph_index].x) as f32;
                    let pos_y = (line_origin.y - first_line_y + positions[glyph_index].y) as f32;

                    let ex0 = info.x_min - DILATION_MARGIN;
                    let ex1 = info.x_max + DILATION_MARGIN;
                    let ey0 = info.y_min - DILATION_MARGIN;
                    let ey1 = info.y_max + DILATION_MARGIN;

                    let px0 = ex0 * font_size;
                    let py0 = ey0 * font_size;
                    let px1 = ex1 * font_size;
                    let py1 = ey1 * font_size;

                    bounds_min_x = bounds_min_x.min(pos_x + px0);
                    bounds_min_y = bounds_min_y.min(pos_y + py0);
                    bounds_max_x = bounds_max_x.max(pos_x + px1);
                    bounds_max_y = bounds_max_y.max(pos_y + py1);

                    let tex_z = f32::from_bits(info.band_start);
                    let bmax_x = (info.num_vert_bands - 1) as u32;
                    let bmax_y = (info.num_horiz_bands - 1) as u32;
                    let tex_w = f32::from_bits(bmax_x | (bmax_y << 16));

                    let band_transform = [
                        info.band_scale_x,
                        info.band_scale_y,
                        info.band_offset_x,
                        info.band_offset_y,
                    ];

                    let inv_scale = if font_size.abs() > f32::EPSILON {
                        1.0 / font_size
                    } else {
                        1.0
                    };
                    let inv_jacobian = [inv_scale, 0.0, 0.0, inv_scale];

                    let corners = [
                        (px0, py0, ex0, ey0),
                        (px1, py0, ex1, ey0),
                        (px1, py1, ex1, ey1),
                        (px0, py1, ex0, ey1),
                    ];

                    let base_index = vertices.len();
                    if base_index > (u16::MAX as usize).saturating_sub(4) {
                        return Err("text mesh exceeded 16-bit index capacity".to_string());
                    }

                    for &(px, py, ex, ey) in &corners {
                        let normal = normalize2([ex, ey]);
                        vertices.push(GlyphVertex {
                            pos_and_norm: [pos_x + px, pos_y + py, normal[0], normal[1]],
                            tex_and_atlas_offsets: [ex, ey, tex_z, tex_w],
                            inv_jacobian,
                            band_transform,
                            color,
                        });
                    }

                    indices.extend_from_slice(&[
                        base_index as u16,
                        (base_index + 1) as u16,
                        (base_index + 2) as u16,
                        base_index as u16,
                        (base_index + 2) as u16,
                        (base_index + 3) as u16,
                    ]);
                    submesh_index_count += 6;
                }

                if submesh_index_count > 0 {
                    submeshes.push(TextSubmesh {
                        font_key,
                        index_buffer_offset: submesh_index_start * mem::size_of::<u16>(),
                        index_count: submesh_index_count,
                    });
                }
            }

            let bounds = if bounds_max_x > bounds_min_x && bounds_max_y > bounds_min_y {
                cg::Rect::new(
                    bounds_min_x as f64,
                    bounds_min_y as f64,
                    (bounds_max_x - bounds_min_x) as f64,
                    (bounds_max_y - bounds_min_y) as f64,
                )
            } else {
                cg::Rect::zero()
            };

            let vertex_buffer = if vertices.is_empty() {
                None
            } else {
                Some(
                    self.device
                        .new_buf_with_slice(&vertices, Default::default())
                        .ok_or("failed to create vertex buffer")?,
                )
            };

            let index_buffer = if indices.is_empty() {
                None
            } else {
                Some(
                    self.device
                        .new_buf_with_slice(&indices, Default::default())
                        .ok_or("failed to create index buffer")?,
                )
            };

            Ok(TextMesh {
                vertex_buffer,
                index_buffer,
                submeshes,
                bounds,
            })
        }

        fn render_mesh(
            &self,
            mesh: &TextMesh,
            view: &ViewConstants,
            encoder: &mut mtl::RenderCmdEncoder,
        ) -> Result<(), String> {
            if mesh.submeshes.is_empty() {
                return Ok(());
            }

            let Some(vertex_buffer) = mesh.vertex_buffer.as_ref() else {
                return Ok(());
            };
            let Some(index_buffer) = mesh.index_buffer.as_ref() else {
                return Ok(());
            };

            encoder.set_render_ps(&self.render_pipeline);
            encoder.set_vertex_buf_at(Some(vertex_buffer.as_ref()), 0, 0);
            encoder.copy_to_vertex_at(view, 1);
            encoder.set_cull_mode(mtl::CullMode::None);

            let mut bound_font_key: Option<&str> = None;
            for submesh in &mesh.submeshes {
                if bound_font_key != Some(submesh.font_key.as_str()) {
                    let atlas = self
                        .atlas_cache
                        .get(&submesh.font_key)
                        .ok_or_else(|| format!("missing atlas for {}", submesh.font_key))?;
                    let curve_buffer = atlas
                        .curve_buffer
                        .as_ref()
                        .ok_or_else(|| format!("missing curve buffer for {}", submesh.font_key))?;
                    let band_buffer = atlas
                        .band_buffer
                        .as_ref()
                        .ok_or_else(|| format!("missing band buffer for {}", submesh.font_key))?;
                    encoder.set_fragment_buf_at(Some(curve_buffer.as_ref()), 0, 0);
                    encoder.set_fragment_buf_at(Some(band_buffer.as_ref()), 0, 1);
                    bound_font_key = Some(submesh.font_key.as_str());
                }

                encoder.draw_indexed_primitives(
                    mtl::Primitive::Triangle,
                    submesh.index_count,
                    mtl::IndexType::U16,
                    index_buffer.as_ref(),
                    submesh.index_buffer_offset,
                );
            }

            Ok(())
        }
    }

    impl AttrStringBuilder {
        fn new() -> Self {
            Self {
                text: cf::AttrStringMut::with_max_len(0),
                font_cache: HashMap::new(),
            }
        }

        fn font(&mut self, font_name: &str, font_size: f64) -> arc::R<ct::Font> {
            let key = format!("{font_name}:{font_size}");
            if let Some(font) = self.font_cache.get(&key) {
                return font.retained();
            }

            let font = ct::Font::with_name_size(&cf::String::from_str(font_name), font_size);
            self.font_cache.insert(key, font.retained());
            font
        }

        fn append_span(&mut self, span: &SpanSpec) {
            let font = self.font(span.font_name, span.font_size);
            let color = cg::Color::generic_rgba(
                span.color_linear[0] as f64,
                span.color_linear[1] as f64,
                span.color_linear[2] as f64,
                span.color_linear[3] as f64,
            );
            let attrs = cf::Dictionary::with_keys_values(
                &[
                    ct::StringAttrName::font().as_ref(),
                    ct::StringAttrName::foreground_color().as_ref(),
                ],
                &[font.as_ref(), color.as_ref()],
            )
            .expect("attribute dictionary");

            let replacement = cf::String::from_str(&span.text);
            let start = self.text.len() as isize;
            let replacement_len = replacement.len() as isize;
            self.text
                .replace_string(cf::Range::new(start, 0), &replacement);
            self.text
                .set_attributes(cf::Range::new(start, replacement_len), &attrs, true);
        }

        fn build(mut self, spans: &[SpanSpec]) -> arc::R<cf::AttrString> {
            for span in spans {
                self.append_span(span);
            }
            self.text.copy()
        }
    }

    pub fn main() {
        let result = ar_pool(run);
        if let Err(err) = result {
            eprintln!("error: {err}");
            std::process::exit(1);
        }
    }

    fn run() -> Result<(), String> {
        let args = parse_args()?;
        render_demo_png(&args.output_path)?;
        Ok(())
    }

    fn parse_args() -> Result<Args, String> {
        let mut output_path = DEFAULT_OUTPUT_PATH.to_string();
        let mut iter = env::args().skip(1);

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--output" => {
                    output_path = iter.next().ok_or("--output requires a filesystem path")?;
                }
                "--help" | "-h" => {
                    print_usage();
                    std::process::exit(0);
                }
                other => return Err(format!("unknown argument: {other}")),
            }
        }

        Ok(Args { output_path })
    }

    fn print_usage() {
        println!("Usage:");
        println!("  cargo run --example mtl-slug -- [--output PATH]");
    }

    fn render_demo_png(output_path: &str) -> Result<(), String> {
        let device = mtl::Device::sys_default().ok_or("no Metal device available")?;
        let command_queue = device
            .new_cmd_queue()
            .ok_or("failed to create command queue")?;

        let attr_string = build_demo_text();
        let mut renderer = TextRenderer::new(device.retained(), mtl::PixelFormat::Bgra8UNormSrgb)?;
        let mesh =
            renderer.make_text_mesh(&attr_string, cg::Size::new(LAYOUT_LIMIT, LAYOUT_LIMIT))?;
        let view = ViewConstants {
            model_view_projection_matrix: make_fit_transform(mesh.bounds),
            viewport_size: [DEMO_WIDTH as f32, DEMO_HEIGHT as f32],
            _padding: [0.0, 0.0],
        };

        let target = make_render_target(&device, DEMO_WIDTH, DEMO_HEIGHT)?;
        let render_pass = mtl::RenderPassDesc::new();
        let mut color_attach = render_pass.color_attaches().get(0);
        color_attach.set_clear_color(mtl::ClearColor::new(0.02, 0.02, 0.02, 1.0));
        color_attach.set_load_action(mtl::LoadAction::Clear);
        color_attach.set_store_action(mtl::StoreAction::Store);
        color_attach.set_texture(Some(&target));

        let mut command_buffer = command_queue
            .new_cmd_buf_unretained_refs()
            .ok_or("failed to create command buffer")?;

        command_buffer.render(&render_pass, |encoder| {
            encoder.set_vp_rect(0.0, 0.0, DEMO_WIDTH as f64, DEMO_HEIGHT as f64);
            renderer.render_mesh(&mesh, &view, encoder).unwrap();
        });
        command_buffer.commit();
        command_buffer.wait_until_completed();

        let image = ci::Image::with_mtl_texture(&target, None).ok_or("failed to wrap CI image")?;
        let context = ci::Context::new();
        let color_space = cg::ColorSpace::device_rgb().ok_or("failed to create RGB color space")?;
        let url = ns::Url::with_fs_path_str(output_path, false);
        context
            .write_png_to_url(
                &image,
                &url,
                ci::Format::rgba8(),
                &color_space,
                ns::Dictionary::new().as_ref(),
            )
            .map_err(|err| format!("failed to write PNG: {err:?}"))?;

        println!("image is written to {}", output_path);
        Ok(())
    }

    fn build_demo_text() -> arc::R<cf::AttrString> {
        let mut spans = Vec::new();
        let title = "This is Slug rendered with Metal\n";
        let char_count = title.chars().count().max(1);
        for (index, ch) in title.chars().enumerate() {
            let hue = if char_count > 1 {
                index as f32 / (char_count as f32 - 1.0)
            } else {
                0.0
            };
            let rgb = rgb_from_hue(hue);
            spans.push(SpanSpec {
                text: ch.to_string(),
                font_name: "Zapfino",
                font_size: DEFAULT_FONT_SIZE,
                color_linear: linearize_rgba([rgb[0], rgb[1], rgb[2], 1.0]),
            });
        }

        for phrase in DEFAULT_PHRASES {
            spans.push(SpanSpec {
                text: (*phrase).to_string(),
                font_name: "HelveticaNeue",
                font_size: DEFAULT_FONT_SIZE,
                color_linear: [1.0, 1.0, 1.0, 1.0],
            });
        }

        AttrStringBuilder::new().build(&spans)
    }

    fn make_render_target(
        device: &mtl::Device,
        width: usize,
        height: usize,
    ) -> Result<arc::R<mtl::Texture>, String> {
        let desc = mtl::TextureDesc::new_2d(mtl::PixelFormat::Bgra8UNormSrgb, width, height, false);
        device
            .new_texture(&desc)
            .ok_or("failed to create render target".to_string())
    }

    fn font_postscript_name(font: &ct::Font) -> String {
        font.post_script_name().to_string()
    }

    fn extract_run_color(attrs: &cf::DictionaryOf<cf::String, cf::Type>) -> [f32; 4] {
        let Some(color_type) = attrs.get(ct::StringAttrName::foreground_color()) else {
            return [1.0, 1.0, 1.0, 1.0];
        };
        let color = unsafe { &*(color_type as *const cf::Type as *const cg::Color) };
        let Some(slice) = color.components() else {
            return [1.0, 1.0, 1.0, 1.0];
        };
        if slice.len() >= 4 {
            [
                slice[0] as f32,
                slice[1] as f32,
                slice[2] as f32,
                slice[3] as f32,
            ]
        } else if slice.len() >= 2 {
            let gray = slice[0] as f32;
            [gray, gray, gray, slice[1] as f32]
        } else {
            [1.0, 1.0, 1.0, 1.0]
        }
    }

    fn extract_quad_beziers_from_path(path: &cg::Path) -> Vec<QuadBezier> {
        let mut state = PathExtractionState::new();
        path.apply_mut(|element| {
            use cg::PathElementType as ElementType;

            match element.type_ {
                ElementType::MoveToPoint => {
                    let point = element.points()[0];
                    state.subpath_start = point;
                    state.current = point;
                }
                ElementType::AddLineToPoint => {
                    let point = element.points()[0];
                    let mid = midpoint(state.current, point);
                    state.curves.push(QuadBezier {
                        p0: [state.current.x as f32, state.current.y as f32],
                        p1: [mid.x as f32, mid.y as f32],
                        p2: [point.x as f32, point.y as f32],
                    });
                    state.current = point;
                }
                ElementType::AddQuadCurveToPoint => {
                    let points = element.points();
                    let ctrl = points[0];
                    let end = points[1];
                    state.curves.push(QuadBezier {
                        p0: [state.current.x as f32, state.current.y as f32],
                        p1: [ctrl.x as f32, ctrl.y as f32],
                        p2: [end.x as f32, end.y as f32],
                    });
                    state.current = end;
                }
                ElementType::AddCurveToPoint => {
                    let points = element.points();
                    let quadratics =
                        cubic_to_quadratic(state.current, points[0], points[1], points[2]);
                    state.curves.extend_from_slice(&quadratics);
                    state.current = points[2];
                }
                ElementType::CloseSubpath => {
                    if distance_squared(state.current, state.subpath_start) > 1.0e-10 {
                        let mid = midpoint(state.current, state.subpath_start);
                        state.curves.push(QuadBezier {
                            p0: [state.current.x as f32, state.current.y as f32],
                            p1: [mid.x as f32, mid.y as f32],
                            p2: [state.subpath_start.x as f32, state.subpath_start.y as f32],
                        });
                    }
                    state.current = state.subpath_start;
                }
            }
        });
        state.curves
    }

    fn midpoint(a: cg::Point, b: cg::Point) -> cg::Point {
        cg::Point::new((a.x + b.x) * 0.5, (a.y + b.y) * 0.5)
    }

    fn lerp_point(a: cg::Point, b: cg::Point, t: f64) -> cg::Point {
        cg::Point::new(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t)
    }

    fn distance_squared(a: cg::Point, b: cg::Point) -> f64 {
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        dx * dx + dy * dy
    }

    fn cubic_to_quadratic(
        p0: cg::Point,
        c1: cg::Point,
        c2: cg::Point,
        p3: cg::Point,
    ) -> [QuadBezier; 2] {
        let m01 = lerp_point(p0, c1, 0.5);
        let m12 = lerp_point(c1, c2, 0.5);
        let m23 = lerp_point(c2, p3, 0.5);
        let m012 = lerp_point(m01, m12, 0.5);
        let m123 = lerp_point(m12, m23, 0.5);
        let mid = lerp_point(m012, m123, 0.5);

        [
            QuadBezier {
                p0: [p0.x as f32, p0.y as f32],
                p1: [m012.x as f32, m012.y as f32],
                p2: [mid.x as f32, mid.y as f32],
            },
            QuadBezier {
                p0: [mid.x as f32, mid.y as f32],
                p1: [m123.x as f32, m123.y as f32],
                p2: [p3.x as f32, p3.y as f32],
            },
        ]
    }

    fn make_fit_transform(bounds: cg::Rect) -> [[f32; 4]; 4] {
        let width = bounds.size.width.max(1.0) as f32;
        let height = bounds.size.height.max(1.0) as f32;
        let scale = (1.9 / width).min(1.9 / height);
        let tx = -((bounds.origin.x + bounds.size.width * 0.5) as f32) * scale;
        let ty = ((bounds.origin.y + bounds.size.height * 0.5) as f32) * scale;
        [
            [scale, 0.0, 0.0, 0.0],
            [0.0, -scale, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [tx, ty, 0.0, 1.0],
        ]
    }

    fn normalize2(v: [f32; 2]) -> [f32; 2] {
        let len_sq = v[0] * v[0] + v[1] * v[1];
        if len_sq <= f32::EPSILON {
            [0.0, 0.0]
        } else {
            let inv_len = len_sq.sqrt().recip();
            [v[0] * inv_len, v[1] * inv_len]
        }
    }

    fn rgb_from_hue(mut hue: f32) -> [f32; 3] {
        hue = hue.rem_euclid(1.0);
        let r = (hue * 6.0 - 3.0).abs() - 1.0;
        let g = 2.0 - (hue * 6.0 - 2.0).abs();
        let b = 2.0 - (hue * 6.0 - 4.0).abs();
        [r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0)]
    }

    fn linearize_rgba(color: [f32; 4]) -> [f32; 4] {
        [
            srgb_to_linear(color[0]),
            srgb_to_linear(color[1]),
            srgb_to_linear(color[2]),
            color[3],
        ]
    }

    fn srgb_to_linear(x: f32) -> f32 {
        if x <= 0.04045 {
            x / 12.92
        } else {
            ((x + 0.055) / 1.055).powf(2.4)
        }
    }
}

#[cfg(target_os = "macos")]
fn main() {
    macos::main();
}

#[cfg(not(target_os = "macos"))]
fn main() {
    panic!("mtl-slug example currently supports macOS only");
}
