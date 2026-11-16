use macroquad::prelude::*;

const SCREEN_WIDTH: f32 = 960.0;
const SCREEN_HEIGHT: f32 = 600.0;
const LEFT: f32 = 143.0;
const RIGHT: f32 = 817.0;
const TOP: f32 = 83.0;
const BOTTOM: f32 = 520.0;
const WINNING_SCORE: u8 = 9;

const PAPER: Color = Color::new(0.94, 0.87, 0.72, 1.0);
const ORANGE: Color = Color::new(0.90, 0.42, 0.21, 1.0);
const OLIVE: Color = Color::new(0.66, 0.64, 0.30, 1.0);
const COURT: Color = Color::new(0.30, 0.36, 0.30, 1.0);
const COURT_LIGHT: Color = Color::new(0.38, 0.44, 0.35, 1.0);
const INK: Color = Color::new(0.12, 0.12, 0.10, 1.0);
const MUTED: Color = Color::new(0.66, 0.60, 0.47, 1.0);
const BALL_COLOR: Color = Color::new(0.83, 0.84, 0.34, 1.0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
    Ready,
    Playing,
    Point,
    GameOver,
}

struct Racket {
    x: f32,
    y: f32,
    cooldown: f32,
    swing_time: f32,
}

struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    trail: Vec<Vec2>,
}

struct Game {
    phase: Phase,
    player_score: u8,
    opponent_score: u8,
    player_serves: bool,
    player: Racket,
    opponent: Racket,
    ball: Ball,
    bounces: u8,
    point_timer: f32,
    last_point: &'static str,
}

impl Game {
    fn new() -> Self {
        Self {
            phase: Phase::Ready,
            player_score: 0,
            opponent_score: 0,
            player_serves: true,
            player: Racket {
                x: 480.0,
                y: 460.0,
                cooldown: 0.0,
                swing_time: 0.0,
            },
            opponent: Racket {
                x: 480.0,
                y: 137.0,
                cooldown: 0.0,
                swing_time: 0.0,
            },
            ball: Ball {
                x: 480.0,
                y: 300.0,
                vx: 0.0,
                vy: 0.0,
                trail: Vec::new(),
            },
            bounces: 0,
            point_timer: 0.0,
            last_point: "",
        }
    }

    fn serve(&mut self) {
        self.phase = Phase::Playing;
        self.bounces = 0;
        self.ball.trail.clear();
        if self.player_serves {
            self.ball.x = self.player.x;
            self.ball.y = self.player.y - 42.0;
            self.ball.vx = rand::gen_range(-1.5, 1.5);
            self.ball.vy = -345.0;
        } else {
            self.ball.x = self.opponent.x;
            self.ball.y = self.opponent.y + 35.0;
            self.ball.vx = rand::gen_range(-1.5, 1.5);
            self.ball.vy = 315.0;
        }
    }

    fn score_point(&mut self, player_wins: bool) {
        if self.phase != Phase::Playing {
            return;
        }
        if player_wins {
            self.player_score += 1;
            self.last_point = "POINT TO YOU!";
        } else {
            self.opponent_score += 1;
            self.last_point = "POINT TO CLUB PRO";
        }
        self.player_serves = player_wins;
        self.phase = if self.player_score >= WINNING_SCORE || self.opponent_score >= WINNING_SCORE {
            Phase::GameOver
        } else {
            Phase::Point
        };
        self.point_timer = 1.2;
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn move_racket(racket: &mut Racket, dx: f32, dy: f32, min_y: f32, max_y: f32, dt: f32) {
        racket.x = (racket.x + dx * 290.0 * dt).clamp(LEFT + 28.0, RIGHT - 28.0);
        racket.y = (racket.y + dy * 250.0 * dt).clamp(min_y, max_y);
    }

    fn swing(racket: &mut Racket, ball: &mut Ball, send_down: bool) -> bool {
        if racket.cooldown > 0.0 || (ball.x - racket.x).hypot(ball.y - racket.y) > 81.0 {
            return false;
        }
        racket.swing_time = 0.18;
        racket.cooldown = 0.36;
        let horizontal = ((ball.x - racket.x) / 75.0).clamp(-1.0, 1.0);
        ball.vx = horizontal * 250.0 + rand::gen_range(-42.0, 42.0);
        ball.vy = if send_down {
            ball.vy.abs().max(280.0) + 60.0
        } else {
            -(ball.vy.abs().max(280.0) + 60.0)
        };
        ball.x += if horizontal == 0.0 {
            3.0
        } else {
            horizontal.signum() * 3.0
        };
        true
    }

    fn update(&mut self, dt: f32) {
        self.player.cooldown = (self.player.cooldown - dt).max(0.0);
        self.opponent.cooldown = (self.opponent.cooldown - dt).max(0.0);
        self.player.swing_time = (self.player.swing_time - dt).max(0.0);
        self.opponent.swing_time = (self.opponent.swing_time - dt).max(0.0);

        let dx = f32::from(is_key_down(KeyCode::Right) || is_key_down(KeyCode::D))
            - f32::from(is_key_down(KeyCode::Left) || is_key_down(KeyCode::A));
        let dy = f32::from(is_key_down(KeyCode::Down) || is_key_down(KeyCode::S))
            - f32::from(is_key_down(KeyCode::Up) || is_key_down(KeyCode::W));
        Self::move_racket(&mut self.player, dx, dy, 318.0, 488.0, dt);

        if self.phase == Phase::Point {
            self.point_timer -= dt;
            if self.point_timer <= 0.0 {
                self.phase = Phase::Ready;
            }
        }

        if self.phase != Phase::Playing {
            return;
        }

        let target_x = self.ball.x.clamp(LEFT + 35.0, RIGHT - 35.0);
        let target_y = if self.ball.vy < 0.0 { 137.0 } else { 175.0 };
        let ai_dx = ((target_x - self.opponent.x) / 18.0).clamp(-1.0, 1.0) * 0.78;
        let ai_dy = ((target_y - self.opponent.y) / 18.0).clamp(-1.0, 1.0) * 0.6;
        Self::move_racket(&mut self.opponent, ai_dx, ai_dy, 112.0, 190.0, dt);
        if self.ball.vy < 0.0
            && (self.ball.x - self.opponent.x).hypot(self.ball.y - self.opponent.y) < 70.0
        {
            Self::swing(&mut self.opponent, &mut self.ball, true);
        }

        if is_key_pressed(KeyCode::Space) {
            Self::swing(&mut self.player, &mut self.ball, false);
        }

        self.ball.trail.insert(0, vec2(self.ball.x, self.ball.y));
        self.ball.trail.truncate(7);
        self.ball.x += self.ball.vx * dt;
        self.ball.y += self.ball.vy * dt;

        if self.ball.x < LEFT + 9.0 || self.ball.x > RIGHT - 9.0 {
            self.ball.x = self.ball.x.clamp(LEFT + 9.0, RIGHT - 9.0);
            self.ball.vx *= -0.91;
        }
        if self.ball.y < TOP + 9.0 {
            self.ball.y = TOP + 9.0;
            self.ball.vy *= -0.84;
            self.bounces += 1;
        } else if self.ball.y > BOTTOM - 9.0 {
            self.ball.y = BOTTOM - 9.0;
            self.ball.vy *= -0.84;
            self.bounces += 1;
        }

        if self.ball.y > self.player.y + 55.0 {
            self.score_point(false);
        } else if self.ball.y < self.opponent.y - 50.0 {
            self.score_point(true);
        } else if self.bounces >= 2 {
            self.score_point(self.ball.y < 300.0);
        }
        if self.ball.vx.abs() < 90.0 {
            self.ball.vx *= 1.015;
        }
    }

    fn status(&self) -> &'static str {
        match self.phase {
            Phase::Ready => "READY TO PLAY",
            Phase::Playing => "RALLY IN PLAY",
            Phase::Point => self.last_point,
            Phase::GameOver if self.player_score > self.opponent_score => "MATCH WON!",
            Phase::GameOver => "MATCH LOST",
        }
    }

    fn message(&self) -> Option<&'static str> {
        match self.phase {
            Phase::Ready => Some("PRESS ENTER TO SERVE"),
            Phase::Point => Some(self.last_point),
            Phase::GameOver if self.player_score > self.opponent_score => {
                Some("MATCH WON - WELL PLAYED!")
            }
            Phase::GameOver => Some("MATCH OVER - REMATCH?"),
            Phase::Playing => None,
        }
    }
}

fn draw_court() {
    clear_background(Color::new(0.16, 0.19, 0.16, 1.0));

    draw_triangle(
        vec2(112.0, 72.0),
        vec2(848.0, 72.0),
        vec2(914.0, 544.0),
        Color::new(0.84, 0.72, 0.49, 1.0),
    );
    draw_triangle(
        vec2(112.0, 72.0),
        vec2(914.0, 544.0),
        vec2(46.0, 544.0),
        Color::new(0.84, 0.72, 0.49, 1.0),
    );
    draw_rectangle(143.0, 83.0, 674.0, 437.0, COURT);
    draw_rectangle(143.0, 83.0, 674.0, 240.0, COURT_LIGHT);
    draw_rectangle(143.0, 83.0, 674.0, 19.0, OLIVE);
    draw_rectangle(143.0, 476.0, 674.0, 13.0, ORANGE);
    draw_rectangle(143.0, 489.0, 674.0, 5.0, PAPER);

    for (x1, y1, x2, y2) in [
        (143.0, 344.0, 817.0, 344.0),
        (143.0, 456.0, 817.0, 456.0),
        (480.0, 344.0, 480.0, 456.0),
        (143.0, 82.0, 79.0, 520.0),
        (817.0, 82.0, 881.0, 520.0),
    ] {
        draw_line(x1, y1, x2, y2, 1.5, PAPER);
    }
    draw_rectangle_lines(143.0, 83.0, 674.0, 437.0, 2.0, INK);
    draw_line(112.0, 72.0, 848.0, 72.0, 2.0, MUTED);
    draw_line(848.0, 72.0, 914.0, 544.0, 2.0, MUTED);
    draw_line(914.0, 544.0, 46.0, 544.0, 2.0, MUTED);
    draw_line(46.0, 544.0, 112.0, 72.0, 2.0, MUTED);
    draw_text("FRONT WALL", 425.0, 128.0, 16.0, PAPER);
    draw_text("SQUASH CLUB   -   EST. 1976", 382.0, 154.0, 13.0, PAPER);
}

fn draw_racket(racket: &Racket, is_player: bool, ball: &Ball) {
    let color = if is_player { ORANGE } else { PAPER };
    draw_ellipse(racket.x, racket.y + 17.0, 26.0, 13.0, 0.0, INK);
    draw_ellipse(
        racket.x,
        racket.y + 1.0,
        15.0,
        20.0,
        0.0,
        if is_player {
            ORANGE
        } else {
            Color::new(0.84, 0.77, 0.61, 1.0)
        },
    );
    draw_ellipse(
        racket.x,
        racket.y + 1.0,
        8.0,
        11.0,
        0.0,
        Color::new(0.61, 0.29, 0.18, 1.0),
    );

    let angle = (ball.y - racket.y).atan2(ball.x - racket.x);
    let head_x = racket.x + angle.cos() * 36.0 - angle.sin() * 9.0;
    let head_y = racket.y + angle.sin() * 36.0 + angle.cos() * 9.0;
    draw_ellipse_lines(head_x, head_y, 34.0, 20.0, angle, 3.0, color);
    draw_line(
        racket.x + angle.cos() * 49.0,
        racket.y + angle.sin() * 49.0,
        racket.x + angle.cos() * 67.0,
        racket.y + angle.sin() * 67.0,
        4.0,
        Color::new(0.55, 0.35, 0.20, 1.0),
    );
    for offset in [-5.0, 0.0, 5.0] {
        let cross_x = head_x + angle.cos() * offset;
        let cross_y = head_y + angle.sin() * offset;
        draw_line(
            cross_x - angle.sin() * 10.0,
            cross_y + angle.cos() * 10.0,
            cross_x + angle.sin() * 10.0,
            cross_y - angle.cos() * 10.0,
            1.0,
            PAPER,
        );
    }
    if racket.swing_time > 0.0 {
        draw_circle_lines(
            racket.x,
            racket.y,
            49.0,
            2.0,
            Color::new(0.95, 0.79, 0.41, 1.0),
        );
    }
}

fn draw_ball(ball: &Ball) {
    for (i, point) in ball.trail.iter().enumerate().rev() {
        let alpha = 0.3 * (1.0 - i as f32 / ball.trail.len().max(1) as f32);
        draw_circle(
            point.x,
            point.y,
            9.0 * (1.0 - i as f32 * 0.06),
            Color::new(BALL_COLOR.r, BALL_COLOR.g, BALL_COLOR.b, alpha),
        );
    }
    draw_circle(ball.x, ball.y, 9.0, BALL_COLOR);
    draw_circle(
        ball.x - 3.0,
        ball.y - 3.0,
        3.0,
        Color::new(0.96, 0.94, 0.68, 1.0),
    );
}

fn draw_overlay(game: &Game) {
    let Some(message) = game.message() else {
        return;
    };
    draw_rectangle(
        239.0,
        251.0,
        482.0,
        84.0,
        Color::new(0.09, 0.11, 0.09, 0.84),
    );
    draw_rectangle_lines(
        239.0,
        251.0,
        482.0,
        84.0,
        2.0,
        Color::new(0.83, 0.72, 0.44, 1.0),
    );
    let message_width = measure_text(message, None, 20, 1.0).width;
    draw_text(message, 480.0 - message_width / 2.0, 290.0, 20.0, PAPER);
    let hint = if game.phase == Phase::GameOver {
        "PRESS ENTER FOR A NEW MATCH"
    } else {
        "MOVE  -  TIME YOUR SWING  -  WIN THE RALLY"
    };
    let hint_width = measure_text(hint, None, 11, 1.0).width;
    draw_text(hint, 480.0 - hint_width / 2.0, 315.0, 11.0, OLIVE);
}

fn draw_game(game: &Game) {
    draw_court();
    draw_racket(&game.opponent, false, &game.ball);
    draw_racket(&game.player, true, &game.ball);
    if game.phase == Phase::Playing {
        draw_ball(&game.ball);
    } else if game.phase != Phase::GameOver {
        draw_circle(game.ball.x, game.ball.y, 9.0, BALL_COLOR);
    }
    draw_overlay(game);

    draw_text("SQUASH '76", 34.0, 39.0, 24.0, PAPER);
    draw_text("THE BACK-COURT CLASSIC", 36.0, 56.0, 10.0, OLIVE);
    draw_text(
        &format!("{}   :   {}", game.player_score, game.opponent_score),
        750.0,
        45.0,
        28.0,
        PAPER,
    );
    draw_text(
        if game.player_serves {
            "YOUR SERVE"
        } else {
            "CLUB PRO SERVES"
        },
        750.0,
        61.0,
        10.0,
        ORANGE,
    );

    let status_width = measure_text(game.status(), None, 15, 1.0).width;
    draw_text(
        game.status(),
        SCREEN_WIDTH / 2.0 - status_width / 2.0,
        579.0,
        15.0,
        OLIVE,
    );
    draw_text(
        "MOVE  ARROWS / WASD     SWING  SPACE     SERVE  ENTER",
        33.0,
        579.0,
        12.0,
        MUTED,
    );
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Squash '76".to_owned(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        let dt = get_frame_time().min(0.04);
        if is_key_pressed(KeyCode::Enter) {
            match game.phase {
                Phase::Ready => game.serve(),
                Phase::GameOver => game.reset(),
                Phase::Point => game.serve(),
                Phase::Playing => {}
            }
        }

        game.update(dt);
        draw_game(&game);
        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winning_a_point_awards_score_and_serve() {
        let mut game = Game::new();
        game.phase = Phase::Playing;

        game.score_point(true);

        assert_eq!(game.player_score, 1);
        assert_eq!(game.opponent_score, 0);
        assert!(game.player_serves);
        assert_eq!(game.phase, Phase::Point);
    }

    #[test]
    fn reaching_nine_ends_the_match() {
        let mut game = Game::new();
        game.player_score = WINNING_SCORE - 1;
        game.phase = Phase::Playing;

        game.score_point(true);

        assert_eq!(game.player_score, WINNING_SCORE);
        assert_eq!(game.phase, Phase::GameOver);
    }
}
