use sdl3::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    messagebox::{show_simple_message_box, MessageBoxFlag},
    pixels::Color,
    render::{create_renderer, FPoint, FRect, Vertex, WindowCanvas},
    timer::ticks,
    video::FullscreenType,
    Sdl, VideoSubsystem,
};
use sdl3_main::{app_impl, AppResult, AppResultWithState, MainThreadData};
use std::sync::Mutex;

const LOGICAL_WINDOW_SIZE: (u32, u32) = (800, 600);

/// Draws a filled circle in drawing units using the canvas's current color.
#[allow(dead_code)] // Retained as an alternative to the arrow pointer.
fn draw_circle(
    canvas: &mut WindowCanvas,
    center_x: f32,
    center_y: f32,
    radius: f32,
) -> Result<(), sdl3::Error> {
    if !radius.is_finite() || radius <= 0.0 {
        return Ok(());
    }

    const SEGMENTS: usize = 64;
    let center = Vertex {
        position: FPoint::new(center_x, center_y),
        color: canvas.draw_color().into(),
        tex_coord: FPoint::new(0.0, 0.0),
    };
    let mut vertices = [center; SEGMENTS + 1];
    let mut indices = [[0_u16; 3]; SEGMENTS];

    for i in 0..SEGMENTS {
        let angle = std::f32::consts::TAU * i as f32 / SEGMENTS as f32;
        let (sin, cos) = angle.sin_cos();
        vertices[i + 1].position = FPoint::new(center_x + radius * cos, center_y + radius * sin);
        // Each triangle joins the center to two neighbors, wrapping at the end.
        indices[i] = [0, (i + 1) as u16, ((i + 1) % SEGMENTS + 1) as u16];
    }

    canvas.render_geometry(&vertices, None, indices.as_slice())
}

/// Draws a filled arrow pointing up and left, with its tip at (tip_x, tip_y).
/// `size` is its height in drawing units; the canvas supplies the color.
fn draw_mouse_pointer(
    canvas: &mut WindowCanvas,
    tip_x: f32,
    tip_y: f32,
    size: f32,
) -> Result<(), sdl3::Error> {
    if !size.is_finite() || size <= 0.0 {
        return Ok(());
    }

    let scale = size / 24.0;
    let color = canvas.draw_color().into();
    let vertices = [
        (0.0, 0.0), // Tip / mouse hotspot.
        (16.0, 13.0),
        (9.0, 13.0),
        (13.0, 22.0),
        (9.0, 24.0),
        (5.0, 15.0),
        (0.0, 20.0),
    ]
    .map(|(x, y)| Vertex {
        position: FPoint::new(tip_x + x * scale, tip_y + y * scale),
        color,
        tex_coord: FPoint::new(0.0, 0.0),
    });
    // Three triangles fill the arrowhead; two fill its inclined shaft.
    let indices = [[0_u16, 1, 2], [0, 2, 5], [0, 5, 6], [2, 3, 4], [2, 4, 5]];

    canvas.render_geometry(&vertices, None, indices.as_slice())
}

fn window_size_for_scale(scale: f32) -> (u32, u32) {
    (
        (LOGICAL_WINDOW_SIZE.0 as f32 * scale).round().max(1.0) as u32,
        (LOGICAL_WINDOW_SIZE.1 as f32 * scale).round().max(1.0) as u32,
    )
}

struct App {
    main: MainThreadData<MainData>,
    mx: f32,
    my: f32,
    last_repaint_mouse: Option<(f32, f32)>,
    needs_redraw: bool,
    requested_window_size: (u32, u32),
}

struct MainData {
    canvas: WindowCanvas,
    _video: VideoSubsystem,
    _sdl: Sdl,
}

#[app_impl]
impl App {
    fn new() -> Result<Self, Box<dyn ::core::error::Error>> {
        // Limit repaints to 25 per second by letting SDL pace app_iterate().
        // Some platforms ignore this hint; see SDL_HINT_MAIN_CALLBACK_RATE.
        sdl3::hint::set("SDL_MAIN_CALLBACK_RATE", "60");

        let sdl = sdl3::init()?;
        let video = sdl.video()?;
        // Before a window exists, the display content scale converts logical
        // dimensions to window units. Create it on the display we queried.
        let initial_scale = video.get_primary_display()?.get_content_scale()?;
        let initial_window_size = window_size_for_scale(initial_scale);
        let window = video
            .window(
                "Hello callbacks api!",
                initial_window_size.0,
                initial_window_size.1,
            )
            .position_centered()
            .high_pixel_density()
            .build()?;
        let canvas = create_renderer(window, None)?;
        Ok(Self {
            main: MainThreadData::assert_new(MainData {
                canvas,
                _video: video,
                _sdl: sdl,
            }),
            mx: 0.0,
            my: 0.0,
            last_repaint_mouse: None,
            needs_redraw: true,
            requested_window_size: initial_window_size,
        })
    }

    fn app_init() -> AppResultWithState<Box<Mutex<Self>>> {
        match Self::new() {
            Ok(app) => AppResultWithState::Continue(Box::new(Mutex::new(app))),
            Err(err) => {
                let error_msg = format!("Error initializing SDL: {err:?}");
                eprintln!("{error_msg}");
                let _ = show_simple_message_box(MessageBoxFlag::ERROR, "Error!", &error_msg, None);
                AppResultWithState::Failure(None)
            }
        }
    }

    fn app_iterate(&mut self) {
        let main = self.main.assert_get_mut();
        let canvas = &mut main.canvas;

        // SetWindowSize uses window units, not framebuffer pixels. Dividing by
        // pixel density avoids scaling twice on macOS/Wayland high-DPI windows.
        let window = canvas.window_mut();
        let display_scale = window.display_scale();
        let window_scale = display_scale / window.pixel_density();
        if window_scale.is_finite()
            && window_scale > 0.0
            && !window.is_minimized()
            && !window.is_maximized()
            && window.fullscreen_state() == FullscreenType::Off
        {
            // Always derive from the logical size to avoid rounding drift when
            // moving back and forth between differently scaled monitors.
            let size = window_size_for_scale(window_scale);
            if size != self.requested_window_size {
                if let Err(err) = window.set_size(size.0, size.1) {
                    eprintln!("Error updating window size for display scale: {err}");
                    return;
                }
                // Resizing may be asynchronous or constrained by the desktop.
                // Remember the request so we don't repeat it every callback.
                self.requested_window_size = size;
                self.needs_redraw = true;
            }
        }

        // sdl3 0.20 does not expose SDL_EVENT_WINDOW_DISPLAY_SCALE_CHANGED.
        // Check before the idle guard so monitor/settings changes redraw even
        // when the mouse is stationary or outside the window.
        if display_scale.is_finite()
            && display_scale > 0.0
            && canvas.scale() != (display_scale, display_scale)
        {
            if let Err(err) = canvas.set_scale(display_scale, display_scale) {
                eprintln!("Error updating display scale: {err}");
                return;
            }
            self.needs_redraw = true;
        }

        let window = canvas.window();
        let (width, height) = window.size();
        // Keep mouse coordinates in window units for this bounds check; the
        // renderer scale only affects drawing. Captured motion may be outside.
        let mouse_in_window = window.has_mouse_focus()
            && (0.0..width as f32).contains(&self.mx)
            && (0.0..height as f32).contains(&self.my);
        let mouse_position = (self.mx, self.my);
        if !self.needs_redraw
            && (!mouse_in_window || self.last_repaint_mouse == Some(mouse_position))
        {
            return;
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(Color::WHITE);

        // Convert the pixel size to drawing units so centering works at any DPI.
        let (pixel_width, pixel_height) = canvas.window().size_in_pixels();
        let (scale_x, scale_y) = canvas.scale();
        let rectangle_width = 200.0;
        let rectangle_height = 120.0;
        let rectangle = FRect::new(
            (pixel_width as f32 / scale_x - rectangle_width) / 2.0,
            (pixel_height as f32 / scale_y - rectangle_height) / 2.0,
            rectangle_width,
            rectangle_height,
        );
        let _ = canvas.draw_rect(rectangle);

        let _ = canvas.draw_debug_text(&format!("Callbacks running for {} ms", ticks()), (4, 4));
        let _ = canvas.draw_debug_text(&format!("Mouse x: {}", self.mx), (4, 20));
        let _ = canvas.draw_debug_text(&format!("      y: {}", self.my), (4, 28));
        let _ = canvas.draw_debug_text(&format!("Display scale: {:.2}", display_scale), (4, 36));
        let _ = canvas.draw_debug_text(
            &format!(
                "Window size: {}x{} ({}x{} pixels)",
                width, height, pixel_width, pixel_height
            ),
            (4, 48),
        );
        canvas.set_draw_color(Color::YELLOW);
        // Mouse events use window units; geometry uses scaled drawing units.
        let pixel_density = canvas.window().pixel_density();
        if let Err(err) = draw_mouse_pointer(
            canvas,
            self.mx * pixel_density / scale_x,
            self.my * pixel_density / scale_y,
            20.4,
        ) {
            eprintln!("Error drawing mouse pointer: {err}");
        }

        canvas.present();
        self.last_repaint_mouse = Some(mouse_position);
        self.needs_redraw = false;
    }

    // event can also be taken by value or by mut reference
    fn app_event(&mut self, event: &Event) -> AppResult {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => AppResult::Success,

            Event::MouseMotion { x, y, .. } => {
                self.mx = *x;
                self.my = *y;
                AppResult::Continue
            }

            Event::Window {
                win_event:
                    WindowEvent::Shown
                    | WindowEvent::Exposed
                    | WindowEvent::Restored
                    | WindowEvent::Resized(_, _)
                    | WindowEvent::PixelSizeChanged(_, _)
                    | WindowEvent::DisplayChanged(_),
                ..
            } => {
                // SDL's renderer tracks the window's pixel size automatically.
                self.needs_redraw = true;
                AppResult::Continue
            }

            _ => AppResult::Continue,
        }
    }

    // app_quit is optional if dropping App does sufficient cleanup
}
