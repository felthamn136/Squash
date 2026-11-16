use macroquad::prelude::*;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const LEFT_BAR_WIDTH: f32 = 12.0;
const TOP_DASH_WIDTH: f32 = 28.0;
const TOP_DASH_GAP: f32 = 18.0;
const TOP_DASH_HEIGHT: f32 = 12.0;
const SCANLINE_SPACING: f32 = 4.0;
const SCANLINE_COLOR: Color = Color::new(0.055, 0.055, 0.055, 1.0);

fn window_conf() -> Conf {
    Conf {
        window_title: "Squash '76".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);

        let mut y = 2.0;
        while y < screen_height() {
            draw_rectangle(0.0, y, screen_width(), 1.0, SCANLINE_COLOR);
            y += SCANLINE_SPACING;
        }

        draw_rectangle(0.0, 0.0, LEFT_BAR_WIDTH, screen_height(), WHITE);
        let mut x = 0.0;
        while x < screen_width() {
            draw_rectangle(x, 0.0, TOP_DASH_WIDTH, TOP_DASH_HEIGHT, WHITE);
            x += TOP_DASH_WIDTH + TOP_DASH_GAP;
        }
        let bottom_y = screen_height() - TOP_DASH_HEIGHT;
        let mut x = 0.0;
        while x < screen_width() {
            draw_rectangle(x, bottom_y, TOP_DASH_WIDTH, TOP_DASH_HEIGHT, WHITE);
            x += TOP_DASH_WIDTH + TOP_DASH_GAP;
        }
        next_frame().await;
    }
}
