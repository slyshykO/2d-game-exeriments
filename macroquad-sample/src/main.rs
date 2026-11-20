use macroquad::prelude::*;

/// Draws a filled arrow pointing up and left, with its tip at (tip_x, tip_y).
/// `size` is the arrow's height in drawing units.
fn draw_mouse_pointer(tip_x: f32, tip_y: f32, size: f32, color: Color) {
    if !size.is_finite() || size <= 0.0 {
        return;
    }

    let scale = size / 24.0;
    let vertices = [
        (0.0, 0.0), // Tip / mouse hotspot.
        (16.0, 13.0),
        (9.0, 13.0),
        (13.0, 22.0),
        (9.0, 24.0),
        (5.0, 15.0),
        (0.0, 20.0),
    ]
    .map(|(x, y)| vec2(tip_x + x * scale, tip_y + y * scale));

    // Three triangles fill the arrowhead; two fill its inclined shaft.
    for [a, b, c] in [[0, 1, 2], [0, 2, 5], [0, 5, 6], [2, 3, 4], [2, 4, 5]] {
        draw_triangle(vertices[a], vertices[b], vertices[c], color);
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Hello macroquad!".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: true,
        high_dpi: true,
        platform: miniquad::conf::Platform {
            swap_interval: Some(0),
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        clear_background(BLACK);

        draw_rectangle_lines(
            (screen_width() - 200.0) / 2.0,
            (screen_height() - 120.0) / 2.0,
            200.0,
            120.0,
            2.0,
            WHITE,
        );

        let (mouse_x, mouse_y) = mouse_position();
        draw_mouse_pointer(mouse_x, mouse_y, 20.4, ORANGE);
        draw_text(
            &format!("Mouse: ({mouse_x:.0}, {mouse_y:.0})"),
            12.0,
            24.0,
            24.0,
            WHITE,
        );
        draw_text("Move the mouse. Esc to quit.", 12.0, 48.0, 24.0, GRAY);

        next_frame().await;
    }
}
