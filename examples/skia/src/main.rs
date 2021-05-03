use std::convert::TryInto;
use skia_safe::{
    gpu::{gl::FramebufferInfo, BackendRenderTarget, SurfaceOrigin},
    ColorType, Surface, Paint, Color, PaintStyle
};

use dri::kms::{drm_screen_width, drm_screen_height, init, swap_buffers};
use dri::gl::*;

fn main() {
    unsafe{init()};

    let mut gr_context = skia_safe::gpu::Context::new_gl(None, None)
        .expect("failed to create skia gl context");

    let fb_info = {
        let mut fboid: GLint = 0;
        unsafe { glGetIntegerv(GL_FRAMEBUFFER_BINDING, &mut fboid) };

        FramebufferInfo {
            fboid: fboid.try_into().unwrap(),
            format: skia_safe::gpu::gl::Format::RGBA8.into()
        }
    };

    fn create_surface(
        fb_info: FramebufferInfo,
        gr_context: &mut skia_safe::gpu::Context,
    ) -> skia_safe::Surface {
        let backend_render_target = BackendRenderTarget::new_gl(
            (unsafe{drm_screen_width()}, unsafe{drm_screen_height()}),
            None, 0,
            fb_info,
        );
        Surface::from_backend_render_target(
            gr_context,
            &backend_render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        ).unwrap()
    }
    let mut surface = create_surface(fb_info, &mut gr_context);
    use skia_safe::Path;

    let canvas = surface.canvas();
    let mut t = 0f32;
    let mut animation = skia_safe::animation::Animation::from_data(include_bytes!("../small_lottie.json")).unwrap();
    let duration = animation.num_frames();
    loop {
        t += 0.1f32;
        let y_shift = 200. * t.cos();
        let x_shift = 100. * t.sin();
        let path = Path::rect(skia_safe::Rect::new(x_shift + 50., y_shift + 1000., x_shift + 500., 50. + y_shift), None);
        canvas.clear(Color::BLACK);
        let mut paint = Paint::default();
        paint.set_color(Color::WHITE);
        paint.set_anti_alias(true);
        paint.set_stroke_width(10.);
        paint.set_style(PaintStyle::Stroke);
        canvas.draw_path(&path, &paint);
        let frame = 10. * t as f64 % duration;
        animation.seek_frame::<()>(frame);
        animation.render(canvas, skia_safe::Rect::from_xywh(y_shift, x_shift, 500., 500.));
        canvas.flush();
        unsafe{swap_buffers();}

    }
}
