use std::ffi::c_void;
use std::marker::PhantomData;

use skia_safe::{
    gpu::{gl::FramebufferInfo, BackendRenderTarget, SurfaceOrigin, DirectContext},
    Color as SkiaColor, ColorType, Surface, RCHandle,
};

extern crate gl_rs;

use gl::types::*;
use gl_rs as gl;

use crate::{Backend, Color, Error, Renderer, Settings, Theme, Viewport};

use iced_graphics::{compositor, Antialiasing, Size};

use iced_graphics::compositor::Information;

pub struct Compositor<Theme> {
    theme: PhantomData<Theme>,
}

impl<Theme> Compositor<Theme> {
    fn create_surface(
        fb_info: &FramebufferInfo,
        gr_context: &mut skia_safe::gpu::DirectContext,
    ) -> skia_safe::Surface {
        // let pixel_format = windowed_context.get_pixel_format();
        // let size = windowed_context.window().inner_size();
        let backend_render_target = BackendRenderTarget::new_gl(
            (
                // size.width.try_into().unwrap(),
                500, // size.height.try_into().unwrap(),
                500,
            ),
            // pixel_format.multisampling.map(|s| s.try_into().unwrap()),
            None,
            // pixel_format.stencil_bits.try_into().unwrap(),
            8,
            *fb_info,
        );
        Surface::from_backend_render_target(
            gr_context,
            &backend_render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        )
        .unwrap()
    }
}

impl<Theme> iced_graphics::window::GLCompositor for Compositor<Theme> {
    type Renderer = Renderer<Theme>;
    type Settings = Settings;

    unsafe fn new(
        _settings: Self::Settings,
        loader_function: impl FnMut(&str) -> *const c_void,
    ) -> Result<(Self, Self::Renderer), Error> {
        gl::load_with(loader_function);

        let mut gr_context =
            skia_safe::gpu::DirectContext::new_gl(None, None).unwrap();

        let fb_info = {
            let mut fboid: GLint = 0;
            unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

            FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: skia_safe::gpu::gl::Format::RGBA8.into(),
            }
        };

        let surface = Self::create_surface(&fb_info, &mut gr_context);

        let renderer = Renderer::new(Backend::new(gr_context, surface));
        Ok((
            Self {
                theme: PhantomData,

            },
            renderer,
        ))
    }

    fn sample_count(settings: &Self::Settings) -> u32 {
        0
    }
    fn resize_viewport(&mut self, physical_size: Size<u32>) {}

    fn fetch_information(&self) -> Information {
        todo!()
    }

    fn present<T: AsRef<str>>(
        &mut self,
        renderer: &mut Self::Renderer,
        viewport: &Viewport,
        _background_color: Color,
        overlay: &[T],
    ) {
        renderer.with_primitives(|backend, primitive| {
            backend.present( primitive, viewport, overlay);
        });

    }
}
