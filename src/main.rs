use macroquad::prelude::*;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const LEFT_BAR_WIDTH: f32 = 12.0;
const TOP_DASH_WIDTH: f32 = 28.0;
const TOP_DASH_GAP: f32 = 18.0;
const TOP_DASH_HEIGHT: f32 = 12.0;
const RIGHT_MARKER_OFFSET: f32 = 133.0;
const RIGHT_MARKER_HEIGHT: f32 = 82.0;
const RIGHT_MARKER_WIDTH: f32 = 10.0;
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
const SCANLINE_COLOR: Color = Color::new(0.045, 0.028, 0.009, 1.0);
const WALL_SCANLINE_COLOR: Color = Color::new(0.62, 0.42, 0.18, 1.0);
const PADDLE_SCANLINE_COLOR: Color = Color::new(0.58, 0.39, 0.14, 1.0);

struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    spin: f32,
    moving: bool,
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
            moving: false,
            trail: Vec::new(),
        }
    }

    fn serve(&mut self) {
        self.x = WINDOW_WIDTH as f32 / 2.0;
        self.y = WINDOW_HEIGHT as f32 / 2.0;
        self.vx = -BALL_HORIZONTAL_SPEED;
        self.vy = BALL_VERTICAL_SPEED;
        self.spin = 0.0;
        self.moving = true;
        self.trail.clear();
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn return_from_paddle(&mut self, impact: f32, slice: bool, curve: bool) {
        let speedup = if slice { SLICE_SPEEDUP } else { RALLY_SPEEDUP };
        let speed = (self.vx.hypot(self.vy) * speedup).min(MAX_BALL_SPEED);
        let angle_scale = if slice { 0.18 } else { MAX_BOUNCE_ANGLE };
        let angle = impact.clamp(-1.0, 1.0) * angle_scale;
        self.vx = -speed * angle.cos();
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

    fn update(&mut self, dt: f32, paddle_y: f32, slice: bool, curve: bool) {
        if !self.moving {
            return;
        }

        let radius = BALL_SIZE / 2.0;
        self.trail.insert(0, vec2(self.x, self.y));
        self.trail.truncate(8);
        self.vy += self.spin * dt;
        self.spin *= (-0.8 * dt).exp();
        let speed = self.vx.hypot(self.vy);
        if speed > MAX_BALL_SPEED {
            let scale = MAX_BALL_SPEED / speed;
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

        let paddle_x = WINDOW_WIDTH as f32 - RIGHT_MARKER_OFFSET;
        let reached_paddle =
            self.vx > 0.0 && previous_x + radius < paddle_x && self.x + radius >= paddle_x;
        if reached_paddle {
            let impact = (self.y - paddle_y) / (RIGHT_MARKER_HEIGHT / 2.0);
            if impact.abs() <= 1.0 {
                self.x = paddle_x - radius;
                self.return_from_paddle(impact, slice, curve);
            } else {
                self.reset();
            }
        } else if self.x - radius > paddle_x + RIGHT_MARKER_WIDTH {
            self.reset();
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Squash '76".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
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

fn draw_scanlined_rect(x: f32, y: f32, width: f32, height: f32, line_color: Color) {
    draw_rectangle(x, y, width, height, PHOSPHOR);
    let mut line_y = y + 2.0;
    while line_y < y + height {
        draw_rectangle(x, line_y, width, 1.0, line_color);
        line_y += SCANLINE_SPACING;
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut paddle_y = screen_height() / 2.0;
    let mut ball = Ball::new();

    loop {
        clear_background(BLACK);
        let dt = get_frame_time().min(0.04);

        let direction = f32::from(is_key_down(KeyCode::S)) - f32::from(is_key_down(KeyCode::W));
        paddle_y = (paddle_y + direction * PADDLE_SPEED * dt).clamp(
            TOP_DASH_HEIGHT + RIGHT_MARKER_HEIGHT / 2.0,
            screen_height() - TOP_DASH_HEIGHT - RIGHT_MARKER_HEIGHT / 2.0,
        );
        if is_key_pressed(KeyCode::Space) && !ball.moving {
            ball.serve();
        }
        ball.update(
            dt,
            paddle_y,
            is_key_down(KeyCode::A),
            is_key_down(KeyCode::D),
        );

        let mut y = 2.0;
        while y < screen_height() {
            draw_rectangle(0.0, y, screen_width(), 1.0, SCANLINE_COLOR);
            y += SCANLINE_SPACING;
        }

        draw_scanlined_rect(
            0.0,
            0.0,
            LEFT_BAR_WIDTH,
            screen_height(),
            WALL_SCANLINE_COLOR,
        );
        let mut x = 0.0;
        while x < screen_width() {
            draw_scanlined_rect(x, 0.0, TOP_DASH_WIDTH, TOP_DASH_HEIGHT, WALL_SCANLINE_COLOR);
            x += TOP_DASH_WIDTH + TOP_DASH_GAP;
        }
        let bottom_y = screen_height() - TOP_DASH_HEIGHT;
        let mut x = 0.0;
        while x < screen_width() {
            draw_scanlined_rect(
                x,
                bottom_y,
                TOP_DASH_WIDTH,
                TOP_DASH_HEIGHT,
                WALL_SCANLINE_COLOR,
            );
            x += TOP_DASH_WIDTH + TOP_DASH_GAP;
        }
        draw_scanlined_rect(
            screen_width() - RIGHT_MARKER_OFFSET,
            paddle_y - RIGHT_MARKER_HEIGHT / 2.0,
            RIGHT_MARKER_WIDTH,
            RIGHT_MARKER_HEIGHT,
            PADDLE_SCANLINE_COLOR,
        );
        draw_ball(&ball);
        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ball_rebounds_from_left_and_top_walls() {
        let mut ball = Ball {
            x: LEFT_BAR_WIDTH + BALL_SIZE / 2.0 + 1.0,
            y: TOP_DASH_HEIGHT + BALL_SIZE / 2.0 + 1.0,
            vx: -100.0,
            vy: -100.0,
            spin: 0.0,
            moving: true,
            trail: Vec::new(),
        };

        ball.update(0.04, WINDOW_HEIGHT as f32 / 2.0, false, false);

        assert!(ball.vx > 0.0);
        assert!(ball.vy > 0.0);
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
            moving: true,
            trail: Vec::new(),
        };

        ball.update(0.04, paddle_y, false, false);

        assert!(ball.moving);
        assert!(ball.vx < 0.0);
        assert!(ball.vx.hypot(ball.vy) > 300.0);
        assert!(ball.vx.hypot(ball.vy) <= MAX_BALL_SPEED);
    }

    #[test]
    fn paddle_hit_position_controls_pong_style_return_angle() {
        let mut center_hit = Ball::new();
        center_hit.vx = 300.0;
        center_hit.return_from_paddle(0.0, false, false);
        assert!(center_hit.vx < 0.0);
        assert!(center_hit.vy.abs() < f32::EPSILON);

        let mut edge_hit = Ball::new();
        edge_hit.vx = 300.0;
        edge_hit.return_from_paddle(2.0, false, false);
        let edge_angle = edge_hit.vy.atan2(-edge_hit.vx);
        assert!((edge_angle - MAX_BOUNCE_ANGLE).abs() < 0.001);
    }

    #[test]
    fn slice_is_fast_and_stays_flat() {
        let mut ball = Ball::new();
        ball.vx = 300.0;
        ball.return_from_paddle(1.0, true, false);

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
            moving: true,
            trail: Vec::new(),
        };

        ball.update(0.04, paddle_y, false, true);
        let curved_vy = ball.vy;
        ball.update(0.04, paddle_y, false, false);

        assert!(ball.spin > 0.0);
        assert!(ball.vy > curved_vy);
        assert!(ball.vx < 0.0);
        assert!(ball.x + BALL_SIZE / 2.0 <= paddle_x);
    }
}
