use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Hello macroquad!".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: true,
        high_dpi: true,
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
        draw_circle(mouse_x, mouse_y, 6.0, ORANGE);
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
