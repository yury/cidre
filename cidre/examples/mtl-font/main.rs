// port of https://github.com/evanw/theta/blob/master/src/core/font.sk

use std::cmp::min;

use cidre::{blocks, cf, cg, ct, mtl, UniChar};

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
        self.countour_count += 1;
        let x = x.into();
        let y = y.into();
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
    use cg::PathElementType::*;
    match element.type_ {
        MoveToPoint => {
            let point = element.points()[0];
            compiler.move_to(point.x as f32, point.y as f32);
        }
        AddLineToPoint => {
            let point = element.points()[0];
            compiler.line_to(point.x as f32, point.y as f32);
        }
        AddQuadCurveToPoint => {
            let points = element.points();
            let c = points[0];
            let p = points[1];
            compiler.curve_to(c.x as f32, c.y as f32, p.x as f32, p.y as f32);
        }
        AddCurveToPoint => panic!("unexpected curve to"),
        CloseSubpath => compiler.close(),
    }
}

fn main() {
    let font = ct::Font::with_name_size(cf::String::from_str("Verdana").as_ref(), 28.0);
    let utf16 = "abcdef$@".encode_utf16().collect::<Vec<u16>>();
    let mut glyphs = vec![cg::Glyph::new(0); utf16.len()];
    font.glyphs_for_characters(&utf16, &mut glyphs).unwrap();
    let mut compiler = GlyphCompiler::default();
    for g in glyphs {
        if let Some(path) = font.path_for_glyph(g, None) {
            let gg = Glyph::default();
            compiler.begin(gg);
            path.apply(&mut compiler, apply);
            compiler.end();
            eprintln!(
                "{:?} {:?}",
                compiler.vertices.len(),
                compiler.vertices.len() / 4
            );
        } else {
            eprintln!("no path for {:?}", g);
        }
    }
}
