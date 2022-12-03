use iced_graphics::layer::Quad;
use iced_graphics::{font, Color};
use iced_graphics::{Font, Point, Size};
use iced_graphics::{Layer, Primitive, Transformation, Viewport};
use iced_native::text::Hit;

use skia_safe::gpu::DirectContext;
use skia_safe::{Canvas, Color4f, Paint, Rect, Surface};

pub struct Backend {
    context: DirectContext,
    surface: Surface,
}

impl Backend {
    pub fn new(context: DirectContext, surface: Surface) -> Self {
        Self { context, surface }
    }

    fn canvas(&mut self) -> &mut Canvas {
        self.surface.canvas()
    }

    pub fn present<T: AsRef<str>>(
        &mut self,
        primitives: &[Primitive],
        viewport: &Viewport,
        overlay_text: &[T],
    ) {
        self.surface.canvas().clear(skia_safe::Color::WHITE);
        let viewport_size = viewport.physical_size();
        let scale_factor = viewport.scale_factor() as f32;
        let projection = viewport.projection();

        let mut layers = Layer::generate(primitives, viewport);
        layers.push(Layer::overlay(overlay_text, viewport));

        for layer in layers {
            self.flush(scale_factor, projection, &layer, viewport_size.height);
        }
    }

    fn flush(
        &mut self,
        scale_factor: f32,
        _transformation: Transformation,
        layer: &Layer<'_>,
        target_height: u32,
    ) {
        let mut bounds = (layer.bounds * scale_factor).snap();

        if bounds.width < 1 || bounds.height < 1 {
            return;
        }

        bounds.height = bounds.height.min(target_height);
        self.draw_quads(&layer.quads);
        self.context.flush(None);
    }

    fn draw_quads(&mut self, quads: &Vec<Quad>) {
        for quad in quads.iter() {
            let paint = Paint::new(convert_color(quad.color), None);
            let mut stroke_paint =
                Paint::new(convert_color(quad.border_color), None);
            stroke_paint.set_stroke_width(quad.border_width);
            let left = quad.position[0];
            let top = quad.position[1];
            let rect = Rect {
                left,
                top,
                right: left + quad.size[0],
                bottom: top + quad.size[1],
            };
            self.canvas().draw_rect(rect, &paint);
            self.canvas().draw_rect(rect, &stroke_paint);
        }
        // quads.first().
    }
}

fn convert_color(iced_color: [f32; 4]) -> Color4f {
    return Color4f {
        r: iced_color[0],
        g: iced_color[1],
        b: iced_color[2],
        a: iced_color[3],
    };
}

impl iced_graphics::Backend for Backend {}

impl iced_graphics::backend::Text for Backend {
    const ICON_FONT: Font = font::ICONS;
    const CHECKMARK_ICON: char = font::CHECKMARK_ICON;
    const ARROW_DOWN_ICON: char = font::ARROW_DOWN_ICON;

    fn default_size(&self) -> u16 {
        0
    }

    fn measure(
        &self,
        _contents: &str,
        _size: f32,
        _font: Font,
        _bounds: Size,
    ) -> (f32, f32) {
        (0.0, 0.0)
    }

    fn hit_test(
        &self,
        _contents: &str,
        _size: f32,
        _font: Font,
        _bounds: Size,
        _point: Point,
        _nearest_only: bool,
    ) -> Option<Hit> {
        todo!()
    }
}
