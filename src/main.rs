use macroquad::prelude::*;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const GAME_WIDTH: f32 = WINDOW_WIDTH as f32;
const GAME_HEIGHT: f32 = WINDOW_HEIGHT as f32;
const LEFT_BAR_WIDTH: f32 = 12.0;
const TOP_DASH_WIDTH: f32 = 28.0;
const TOP_DASH_GAP: f32 = 18.0;
const TOP_DASH_HEIGHT: f32 = 12.0;
const RIGHT_MARKER_OFFSET: f32 = 133.0;
const RIGHT_MARKER_HEIGHT: f32 = 82.0;
const RIGHT_MARKER_WIDTH: f32 = 10.0;
const PADDLE_COLUMN_X: f32 = WINDOW_WIDTH as f32 - RIGHT_MARKER_OFFSET;
const SECOND_PADDLE_WIDTH: f32 = RIGHT_MARKER_WIDTH;
const PADDLE_SPEED: f32 = 300.0;
const BALL_SIZE: f32 = 14.0;
const BALL_HORIZONTAL_SPEED: f32 = 300.0;
const BALL_VERTICAL_SPEED: f32 = 200.0;
const RALLY_SPEEDUP: f32 = 1.04;
const SLICE_SPEEDUP: f32 = 1.10;
const MAX_BALL_SPEED: f32 = 620.0;
const MAX_BOUNCE_ANGLE: f32 = 0.9;
const CURVE_ACCELERATION: f32 = 600.0;
const SCANLINE_SPACING: f32 = 4.0;
const PHOSPHOR: Color = Color::new(0.82, 0.54, 0.18, 1.0);
const SECOND_PADDLE_PHOSPHOR: Color = Color::new(0.9, 0.64, 0.32, 1.0);
const SCANLINE_COLOR: Color = Color::new(0.045, 0.028, 0.009, 1.0);
const WALL_SCANLINE_COLOR: Color = Color::new(0.62, 0.42, 0.18, 1.0);
const PADDLE_SCANLINE_COLOR: Color = Color::new(0.58, 0.39, 0.14, 1.0);
const SECOND_PADDLE_SCANLINE_COLOR: Color = Color::new(0.7, 0.52, 0.28, 1.0);
const GAME_SPEEDS: [f32; 3] = [0.75, 1.0, 1.35];
const GAME_SPEED_LABELS: [&str; 3] = ["SLOW", "NORMAL", "FAST"];
const PADDLE_SIZES: [f32; 3] = [56.0, RIGHT_MARKER_HEIGHT, 112.0];
const PADDLE_SIZE_LABELS: [&str; 3] = ["SMALL", "NORMAL", "LARGE"];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Screen {
    Attraction,
    Playing,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GameMode {
    OnePlayer,
    TwoPlayer,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Player {
    One,
    Two,
}

impl Player {
    fn opponent(self) -> Self {
        match self {
            Self::One => Self::Two,
            Self::Two => Self::One,
        }
    }

    fn score_index(self) -> usize {
        match self {
            Self::One => 0,
            Self::Two => 1,
        }
    }
}

struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    spin: f32,
    speed_scale: f32,
    moving: bool,
    missed_by: Option<Player>,
    trail: Vec<Vec2>,
}

impl Ball {
    fn new() -> Self {
        Self {
            x: WINDOW_WIDTH as f32 / 2.0,
            y: WINDOW_HEIGHT as f32 / 2.0,
            vx: 0.0,
            vy: 0.0,
            spin: 0.0,
            speed_scale: 1.0,
            moving: false,
            missed_by: None,
            trail: Vec::new(),
        }
    }

    fn serve(&mut self, speed_scale: f32) {
        self.x = WINDOW_WIDTH as f32 / 2.0;
        self.y = WINDOW_HEIGHT as f32 / 2.0;
        self.vx = -BALL_HORIZONTAL_SPEED * speed_scale;
        self.vy = BALL_VERTICAL_SPEED * speed_scale;
        self.spin = 0.0;
        self.speed_scale = speed_scale;
        self.moving = true;
        self.missed_by = None;
        self.trail.clear();
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn return_from_paddle(&mut self, impact: f32, slice: bool, curve: bool, toward_right: bool) {
        let speedup = if slice { SLICE_SPEEDUP } else { RALLY_SPEEDUP };
        let speed = (self.vx.hypot(self.vy) * speedup).min(MAX_BALL_SPEED * self.speed_scale);
        let angle_scale = if slice { 0.18 } else { MAX_BOUNCE_ANGLE };
        let angle = impact.clamp(-1.0, 1.0) * angle_scale;
        self.vx = if toward_right { 1.0 } else { -1.0 } * speed * angle.cos();
        self.vy = speed * angle.sin();
        self.spin = if curve && !slice {
            if impact < 0.0 {
                -CURVE_ACCELERATION
            } else {
                CURVE_ACCELERATION
            }
        } else {
            0.0
        };
    }

    fn update(
        &mut self,
        dt: f32,
        second_paddle_y: f32,
        right_paddle_y: f32,
        paddle_height: f32,
        slice: bool,
        curve: bool,
    ) -> Option<Player> {
        if !self.moving {
            return None;
        }

        let radius = BALL_SIZE / 2.0;
        self.trail.insert(0, vec2(self.x, self.y));
        self.trail.truncate(8);
        self.vy += self.spin * dt;
        self.spin *= (-0.8 * dt).exp();
        let speed = self.vx.hypot(self.vy);
        let max_speed = MAX_BALL_SPEED * self.speed_scale;
        if speed > max_speed {
            let scale = max_speed / speed;
            self.vx *= scale;
            self.vy *= scale;
        }
        let previous_x = self.x;
        self.x += self.vx * dt;
        self.y += self.vy * dt;

        if self.x - radius <= LEFT_BAR_WIDTH {
            self.x = LEFT_BAR_WIDTH + radius;
            self.vx = self.vx.abs();
        }
        if self.y - radius <= TOP_DASH_HEIGHT {
            self.y = TOP_DASH_HEIGHT + radius;
            self.vy = self.vy.abs();
            self.spin = -self.spin * 0.8;
        } else if self.y + radius >= WINDOW_HEIGHT as f32 - TOP_DASH_HEIGHT {
            self.y = WINDOW_HEIGHT as f32 - TOP_DASH_HEIGHT - radius;
            self.vy = -self.vy.abs();
            self.spin = -self.spin * 0.8;
        }

        let reached_paddles = self.vx > 0.0
            && previous_x + radius < PADDLE_COLUMN_X
            && self.x + radius >= PADDLE_COLUMN_X;
        if reached_paddles {
            let second_impact = (self.y - second_paddle_y) / (paddle_height / 2.0);
            let first_impact = (self.y - right_paddle_y) / (paddle_height / 2.0);
            let (missed_player, impact) = if second_impact.abs() <= first_impact.abs() {
                (Player::Two, second_impact)
            } else {
                (Player::One, first_impact)
            };
            if impact.abs() <= 1.0 {
                self.x = PADDLE_COLUMN_X - radius;
                self.return_from_paddle(impact, slice, curve, false);
            } else {
                self.missed_by = Some(missed_player);
            }
        } else if self.vx > 0.0
            && self.missed_by.is_none()
            && self.x - radius > PADDLE_COLUMN_X + SECOND_PADDLE_WIDTH
        {
            let second_impact = (self.y - second_paddle_y).abs();
            let first_impact = (self.y - right_paddle_y).abs();
            self.missed_by = Some(if second_impact <= first_impact {
                Player::Two
            } else {
                Player::One
            });
        }

        if self.x - radius > GAME_WIDTH {
            if let Some(missed_player) = self.missed_by {
                let scoring_player = missed_player.opponent();
                self.reset();
                return Some(scoring_player);
            }
        }
        None
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Squash '76".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: true,
        ..Default::default()
    }
}

fn fitted_viewport(screen_width: f32, screen_height: f32) -> (i32, i32, i32, i32) {
    let scale = (screen_width / GAME_WIDTH).min(screen_height / GAME_HEIGHT);
    let width = (GAME_WIDTH * scale).round().max(1.0) as i32;
    let height = (GAME_HEIGHT * scale).round().max(1.0) as i32;
    let x = ((screen_width.round() as i32) - width) / 2;
    let y = ((screen_height.round() as i32) - height) / 2;
    (x, y, width, height)
}

fn paddle_direction(up_pressed: bool, down_pressed: bool) -> f32 {
    f32::from(up_pressed) - f32::from(down_pressed)
}

fn set_game_camera() -> (i32, i32, i32, i32) {
    let viewport = fitted_viewport(screen_width(), screen_height());
    let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, GAME_WIDTH, GAME_HEIGHT));
    camera.viewport = Some(viewport);
    set_camera(&camera);
    viewport
}

fn draw_ball(ball: &Ball) {
    for (index, position) in ball.trail.iter().enumerate().rev() {
        let alpha = 0.06 * (1.0 - index as f32 / ball.trail.len().max(1) as f32);
        draw_rectangle(
            position.x - BALL_SIZE,
            position.y - BALL_SIZE,
            BALL_SIZE * 2.0,
            BALL_SIZE * 2.0,
            Color::new(PHOSPHOR.r, PHOSPHOR.g, PHOSPHOR.b, alpha),
        );
    }

    for (size, alpha) in [(34.0, 0.025), (24.0, 0.06), (18.0, 0.12)] {
        draw_rectangle(
            ball.x - size / 2.0,
            ball.y - size / 2.0,
            size,
            size,
            Color::new(PHOSPHOR.r, PHOSPHOR.g, PHOSPHOR.b, alpha),
        );
    }
    draw_rectangle(
        ball.x - BALL_SIZE / 2.0,
        ball.y - BALL_SIZE / 2.0,
        BALL_SIZE,
        BALL_SIZE,
        PHOSPHOR,
    );
}

fn with_alpha(mut color: Color, opacity: f32) -> Color {
    color.a *= opacity;
    color
}

fn draw_scanlined_rect(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    fill_color: Color,
    line_color: Color,
) {
    draw_rectangle(x, y, width, height, fill_color);
    let mut line_y = y + 2.0;
    while line_y < y + height {
        draw_rectangle(x, line_y, width, 1.0, line_color);
        line_y += SCANLINE_SPACING;
    }
}

fn draw_game_field(
    second_paddle_y: f32,
    right_paddle_y: f32,
    paddle_height: f32,
    ball: &Ball,
    opacity: f32,
) {
    let mut y = 2.0;
    while y < GAME_HEIGHT {
        draw_rectangle(0.0, y, GAME_WIDTH, 1.0, with_alpha(SCANLINE_COLOR, opacity));
        y += SCANLINE_SPACING;
    }

    draw_scanlined_rect(
        0.0,
        0.0,
        LEFT_BAR_WIDTH,
        GAME_HEIGHT,
        with_alpha(PHOSPHOR, opacity),
        with_alpha(WALL_SCANLINE_COLOR, opacity),
    );
    let mut x = 0.0;
    while x < GAME_WIDTH {
        draw_scanlined_rect(
            x,
            0.0,
            TOP_DASH_WIDTH,
            TOP_DASH_HEIGHT,
            with_alpha(PHOSPHOR, opacity),
            with_alpha(WALL_SCANLINE_COLOR, opacity),
        );
        x += TOP_DASH_WIDTH + TOP_DASH_GAP;
    }
    let bottom_y = GAME_HEIGHT - TOP_DASH_HEIGHT;
    let mut x = 0.0;
    while x < GAME_WIDTH {
        draw_scanlined_rect(
            x,
            bottom_y,
            TOP_DASH_WIDTH,
            TOP_DASH_HEIGHT,
            with_alpha(PHOSPHOR, opacity),
            with_alpha(WALL_SCANLINE_COLOR, opacity),
        );
        x += TOP_DASH_WIDTH + TOP_DASH_GAP;
    }
    draw_scanlined_rect(
        PADDLE_COLUMN_X,
        second_paddle_y - paddle_height / 2.0,
        SECOND_PADDLE_WIDTH,
        paddle_height,
        with_alpha(SECOND_PADDLE_PHOSPHOR, opacity),
        with_alpha(SECOND_PADDLE_SCANLINE_COLOR, opacity),
    );
    draw_scanlined_rect(
        PADDLE_COLUMN_X,
        right_paddle_y - paddle_height / 2.0,
        RIGHT_MARKER_WIDTH,
        paddle_height,
        with_alpha(PHOSPHOR, opacity),
        with_alpha(PADDLE_SCANLINE_COLOR, opacity),
    );
    if opacity < 1.0 {
        draw_rectangle(
            ball.x - BALL_SIZE / 2.0,
            ball.y - BALL_SIZE / 2.0,
            BALL_SIZE,
            BALL_SIZE,
            with_alpha(PHOSPHOR, opacity),
        );
    } else {
        draw_ball(ball);
    }
}

fn draw_centered_text(text: &str, center_x: f32, baseline_y: f32, font_size: f32, color: Color) {
    let dimensions = measure_text(text, None, font_size.max(1.0) as u16, 1.0);
    draw_text(
        text,
        center_x - dimensions.width / 2.0,
        baseline_y,
        font_size.max(1.0),
        color,
    );
}

fn menu_mode_rects() -> [Rect; 2] {
    [
        Rect::new(205.0, 204.0, 185.0, 52.0),
        Rect::new(410.0, 204.0, 185.0, 52.0),
    ]
}

fn menu_option_rects() -> [Rect; 3] {
    [
        Rect::new(250.0, 306.0, 90.0, 48.0),
        Rect::new(355.0, 306.0, 90.0, 48.0),
        Rect::new(460.0, 306.0, 90.0, 48.0),
    ]
}

fn menu_size_rects() -> [Rect; 3] {
    [
        Rect::new(250.0, 407.0, 90.0, 48.0),
        Rect::new(355.0, 407.0, 90.0, 48.0),
        Rect::new(460.0, 407.0, 90.0, 48.0),
    ]
}

fn start_button_rect() -> Rect {
    Rect::new(300.0, 493.0, 200.0, 58.0)
}

fn draw_menu_button(rect: Rect, label: &str, selected: bool, viewport: (i32, i32, i32, i32)) {
    let scale = viewport.2 as f32 / GAME_WIDTH;
    let rect = Rect::new(
        viewport.0 as f32 + rect.x * scale,
        viewport.1 as f32 + rect.y * scale,
        rect.w * scale,
        rect.h * scale,
    );
    let color = if selected {
        PHOSPHOR
    } else {
        SECOND_PADDLE_PHOSPHOR
    };
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.02, 0.015, 0.008, 0.88),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if selected { 2.0 } else { 1.0 },
        with_alpha(color, if selected { 0.95 } else { 0.45 }),
    );
    draw_centered_text(
        label,
        rect.x + rect.w / 2.0,
        rect.y + rect.h / 2.0 + 7.0,
        19.0 * scale,
        with_alpha(color, if selected { 1.0 } else { 0.7 }),
    );
}

fn draw_attraction_screen(
    mode: GameMode,
    speed_index: usize,
    size_index: usize,
    viewport: (i32, i32, i32, i32),
) {
    let scale = viewport.2 as f32 / GAME_WIDTH;
    let center_x = viewport.0 as f32 + GAME_WIDTH / 2.0 * scale;
    let top = viewport.1 as f32;
    draw_centered_text(
        "SQUASH '76",
        center_x,
        top + 117.0 * scale,
        58.0 * scale,
        PHOSPHOR,
    );
    draw_centered_text(
        "SELECT YOUR GAME",
        center_x,
        top + 154.0 * scale,
        17.0 * scale,
        SECOND_PADDLE_PHOSPHOR,
    );

    draw_centered_text(
        "PLAYERS",
        center_x,
        top + 194.0 * scale,
        15.0 * scale,
        PHOSPHOR,
    );
    let mode_rects = menu_mode_rects();
    draw_menu_button(
        mode_rects[0],
        "1 PLAYER",
        mode == GameMode::OnePlayer,
        viewport,
    );
    draw_menu_button(
        mode_rects[1],
        "2 PLAYER",
        mode == GameMode::TwoPlayer,
        viewport,
    );

    draw_centered_text(
        "GAME SPEED",
        center_x,
        top + 295.0 * scale,
        15.0 * scale,
        PHOSPHOR,
    );
    for (index, rect) in menu_option_rects().into_iter().enumerate() {
        draw_menu_button(
            rect,
            GAME_SPEED_LABELS[index],
            speed_index == index,
            viewport,
        );
    }

    draw_centered_text(
        "PADDLE SIZE",
        center_x,
        top + 396.0 * scale,
        15.0 * scale,
        PHOSPHOR,
    );
    for (index, rect) in menu_size_rects().into_iter().enumerate() {
        draw_menu_button(
            rect,
            PADDLE_SIZE_LABELS[index],
            size_index == index,
            viewport,
        );
    }

    let start = start_button_rect();
    draw_menu_button(start, "START GAME", true, viewport);
}

fn mouse_in_game(viewport: (i32, i32, i32, i32)) -> Option<Vec2> {
    let (x, y, width, height) = viewport;
    let mouse = mouse_position();
    if mouse.0 < x as f32
        || mouse.1 < y as f32
        || mouse.0 >= (x + width) as f32
        || mouse.1 >= (y + height) as f32
    {
        return None;
    }

    Some(vec2(
        (mouse.0 - x as f32) * GAME_WIDTH / width as f32,
        (mouse.1 - y as f32) * GAME_HEIGHT / height as f32,
    ))
}

fn draw_scoreboard(scores: [u32; 2], mode: GameMode, viewport: (i32, i32, i32, i32)) {
    let scale = viewport.2 as f32 / GAME_WIDTH;
    let center_x = viewport.0 as f32 + viewport.2 as f32 / 2.0;
    let top = viewport.1 as f32;
    draw_centered_text(
        if mode == GameMode::OnePlayer {
            "CPU"
        } else {
            "PLAYER 2"
        },
        center_x - 105.0 * scale,
        top + 36.0 * scale,
        26.0 * scale,
        SECOND_PADDLE_PHOSPHOR,
    );
    draw_centered_text(
        &scores[1].to_string(),
        center_x - 105.0 * scale,
        top + 108.0 * scale,
        64.0 * scale,
        SECOND_PADDLE_PHOSPHOR,
    );
    draw_centered_text(
        "PLAYER 1",
        center_x + 105.0 * scale,
        top + 36.0 * scale,
        26.0 * scale,
        PHOSPHOR,
    );
    draw_centered_text(
        &scores[0].to_string(),
        center_x + 105.0 * scale,
        top + 108.0 * scale,
        64.0 * scale,
        PHOSPHOR,
    );
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut second_paddle_y = GAME_HEIGHT / 2.0 - RIGHT_MARKER_HEIGHT;
    let mut right_paddle_y = GAME_HEIGHT / 2.0;
    let mut ball = Ball::new();
    let mut scores = [0; 2];
    let mut screen = Screen::Attraction;
    let mut mode = GameMode::OnePlayer;
    let mut speed_index = 1;
    let mut size_index = 1;

    loop {
        set_default_camera();
        clear_background(BLACK);
        let viewport = set_game_camera();
        let dt = get_frame_time().min(0.04);
        let paddle_height = PADDLE_SIZES[size_index];

        if screen == Screen::Attraction {
            let mouse = mouse_in_game(viewport);
            if is_mouse_button_pressed(MouseButton::Left) {
                if let Some(mouse) = mouse {
                    for (index, rect) in menu_mode_rects().into_iter().enumerate() {
                        if rect.contains(mouse) {
                            mode = if index == 0 {
                                GameMode::OnePlayer
                            } else {
                                GameMode::TwoPlayer
                            };
                        }
                    }
                    for (index, rect) in menu_option_rects().into_iter().enumerate() {
                        if rect.contains(mouse) {
                            speed_index = index;
                        }
                    }
                    for (index, rect) in menu_size_rects().into_iter().enumerate() {
                        if rect.contains(mouse) {
                            size_index = index;
                        }
                    }
                    if start_button_rect().contains(mouse) {
                        screen = Screen::Playing;
                        ball.reset();
                        ball.serve(GAME_SPEEDS[speed_index]);
                        scores = [0; 2];
                    }
                }
            }
            draw_game_field(
                GAME_HEIGHT / 2.0 - paddle_height,
                GAME_HEIGHT / 2.0,
                paddle_height,
                &Ball::new(),
                0.2,
            );
            set_default_camera();
            draw_attraction_screen(mode, speed_index, size_index, viewport);
        } else {
            let top_bound = TOP_DASH_HEIGHT + paddle_height / 2.0;
            let bottom_bound = GAME_HEIGHT - TOP_DASH_HEIGHT - paddle_height / 2.0;
            let right_direction =
                paddle_direction(is_key_down(KeyCode::W), is_key_down(KeyCode::S));
            right_paddle_y = (right_paddle_y + right_direction * PADDLE_SPEED * dt)
                .clamp(top_bound, bottom_bound);
            let second_direction = if mode == GameMode::OnePlayer {
                if ball.y > second_paddle_y + 8.0 {
                    1.0
                } else if ball.y < second_paddle_y - 8.0 {
                    -1.0
                } else {
                    0.0
                }
            } else {
                paddle_direction(is_key_down(KeyCode::Up), is_key_down(KeyCode::Down))
            };
            second_paddle_y = (second_paddle_y + second_direction * PADDLE_SPEED * dt)
                .clamp(top_bound, bottom_bound);
            if is_key_pressed(KeyCode::Space) && !ball.moving {
                ball.serve(GAME_SPEEDS[speed_index]);
            }
            if is_key_pressed(KeyCode::Escape) {
                screen = Screen::Attraction;
                ball.reset();
            }
            if let Some(player) = ball.update(
                dt,
                second_paddle_y,
                right_paddle_y,
                paddle_height,
                is_key_down(KeyCode::A),
                is_key_down(KeyCode::D),
            ) {
                scores[player.score_index()] += 1;
            }
            draw_game_field(second_paddle_y, right_paddle_y, paddle_height, &ball, 1.0);
        }
        set_default_camera();
        if screen == Screen::Playing {
            draw_scoreboard(scores, mode, viewport);
        }
        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ball_rebounds_from_top_wall_without_changing_horizontal_direction() {
        let mut ball = Ball {
            x: 200.0,
            y: TOP_DASH_HEIGHT + BALL_SIZE / 2.0 + 1.0,
            vx: -100.0,
            vy: -100.0,
            spin: 0.0,
            speed_scale: 1.0,
            moving: true,
            missed_by: None,
            trail: Vec::new(),
        };

        assert_eq!(
            ball.update(
                0.04,
                WINDOW_HEIGHT as f32 / 2.0,
                WINDOW_HEIGHT as f32 / 2.0,
                RIGHT_MARKER_HEIGHT,
                false,
                false,
            ),
            None
        );

        assert!(ball.vx < 0.0);
        assert!(ball.vy > 0.0);
    }

    #[test]
    fn serve_uses_the_selected_game_speed() {
        let mut ball = Ball::new();
        ball.serve(GAME_SPEEDS[0]);

        assert_eq!(ball.vx, -BALL_HORIZONTAL_SPEED * GAME_SPEEDS[0]);
        assert_eq!(ball.vy, BALL_VERTICAL_SPEED * GAME_SPEEDS[0]);
        assert_eq!(ball.speed_scale, GAME_SPEEDS[0]);
    }

    #[test]
    fn paddle_return_speeds_up_ball_toward_left_wall() {
        let paddle_x = WINDOW_WIDTH as f32 - RIGHT_MARKER_OFFSET;
        let paddle_y = WINDOW_HEIGHT as f32 / 2.0;
        let mut ball = Ball {
            x: paddle_x - BALL_SIZE,
            y: paddle_y,
            vx: 300.0,
            vy: 0.0,
            spin: 0.0,
            speed_scale: 1.0,
            moving: true,
            missed_by: None,
            trail: Vec::new(),
        };

        assert_eq!(
            ball.update(0.04, paddle_y, paddle_y, RIGHT_MARKER_HEIGHT, false, false),
            None
        );

        assert!(ball.moving);
        assert!(ball.vx < 0.0);
        assert!(ball.vx.hypot(ball.vy) > 300.0);
        assert!(ball.vx.hypot(ball.vy) <= MAX_BALL_SPEED);
    }

    #[test]
    fn selected_paddle_size_changes_the_collision_window() {
        let paddle_x = WINDOW_WIDTH as f32 - RIGHT_MARKER_OFFSET;
        let paddle_y = WINDOW_HEIGHT as f32 / 2.0;
        let mut large_paddle_hit = Ball {
            x: paddle_x - BALL_SIZE,
            y: paddle_y + 30.0,
            vx: 300.0,
            vy: 0.0,
            spin: 0.0,
            speed_scale: 1.0,
            moving: true,
            missed_by: None,
            trail: Vec::new(),
        };
        let mut small_paddle_miss = Ball {
            x: paddle_x - BALL_SIZE,
            y: paddle_y + 30.0,
            vx: 300.0,
            vy: 0.0,
            spin: 0.0,
            speed_scale: 1.0,
            moving: true,
            missed_by: None,
            trail: Vec::new(),
        };

        let other_paddle_y = paddle_y - RIGHT_MARKER_HEIGHT;
        large_paddle_hit.update(
            0.04,
            other_paddle_y,
            paddle_y,
            PADDLE_SIZES[2],
            false,
            false,
        );
        small_paddle_miss.update(
            0.04,
            other_paddle_y,
            paddle_y,
            PADDLE_SIZES[0],
            false,
            false,
        );

        assert!(large_paddle_hit.vx < 0.0);
        assert_eq!(small_paddle_miss.missed_by, Some(Player::One));
    }

    #[test]
    fn paddle_hit_position_controls_pong_style_return_angle() {
        let mut center_hit = Ball::new();
        center_hit.vx = 300.0;
        center_hit.return_from_paddle(0.0, false, false, false);
        assert!(center_hit.vx < 0.0);
        assert!(center_hit.vy.abs() < f32::EPSILON);

        let mut edge_hit = Ball::new();
        edge_hit.vx = 300.0;
        edge_hit.return_from_paddle(2.0, false, false, false);
        let edge_angle = edge_hit.vy.atan2(-edge_hit.vx);
        assert!((edge_angle - MAX_BOUNCE_ANGLE).abs() < 0.001);
    }

    #[test]
    fn slice_is_fast_and_stays_flat() {
        let mut ball = Ball::new();
        ball.vx = 300.0;
        ball.return_from_paddle(1.0, true, false, false);

        assert!(ball.vx < 0.0);
        assert!(ball.vy.abs() < 60.0);
        assert!(ball.vx.hypot(ball.vy) > 300.0 * RALLY_SPEEDUP);
    }

    #[test]
    fn curve_adds_spin_without_sending_ball_behind_paddle() {
        let paddle_x = WINDOW_WIDTH as f32 - RIGHT_MARKER_OFFSET;
        let paddle_y = WINDOW_HEIGHT as f32 / 2.0;
        let mut ball = Ball {
            x: paddle_x - BALL_SIZE,
            y: paddle_y + 20.0,
            vx: 300.0,
            vy: 0.0,
            spin: 0.0,
            speed_scale: 1.0,
            moving: true,
            missed_by: None,
            trail: Vec::new(),
        };

        assert_eq!(
            ball.update(0.04, paddle_y, paddle_y, RIGHT_MARKER_HEIGHT, false, true),
            None
        );
        let curved_vy = ball.vy;
        assert_eq!(
            ball.update(0.04, paddle_y, paddle_y, RIGHT_MARKER_HEIGHT, false, false),
            None
        );

        assert!(ball.spin > 0.0);
        assert!(ball.vy > curved_vy);
        assert!(ball.vx < 0.0);
        assert!(ball.x + BALL_SIZE / 2.0 <= paddle_x);
    }

    #[test]
    fn second_paddle_in_shared_column_returns_ball_and_speeds_it_up() {
        let paddle_y = WINDOW_HEIGHT as f32 / 2.0 - RIGHT_MARKER_HEIGHT;
        let first_paddle_y = WINDOW_HEIGHT as f32 / 2.0;
        let mut ball = Ball {
            x: PADDLE_COLUMN_X - BALL_SIZE,
            y: paddle_y,
            vx: 300.0,
            vy: 0.0,
            spin: 0.0,
            speed_scale: 1.0,
            moving: true,
            missed_by: None,
            trail: Vec::new(),
        };

        assert_eq!(
            ball.update(
                0.04,
                paddle_y,
                first_paddle_y,
                RIGHT_MARKER_HEIGHT,
                false,
                false
            ),
            None
        );

        assert!(ball.moving);
        assert!(ball.vx < 0.0);
        assert!(ball.vy.abs() < f32::EPSILON);
        assert!(ball.vx.hypot(ball.vy) > 300.0);
    }

    #[test]
    fn paddle_miss_awards_point_to_the_other_player() {
        let second_paddle_y = WINDOW_HEIGHT as f32 / 2.0 - RIGHT_MARKER_HEIGHT;
        let first_paddle_y = WINDOW_HEIGHT as f32 / 2.0;
        let mut ball = Ball {
            x: PADDLE_COLUMN_X - BALL_SIZE,
            y: first_paddle_y + RIGHT_MARKER_HEIGHT * 1.2,
            vx: 300.0,
            vy: 0.0,
            spin: 0.0,
            speed_scale: 1.0,
            moving: true,
            missed_by: None,
            trail: Vec::new(),
        };

        assert_eq!(
            ball.update(
                0.04,
                second_paddle_y,
                first_paddle_y,
                RIGHT_MARKER_HEIGHT,
                false,
                false
            ),
            None
        );
        assert!(ball.moving);
        assert_eq!(ball.missed_by, Some(Player::One));

        let mut scoring_player = None;
        for _ in 0..20 {
            scoring_player = ball.update(
                0.04,
                second_paddle_y,
                first_paddle_y,
                RIGHT_MARKER_HEIGHT,
                false,
                false,
            );
            if scoring_player.is_some() {
                break;
            }
        }

        assert_eq!(scoring_player, Some(Player::Two));
        assert!(!ball.moving);
    }

    #[test]
    fn resized_viewport_preserves_aspect_ratio_and_centers_game() {
        assert_eq!(fitted_viewport(1600.0, 900.0), (200, 0, 1200, 900));
    }

    #[test]
    fn both_paddle_control_pairs_use_the_same_reversed_direction() {
        assert_eq!(paddle_direction(true, false), 1.0);
        assert_eq!(paddle_direction(false, true), -1.0);
        assert_eq!(paddle_direction(false, false), 0.0);
    }
}
