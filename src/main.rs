#![windows_subsystem = "windows"]

use gilrs::{Axis, Button, EventType, Gilrs};
use macroquad::audio::{load_sound_from_bytes, play_sound_once};
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
const CURVE_ACCELERATION: f32 = 600.0;
const GAME_POINT_LIMIT: u32 = 15;
const SCANLINE_SPACING: f32 = 4.0;
const BOUNCE_SAMPLE_RATE: u32 = 48_000;
const BOUNCE_FREQUENCY_HZ: u32 = 1_200;
const BOUNCE_DURATION_MS: u32 = 60;
const BOUNCE_AMPLITUDE: i16 = 12_000;
const IMPACT_FREQUENCIES_HZ: [u32; 2] = [900, 1_500];
const IMPACT_DURATION_MS: u32 = 60;
const SCORE_START_FREQUENCY_HZ: f32 = 1_600.0;
const SCORE_END_FREQUENCY_HZ: f32 = 420.0;
const SCORE_DURATION_MS: u32 = 250;
const GAME_OVER_START_FREQUENCY_HZ: f32 = 240.0;
const GAME_OVER_END_FREQUENCY_HZ: f32 = 85.0;
const GAME_OVER_DURATION_MS: u32 = 280;
const ALERT_AMPLITUDE: i16 = 12_000;
const GAME_SPEEDS: [f32; 3] = [0.75, 1.0, 1.35];
const GAME_SPEED_LABELS: [&str; 3] = ["SLOW", "NORMAL", "FAST"];
const PADDLE_SIZES: [f32; 2] = [56.0, RIGHT_MARKER_HEIGHT];
const PADDLE_SIZE_LABELS: [&str; 2] = ["SMALL", "NORMAL"];
const REBOUND_ANGLE_DEGREES: [f32; 2] = [20.0, 40.0];
const REBOUND_ANGLE_LABELS: [&str; 2] = ["SHALLOW 20", "STEEP 40"];
const PLAYER_MODE_LABELS: [&str; 2] = ["1 PLAYER", "2 PLAYER"];
const SERVE_MODE_LABELS: [&str; 2] = ["AUTO", "MANUAL"];
const SCANLINE_LABELS: [&str; 2] = ["OFF", "ON"];
const GAME_COLOR_LABELS: [&str; 3] = ["GREEN", "AMBER", "WHITE"];
const CONTROL_MODE_LABELS: [&str; 2] = ["KEYBOARD", "JOYSTICK"];

#[derive(Clone, Copy)]
struct Palette {
    primary: Color,
    secondary: Color,
    background_scanline: Color,
    wall_scanline: Color,
    paddle_scanline: Color,
    second_paddle_scanline: Color,
    highlight: Color,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GameColor {
    Green,
    Amber,
    White,
}

impl GameColor {
    fn from_index(index: usize) -> Self {
        match index {
            0 => Self::Green,
            1 => Self::Amber,
            _ => Self::White,
        }
    }

    fn index(self) -> usize {
        match self {
            Self::Green => 0,
            Self::Amber => 1,
            Self::White => 2,
        }
    }

    fn palette(self) -> Palette {
        match self {
            Self::Green => Palette {
                primary: Color::new(0.34, 0.95, 0.48, 1.0),
                secondary: Color::new(0.66, 1.0, 0.62, 1.0),
                background_scanline: Color::new(0.012, 0.055, 0.018, 1.0),
                wall_scanline: Color::new(0.22, 0.68, 0.32, 1.0),
                paddle_scanline: Color::new(0.18, 0.62, 0.28, 1.0),
                second_paddle_scanline: Color::new(0.38, 0.76, 0.42, 1.0),
                highlight: Color::new(0.75, 1.0, 0.72, 1.0),
            },
            Self::Amber => Palette {
                primary: Color::new(0.82, 0.54, 0.18, 1.0),
                secondary: Color::new(0.9, 0.64, 0.32, 1.0),
                background_scanline: Color::new(0.045, 0.028, 0.009, 1.0),
                wall_scanline: Color::new(0.62, 0.42, 0.18, 1.0),
                paddle_scanline: Color::new(0.58, 0.39, 0.14, 1.0),
                second_paddle_scanline: Color::new(0.7, 0.52, 0.28, 1.0),
                highlight: Color::new(0.95, 0.74, 0.39, 1.0),
            },
            Self::White => Palette {
                primary: Color::new(0.92, 0.94, 1.0, 1.0),
                secondary: Color::new(0.72, 0.78, 0.88, 1.0),
                background_scanline: Color::new(0.035, 0.04, 0.055, 1.0),
                wall_scanline: Color::new(0.72, 0.78, 0.88, 1.0),
                paddle_scanline: Color::new(0.64, 0.7, 0.82, 1.0),
                second_paddle_scanline: Color::new(0.54, 0.62, 0.75, 1.0),
                highlight: Color::new(1.0, 1.0, 1.0, 1.0),
            },
        }
    }
}

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

#[derive(Clone, Copy, PartialEq, Eq)]
enum ServeMode {
    Automatic,
    Manual,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ControlMode {
    Keyboard,
    Joystick,
}

#[derive(Default)]
struct JoystickInput {
    connected: bool,
    player_one_y: f32,
    player_two_y: f32,
    player_one_up: bool,
    player_one_down: bool,
    player_two_up: bool,
    player_two_down: bool,
    serve_pressed: bool,
    menu_pressed: bool,
    slice_pressed: bool,
    curve_pressed: bool,
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
    bounce_this_frame: Option<BounceKind>,
    trail: Vec<Vec2>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BounceKind {
    SideWall,
    HardImpact,
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
            bounce_this_frame: None,
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

    fn return_from_paddle(
        &mut self,
        impact: f32,
        slice: bool,
        curve: bool,
        toward_right: bool,
        rebound_angle_degrees: f32,
    ) {
        let speedup = if slice { SLICE_SPEEDUP } else { RALLY_SPEEDUP };
        let speed = (self.vx.hypot(self.vy) * speedup).min(MAX_BALL_SPEED * self.speed_scale);
        let angle_scale = if slice {
            0.18
        } else {
            rebound_angle_degrees.to_radians()
        };
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
        mode: GameMode,
        rebound_angle_degrees: f32,
    ) -> Option<Player> {
        self.bounce_this_frame = None;
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
            self.bounce_this_frame = Some(BounceKind::HardImpact);
        }
        if self.y - radius <= TOP_DASH_HEIGHT {
            self.y = TOP_DASH_HEIGHT + radius;
            self.vy = self.vy.abs();
            self.spin = -self.spin * 0.8;
            self.bounce_this_frame = Some(BounceKind::SideWall);
        } else if self.y + radius >= WINDOW_HEIGHT as f32 - TOP_DASH_HEIGHT {
            self.y = WINDOW_HEIGHT as f32 - TOP_DASH_HEIGHT - radius;
            self.vy = -self.vy.abs();
            self.spin = -self.spin * 0.8;
            self.bounce_this_frame = Some(BounceKind::SideWall);
        }

        let reached_paddles = self.vx > 0.0
            && previous_x + radius < PADDLE_COLUMN_X
            && self.x + radius >= PADDLE_COLUMN_X;
        if reached_paddles {
            let first_impact = (self.y - right_paddle_y) / (paddle_height / 2.0);
            let (missed_player, impact) = if mode == GameMode::OnePlayer {
                (Player::One, first_impact)
            } else {
                let second_impact = (self.y - second_paddle_y) / (paddle_height / 2.0);
                if second_impact.abs() <= first_impact.abs() {
                    (Player::Two, second_impact)
                } else {
                    (Player::One, first_impact)
                }
            };
            if impact.abs() <= 1.0 {
                self.x = PADDLE_COLUMN_X - radius;
                self.return_from_paddle(impact, slice, curve, false, rebound_angle_degrees);
                self.bounce_this_frame = Some(BounceKind::HardImpact);
            } else {
                self.missed_by = Some(missed_player);
            }
        } else if self.vx > 0.0
            && self.missed_by.is_none()
            && self.x - radius > PADDLE_COLUMN_X + SECOND_PADDLE_WIDTH
        {
            self.missed_by = Some(if mode == GameMode::OnePlayer {
                Player::One
            } else {
                let second_impact = (self.y - second_paddle_y).abs();
                let first_impact = (self.y - right_paddle_y).abs();
                if second_impact <= first_impact {
                    Player::Two
                } else {
                    Player::One
                }
            });
        }

        if self.x - radius > GAME_WIDTH {
            if let Some(missed_player) = self.missed_by {
                let scoring_player = if mode == GameMode::OnePlayer {
                    missed_player
                } else {
                    missed_player.opponent()
                };
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

fn joystick_direction(axis_value: f32, up_pressed: bool, down_pressed: bool) -> f32 {
    let direction = axis_value;
    if direction.abs() >= 0.2 {
        direction
    } else {
        paddle_direction(down_pressed, up_pressed)
    }
}

fn poll_joystick(gilrs: Option<&mut Gilrs>) -> JoystickInput {
    let Some(gilrs) = gilrs else {
        return JoystickInput::default();
    };

    let active_gamepad = gilrs.gamepads().next().map(|(id, _)| id);
    let mut input = JoystickInput::default();
    while let Some(event) = gilrs.next_event() {
        if Some(event.id) != active_gamepad {
            continue;
        }
        if let EventType::ButtonPressed(button, _) = event.event {
            match button {
                Button::South => input.serve_pressed = true,
                Button::East => input.menu_pressed = true,
                _ => {}
            }
        }
    }

    if let Some(id) = active_gamepad {
        let gamepad = gilrs.gamepad(id);
        input.connected = gamepad.is_connected();
        if input.connected {
            input.player_one_y = gamepad
                .axis_data(Axis::LeftStickY)
                .map_or(0.0, |axis| axis.value());
            input.player_two_y = gamepad
                .axis_data(Axis::RightStickY)
                .map_or(0.0, |axis| axis.value());
            input.player_one_up = gamepad.is_pressed(Button::DPadUp);
            input.player_one_down = gamepad.is_pressed(Button::DPadDown);
            input.player_two_up = gamepad.is_pressed(Button::DPadUp);
            input.player_two_down = gamepad.is_pressed(Button::DPadDown);
            input.slice_pressed = gamepad.is_pressed(Button::West);
            input.curve_pressed = gamepad.is_pressed(Button::North);
        }
    }

    input
}

fn award_point(scores: &mut [u32; 2], player: Player, mode: GameMode) -> bool {
    scores[player.score_index()] += 1;
    match mode {
        GameMode::OnePlayer => scores[0] >= GAME_POINT_LIMIT,
        GameMode::TwoPlayer => scores[player.score_index()] >= GAME_POINT_LIMIT,
    }
}

fn auto_serve_if_ready(
    ball: &mut Ball,
    serve_mode: ServeMode,
    game_over: bool,
    speed_scale: f32,
) -> bool {
    if serve_mode == ServeMode::Automatic && !ball.moving && !game_over {
        ball.serve(speed_scale);
        true
    } else {
        false
    }
}

fn start_match(
    ball: &mut Ball,
    scores: &mut [u32; 2],
    game_over: &mut bool,
    serve_mode: ServeMode,
    speed_scale: f32,
) {
    ball.reset();
    *scores = [0; 2];
    *game_over = false;
    if serve_mode == ServeMode::Automatic {
        ball.serve(speed_scale);
    }
}

fn set_game_camera() -> (i32, i32, i32, i32) {
    let viewport = fitted_viewport(screen_width(), screen_height());
    let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, GAME_WIDTH, GAME_HEIGHT));
    camera.viewport = Some(viewport);
    set_camera(&camera);
    viewport
}

fn draw_ball(ball: &Ball, palette: Palette) {
    for (index, position) in ball.trail.iter().enumerate().rev() {
        let alpha = 0.06 * (1.0 - index as f32 / ball.trail.len().max(1) as f32);
        draw_rectangle(
            position.x - BALL_SIZE,
            position.y - BALL_SIZE,
            BALL_SIZE * 2.0,
            BALL_SIZE * 2.0,
            Color::new(
                palette.primary.r,
                palette.primary.g,
                palette.primary.b,
                alpha,
            ),
        );
    }

    for (size, alpha) in [(34.0, 0.025), (24.0, 0.06), (18.0, 0.12)] {
        draw_rectangle(
            ball.x - size / 2.0,
            ball.y - size / 2.0,
            size,
            size,
            Color::new(
                palette.primary.r,
                palette.primary.g,
                palette.primary.b,
                alpha,
            ),
        );
    }
    draw_rectangle(
        ball.x - BALL_SIZE / 2.0,
        ball.y - BALL_SIZE / 2.0,
        BALL_SIZE,
        BALL_SIZE,
        palette.primary,
    );
}

fn with_alpha(mut color: Color, opacity: f32) -> Color {
    color.a *= opacity;
    color
}

fn pcm_wav_from_fn(sample_count: usize, mut sample_at: impl FnMut(usize) -> i16) -> Vec<u8> {
    let data_size = (sample_count * 2) as u32;
    let mut wav = Vec::with_capacity(44 + data_size as usize);
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36 + data_size).to_le_bytes());
    wav.extend_from_slice(b"WAVEfmt ");
    wav.extend_from_slice(&16_u32.to_le_bytes());
    wav.extend_from_slice(&1_u16.to_le_bytes());
    wav.extend_from_slice(&1_u16.to_le_bytes());
    wav.extend_from_slice(&BOUNCE_SAMPLE_RATE.to_le_bytes());
    wav.extend_from_slice(&(BOUNCE_SAMPLE_RATE * 2).to_le_bytes());
    wav.extend_from_slice(&2_u16.to_le_bytes());
    wav.extend_from_slice(&16_u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_size.to_le_bytes());
    for sample_index in 0..sample_count {
        wav.extend_from_slice(&sample_at(sample_index).to_le_bytes());
    }

    wav
}

fn square_wave_wav() -> Vec<u8> {
    let sample_count = (BOUNCE_SAMPLE_RATE * BOUNCE_DURATION_MS / 1_000) as usize;
    let samples_per_cycle = BOUNCE_SAMPLE_RATE / BOUNCE_FREQUENCY_HZ;
    pcm_wav_from_fn(sample_count, |sample_index| {
        if sample_index as u32 % samples_per_cycle < samples_per_cycle / 2 {
            BOUNCE_AMPLITUDE
        } else {
            -BOUNCE_AMPLITUDE
        }
    })
}

fn hard_impact_wav() -> Vec<u8> {
    let sample_count = (BOUNCE_SAMPLE_RATE * IMPACT_DURATION_MS / 1_000) as usize;
    pcm_wav_from_fn(sample_count, |sample_index| {
        let progress = sample_index as f32 / sample_count as f32;
        let envelope = 1.0 - 0.25 * progress;
        let tone = IMPACT_FREQUENCIES_HZ
            .iter()
            .map(|frequency| {
                let phase = (sample_index as u64 * u64::from(*frequency)
                    % u64::from(BOUNCE_SAMPLE_RATE)) as f32
                    / BOUNCE_SAMPLE_RATE as f32;
                if phase < 0.5 {
                    0.5
                } else {
                    -0.5
                }
            })
            .sum::<f32>();
        (tone * envelope * ALERT_AMPLITUDE as f32).round() as i16
    })
}

fn descending_square_wave_wav(
    start_frequency: f32,
    end_frequency: f32,
    duration_ms: u32,
) -> Vec<u8> {
    let sample_count = (BOUNCE_SAMPLE_RATE * duration_ms / 1_000) as usize;
    let mut phase = 0.0_f32;
    pcm_wav_from_fn(sample_count, |sample_index| {
        let progress = sample_index as f32 / sample_count as f32;
        let frequency = start_frequency + (end_frequency - start_frequency) * progress;
        phase = (phase + frequency / BOUNCE_SAMPLE_RATE as f32).fract();
        let square = if phase < 0.5 { 1.0 } else { -1.0 };
        let envelope = (1.0 - progress).powf(1.4);
        (square * envelope * ALERT_AMPLITUDE as f32).round() as i16
    })
}

fn point_scored_wav() -> Vec<u8> {
    descending_square_wave_wav(
        SCORE_START_FREQUENCY_HZ,
        SCORE_END_FREQUENCY_HZ,
        SCORE_DURATION_MS,
    )
}

fn game_over_wav() -> Vec<u8> {
    let sample_count = (BOUNCE_SAMPLE_RATE * GAME_OVER_DURATION_MS / 1_000) as usize;
    let mut phase = 0.0_f32;
    pcm_wav_from_fn(sample_count, |sample_index| {
        let progress = sample_index as f32 / sample_count as f32;
        let frequency = GAME_OVER_START_FREQUENCY_HZ
            + (GAME_OVER_END_FREQUENCY_HZ - GAME_OVER_START_FREQUENCY_HZ) * progress;
        phase = (phase + frequency / BOUNCE_SAMPLE_RATE as f32).fract();
        let sawtooth = phase * 2.0 - 1.0;
        let envelope = (1.0 - progress).powf(1.2);
        (sawtooth * envelope * ALERT_AMPLITUDE as f32).round() as i16
    })
}

fn draw_scanlined_rect(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    fill_color: Color,
    line_color: Color,
    scanlines_enabled: bool,
) {
    draw_rectangle(x, y, width, height, fill_color);
    if scanlines_enabled {
        let mut line_y = y + 2.0;
        while line_y < y + height {
            draw_rectangle(x, line_y, width, 1.0, line_color);
            line_y += SCANLINE_SPACING;
        }
    }
}

fn draw_game_field(
    second_paddle_y: f32,
    right_paddle_y: f32,
    paddle_height: f32,
    ball: &Ball,
    opacity: f32,
    mode: GameMode,
    scanlines_enabled: bool,
    palette: Palette,
) {
    if scanlines_enabled {
        let mut y = 2.0;
        while y < GAME_HEIGHT {
            draw_rectangle(
                0.0,
                y,
                GAME_WIDTH,
                1.0,
                with_alpha(palette.background_scanline, opacity),
            );
            y += SCANLINE_SPACING;
        }
    }

    draw_scanlined_rect(
        0.0,
        0.0,
        LEFT_BAR_WIDTH,
        GAME_HEIGHT,
        with_alpha(palette.primary, opacity),
        with_alpha(palette.wall_scanline, opacity),
        scanlines_enabled,
    );
    let mut x = 0.0;
    while x < GAME_WIDTH {
        draw_scanlined_rect(
            x,
            0.0,
            TOP_DASH_WIDTH,
            TOP_DASH_HEIGHT,
            with_alpha(palette.primary, opacity),
            with_alpha(palette.wall_scanline, opacity),
            scanlines_enabled,
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
            with_alpha(palette.primary, opacity),
            with_alpha(palette.wall_scanline, opacity),
            scanlines_enabled,
        );
        x += TOP_DASH_WIDTH + TOP_DASH_GAP;
    }
    if mode == GameMode::TwoPlayer {
        draw_scanlined_rect(
            PADDLE_COLUMN_X,
            second_paddle_y - paddle_height / 2.0,
            SECOND_PADDLE_WIDTH,
            paddle_height,
            with_alpha(palette.secondary, opacity),
            with_alpha(palette.second_paddle_scanline, opacity),
            scanlines_enabled,
        );
    }
    draw_scanlined_rect(
        PADDLE_COLUMN_X,
        right_paddle_y - paddle_height / 2.0,
        RIGHT_MARKER_WIDTH,
        paddle_height,
        with_alpha(palette.primary, opacity),
        with_alpha(palette.paddle_scanline, opacity),
        scanlines_enabled,
    );
    if opacity < 1.0 {
        draw_rectangle(
            ball.x - BALL_SIZE / 2.0,
            ball.y - BALL_SIZE / 2.0,
            BALL_SIZE,
            BALL_SIZE,
            with_alpha(palette.primary, opacity),
        );
    } else {
        draw_ball(ball, palette);
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

fn option_slider_rects() -> [Rect; 8] {
    [
        Rect::new(110.0, 162.0, 280.0, 68.0),
        Rect::new(410.0, 162.0, 280.0, 68.0),
        Rect::new(110.0, 242.0, 280.0, 68.0),
        Rect::new(410.0, 242.0, 280.0, 68.0),
        Rect::new(110.0, 322.0, 280.0, 68.0),
        Rect::new(410.0, 322.0, 280.0, 68.0),
        Rect::new(110.0, 402.0, 280.0, 68.0),
        Rect::new(410.0, 402.0, 280.0, 68.0),
    ]
}

fn start_button_rect() -> Rect {
    Rect::new(300.0, 520.0, 200.0, 52.0)
}

fn game_over_option_rects() -> [Rect; 3] {
    [
        Rect::new(250.0, 260.0, 300.0, 48.0),
        Rect::new(250.0, 322.0, 300.0, 48.0),
        Rect::new(250.0, 384.0, 300.0, 48.0),
    ]
}

fn draw_menu_button(
    rect: Rect,
    label: &str,
    selected: bool,
    viewport: (i32, i32, i32, i32),
    palette: Palette,
) {
    let scale = viewport.2 as f32 / GAME_WIDTH;
    let rect = Rect::new(
        viewport.0 as f32 + rect.x * scale,
        viewport.1 as f32 + rect.y * scale,
        rect.w * scale,
        rect.h * scale,
    );
    let color = if selected {
        palette.primary
    } else {
        palette.secondary
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

fn slider_choice_index(mouse_x: f32, rect: Rect, choice_count: usize) -> usize {
    let last_index = choice_count.saturating_sub(1);
    if last_index == 0 {
        return 0;
    }

    let first_stop = rect.x + 42.0;
    let last_stop = rect.x + rect.w - 42.0;
    let position = (mouse_x - first_stop) / (last_stop - first_stop) * last_index as f32;
    position.round().clamp(0.0, last_index as f32) as usize
}

fn draw_option_slider(
    label: &str,
    choices: &[&str],
    selected_index: usize,
    rect: Rect,
    viewport: (i32, i32, i32, i32),
    palette: Palette,
) {
    let scale = viewport.2 as f32 / GAME_WIDTH;
    let x = viewport.0 as f32 + rect.x * scale;
    let y = viewport.1 as f32 + rect.y * scale;
    let width = rect.w * scale;
    let center_x = x + width / 2.0;
    let first_stop = x + 42.0 * scale;
    let last_stop = x + width - 42.0 * scale;
    let track_y = y + 42.0 * scale;

    draw_centered_text(
        label,
        center_x,
        y + 13.0 * scale,
        13.0 * scale,
        palette.primary,
    );
    draw_line(
        first_stop,
        track_y,
        last_stop,
        track_y,
        4.0 * scale,
        with_alpha(palette.secondary, 0.65),
    );

    let last_index = choices.len().saturating_sub(1);
    let mut selected_x = first_stop;
    for (index, choice) in choices.iter().enumerate() {
        let fraction = if last_index == 0 {
            0.0
        } else {
            index as f32 / last_index as f32
        };
        let stop_x = first_stop + (last_stop - first_stop) * fraction;
        draw_circle(
            stop_x,
            track_y,
            if index == selected_index {
                9.0 * scale
            } else {
                4.0 * scale
            },
            if index == selected_index {
                palette.primary
            } else {
                palette.secondary
            },
        );
        if index == selected_index {
            selected_x = stop_x;
        }
        draw_centered_text(
            choice,
            stop_x,
            y + 59.0 * scale,
            11.0 * scale,
            if index == selected_index {
                palette.primary
            } else {
                with_alpha(palette.secondary, 0.65)
            },
        );
    }
    draw_circle(selected_x, track_y, 4.0 * scale, palette.highlight);
}

fn draw_attraction_screen(
    mode: GameMode,
    speed_index: usize,
    size_index: usize,
    rebound_angle_index: usize,
    serve_mode: ServeMode,
    scanlines_enabled: bool,
    game_color: GameColor,
    control_mode: ControlMode,
    joystick_connected: bool,
    joystick_unavailable: bool,
    viewport: (i32, i32, i32, i32),
    palette: Palette,
    logo: &Texture2D,
) {
    let scale = viewport.2 as f32 / GAME_WIDTH;
    let center_x = viewport.0 as f32 + GAME_WIDTH / 2.0 * scale;
    let top = viewport.1 as f32;
    let logo_width = GAME_WIDTH * 0.55 * scale;
    draw_texture_ex(
        logo,
        center_x - logo_width / 2.0,
        top + 4.0 * scale,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(logo_width, logo_width * logo.height() / logo.width())),
            ..Default::default()
        },
    );
    draw_centered_text(
        "SELECT YOUR GAME",
        center_x,
        top + 154.0 * scale,
        17.0 * scale,
        palette.secondary,
    );

    let mode_index = usize::from(mode == GameMode::TwoPlayer);
    let serve_index = usize::from(serve_mode == ServeMode::Manual);
    let scanline_index = usize::from(scanlines_enabled);
    let control_index = usize::from(control_mode == ControlMode::Joystick);
    let sliders = [
        ("PLAYERS", &PLAYER_MODE_LABELS[..], mode_index),
        ("GAME SPEED", &GAME_SPEED_LABELS[..], speed_index),
        ("PADDLE SIZE", &PADDLE_SIZE_LABELS[..], size_index),
        (
            "REBOUND ANGLE",
            &REBOUND_ANGLE_LABELS[..],
            rebound_angle_index,
        ),
        ("SERVE", &SERVE_MODE_LABELS[..], serve_index),
        ("SCANLINES", &SCANLINE_LABELS[..], scanline_index),
        ("GAME COLOR", &GAME_COLOR_LABELS[..], game_color.index()),
        ("CONTROLS", &CONTROL_MODE_LABELS[..], control_index),
    ];
    for ((label, choices, selected), rect) in sliders.into_iter().zip(option_slider_rects()) {
        draw_option_slider(label, choices, selected, rect, viewport, palette);
    }

    if control_mode == ControlMode::Joystick && !joystick_connected {
        draw_centered_text(
            if joystick_unavailable {
                "JOYSTICK INPUT UNAVAILABLE"
            } else {
                "CONNECT A JOYSTICK"
            },
            center_x,
            top + 493.0 * scale,
            13.0 * scale,
            palette.secondary,
        );
    }

    let start = start_button_rect();
    draw_menu_button(start, "START GAME", true, viewport, palette);
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

fn draw_scoreboard(
    scores: [u32; 2],
    mode: GameMode,
    game_over: bool,
    viewport: (i32, i32, i32, i32),
    palette: Palette,
) {
    let scale = viewport.2 as f32 / GAME_WIDTH;
    let center_x = viewport.0 as f32 + viewport.2 as f32 / 2.0;
    let top = viewport.1 as f32;
    if mode == GameMode::OnePlayer {
        draw_centered_text(
            "MISSES",
            center_x,
            top + 36.0 * scale,
            26.0 * scale,
            palette.primary,
        );
        draw_centered_text(
            &scores[0].to_string(),
            center_x,
            top + 108.0 * scale,
            64.0 * scale,
            palette.primary,
        );
        if game_over {
            draw_centered_text(
                "GAME OVER",
                center_x,
                top + 200.0 * scale,
                36.0 * scale,
                palette.secondary,
            );
        }
        return;
    }

    draw_centered_text(
        "PLAYER 2",
        center_x - 105.0 * scale,
        top + 36.0 * scale,
        26.0 * scale,
        palette.secondary,
    );
    draw_centered_text(
        &scores[1].to_string(),
        center_x - 105.0 * scale,
        top + 108.0 * scale,
        64.0 * scale,
        palette.secondary,
    );
    draw_centered_text(
        "PLAYER 1",
        center_x + 105.0 * scale,
        top + 36.0 * scale,
        26.0 * scale,
        palette.primary,
    );
    draw_centered_text(
        &scores[0].to_string(),
        center_x + 105.0 * scale,
        top + 108.0 * scale,
        64.0 * scale,
        palette.primary,
    );
    if game_over {
        draw_centered_text(
            if scores[0] >= GAME_POINT_LIMIT {
                "PLAYER 1 WINS"
            } else {
                "PLAYER 2 WINS"
            },
            center_x,
            top + 200.0 * scale,
            36.0 * scale,
            palette.secondary,
        );
    }
}

fn draw_joystick_status(
    control_mode: ControlMode,
    joystick_connected: bool,
    joystick_unavailable: bool,
    baseline_y: f32,
    viewport: (i32, i32, i32, i32),
    palette: Palette,
) {
    if control_mode != ControlMode::Joystick || joystick_connected {
        return;
    }

    let scale = viewport.2 as f32 / GAME_WIDTH;
    let center_x = viewport.0 as f32 + viewport.2 as f32 / 2.0;
    draw_centered_text(
        if joystick_unavailable {
            "JOYSTICK INPUT UNAVAILABLE"
        } else {
            "CONNECT A JOYSTICK"
        },
        center_x,
        viewport.1 as f32 + baseline_y * scale,
        14.0 * scale,
        palette.secondary,
    );
}

fn draw_game_over_options(viewport: (i32, i32, i32, i32), palette: Palette) {
    for (rect, label) in
        game_over_option_rects()
            .into_iter()
            .zip(["PLAY AGAIN", "ATTRACTION SCREEN", "QUIT"])
    {
        draw_menu_button(rect, label, true, viewport, palette);
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let bounce_tone_data = square_wave_wav();
    let bounce_sound = load_sound_from_bytes(&bounce_tone_data)
        .await
        .expect("Failed to load the paddle and wall bounce sound");
    let impact_tone_data = hard_impact_wav();
    let impact_sound = load_sound_from_bytes(&impact_tone_data)
        .await
        .expect("Failed to load the racket and front-wall impact sound");
    let point_tone_data = point_scored_wav();
    let point_sound = load_sound_from_bytes(&point_tone_data)
        .await
        .expect("Failed to load the point-scored sound");
    let game_over_tone_data = game_over_wav();
    let game_over_sound = load_sound_from_bytes(&game_over_tone_data)
        .await
        .expect("Failed to load the game-over and reset sound");
    let attraction_logo =
        Texture2D::from_file_with_format(include_bytes!("../assets/squash-76-logo.png"), None);
    let mut second_paddle_y = GAME_HEIGHT / 2.0 - RIGHT_MARKER_HEIGHT;
    let mut right_paddle_y = GAME_HEIGHT / 2.0;
    let mut ball = Ball::new();
    let mut scores = [0; 2];
    let mut screen = Screen::Attraction;
    let mut mode = GameMode::OnePlayer;
    let mut game_over = false;
    let mut speed_index = 1;
    let mut size_index = 1;
    let mut rebound_angle_index = 0;
    let mut serve_mode = ServeMode::Automatic;
    let mut scanlines_enabled = true;
    let mut game_color = GameColor::Amber;
    let mut control_mode = ControlMode::Keyboard;
    let (mut gilrs, joystick_unavailable) = match Gilrs::new() {
        Ok(gilrs) => (Some(gilrs), false),
        Err(error) => {
            eprintln!("Joystick input unavailable: {error}");
            (None, true)
        }
    };

    loop {
        set_default_camera();
        clear_background(BLACK);
        let viewport = set_game_camera();
        let dt = get_frame_time().min(0.04);
        let paddle_height = PADDLE_SIZES[size_index];
        let joystick_input = poll_joystick(gilrs.as_mut());
        let palette = game_color.palette();

        if screen == Screen::Attraction {
            let mouse = mouse_in_game(viewport);
            if is_mouse_button_pressed(MouseButton::Left) {
                if let Some(mouse) = mouse {
                    for (index, rect) in option_slider_rects().into_iter().enumerate() {
                        if rect.contains(mouse) {
                            let choice_count = match index {
                                1 | 6 => 3,
                                _ => 2,
                            };
                            let choice = slider_choice_index(mouse.x, rect, choice_count);
                            match index {
                                0 => {
                                    mode = if choice == 0 {
                                        GameMode::OnePlayer
                                    } else {
                                        GameMode::TwoPlayer
                                    };
                                }
                                1 => speed_index = choice,
                                2 => size_index = choice,
                                3 => rebound_angle_index = choice,
                                4 => {
                                    serve_mode = if choice == 0 {
                                        ServeMode::Automatic
                                    } else {
                                        ServeMode::Manual
                                    };
                                }
                                5 => scanlines_enabled = choice == 1,
                                6 => game_color = GameColor::from_index(choice),
                                7 => {
                                    control_mode = if choice == 0 {
                                        ControlMode::Keyboard
                                    } else {
                                        ControlMode::Joystick
                                    };
                                }
                                _ => unreachable!(),
                            }
                        }
                    }
                    if start_button_rect().contains(mouse) {
                        screen = Screen::Playing;
                        start_match(
                            &mut ball,
                            &mut scores,
                            &mut game_over,
                            serve_mode,
                            GAME_SPEEDS[speed_index],
                        );
                    }
                }
            }
            draw_game_field(
                GAME_HEIGHT / 2.0 - paddle_height,
                GAME_HEIGHT / 2.0,
                paddle_height,
                &Ball::new(),
                0.2,
                mode,
                scanlines_enabled,
                palette,
            );
            set_default_camera();
            draw_attraction_screen(
                mode,
                speed_index,
                size_index,
                rebound_angle_index,
                serve_mode,
                scanlines_enabled,
                game_color,
                control_mode,
                joystick_input.connected,
                joystick_unavailable,
                viewport,
                palette,
                &attraction_logo,
            );
        } else {
            if game_over {
                if is_key_pressed(KeyCode::Q) {
                    macroquad::miniquad::window::order_quit();
                    return;
                }
                if is_key_pressed(KeyCode::R) {
                    start_match(
                        &mut ball,
                        &mut scores,
                        &mut game_over,
                        serve_mode,
                        GAME_SPEEDS[speed_index],
                    );
                    play_sound_once(&game_over_sound);
                }
                if is_mouse_button_pressed(MouseButton::Left) {
                    if let Some(mouse) = mouse_in_game(viewport) {
                        let options = game_over_option_rects();
                        if options[0].contains(mouse) {
                            start_match(
                                &mut ball,
                                &mut scores,
                                &mut game_over,
                                serve_mode,
                                GAME_SPEEDS[speed_index],
                            );
                            play_sound_once(&game_over_sound);
                        } else if options[1].contains(mouse) {
                            screen = Screen::Attraction;
                            ball.reset();
                        } else if options[2].contains(mouse) {
                            macroquad::miniquad::window::order_quit();
                            return;
                        }
                    }
                }
            }

            let top_bound = TOP_DASH_HEIGHT + paddle_height / 2.0;
            let bottom_bound = GAME_HEIGHT - TOP_DASH_HEIGHT - paddle_height / 2.0;
            if !game_over {
                let right_direction = match control_mode {
                    ControlMode::Keyboard => {
                        paddle_direction(is_key_down(KeyCode::W), is_key_down(KeyCode::S))
                    }
                    ControlMode::Joystick => joystick_direction(
                        joystick_input.player_one_y,
                        mode == GameMode::OnePlayer && joystick_input.player_one_up,
                        mode == GameMode::OnePlayer && joystick_input.player_one_down,
                    ),
                };
                right_paddle_y = (right_paddle_y + right_direction * PADDLE_SPEED * dt)
                    .clamp(top_bound, bottom_bound);
                if mode == GameMode::TwoPlayer {
                    let second_direction = match control_mode {
                        ControlMode::Keyboard => {
                            paddle_direction(is_key_down(KeyCode::Up), is_key_down(KeyCode::Down))
                        }
                        ControlMode::Joystick => joystick_direction(
                            joystick_input.player_two_y,
                            joystick_input.player_two_up,
                            joystick_input.player_two_down,
                        ),
                    };
                    second_paddle_y = (second_paddle_y + second_direction * PADDLE_SPEED * dt)
                        .clamp(top_bound, bottom_bound);
                }
            }
            let serve_pressed = match control_mode {
                ControlMode::Keyboard => is_key_pressed(KeyCode::Space),
                ControlMode::Joystick => joystick_input.serve_pressed,
            };
            if serve_pressed && !ball.moving && !game_over && serve_mode == ServeMode::Manual {
                ball.serve(GAME_SPEEDS[speed_index]);
            }
            let menu_pressed = is_key_pressed(KeyCode::Escape)
                || (control_mode == ControlMode::Joystick && joystick_input.menu_pressed);
            if menu_pressed {
                screen = Screen::Attraction;
                ball.reset();
            }
            let slice_pressed = match control_mode {
                ControlMode::Keyboard => is_key_down(KeyCode::A),
                ControlMode::Joystick => joystick_input.slice_pressed,
            };
            let curve_pressed = match control_mode {
                ControlMode::Keyboard => is_key_down(KeyCode::D),
                ControlMode::Joystick => joystick_input.curve_pressed,
            };
            if let Some(player) = ball.update(
                dt,
                second_paddle_y,
                right_paddle_y,
                paddle_height,
                slice_pressed,
                curve_pressed,
                mode,
                REBOUND_ANGLE_DEGREES[rebound_angle_index],
            ) {
                game_over = award_point(&mut scores, player, mode);
                play_sound_once(if game_over {
                    &game_over_sound
                } else {
                    &point_sound
                });
            }
            match ball.bounce_this_frame {
                Some(BounceKind::SideWall) => play_sound_once(&bounce_sound),
                Some(BounceKind::HardImpact) => play_sound_once(&impact_sound),
                None => {}
            }
            if screen == Screen::Playing {
                auto_serve_if_ready(&mut ball, serve_mode, game_over, GAME_SPEEDS[speed_index]);
            }
            draw_game_field(
                second_paddle_y,
                right_paddle_y,
                paddle_height,
                &ball,
                1.0,
                mode,
                scanlines_enabled,
                palette,
            );
        }
        set_default_camera();
        if screen == Screen::Playing {
            draw_scoreboard(scores, mode, game_over, viewport, palette);
            draw_joystick_status(
                control_mode,
                joystick_input.connected,
                joystick_unavailable,
                160.0,
                viewport,
                palette,
            );
            if game_over {
                draw_game_over_options(viewport, palette);
            }
        }
        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wav_sample_count(wav: &[u8]) -> usize {
        (u32::from_le_bytes(wav[40..44].try_into().unwrap()) / 2) as usize
    }

    fn sign_changes(wav: &[u8], start: usize, end: usize) -> usize {
        (start..end - 1)
            .filter(|&sample_index| {
                let current = i16::from_le_bytes(
                    wav[44 + sample_index * 2..46 + sample_index * 2]
                        .try_into()
                        .unwrap(),
                );
                let next = i16::from_le_bytes(
                    wav[46 + sample_index * 2..48 + sample_index * 2]
                        .try_into()
                        .unwrap(),
                );
                (current >= 0) != (next >= 0)
            })
            .count()
    }

    #[test]
    fn bounce_sound_is_a_60ms_1200hz_mono_square_wave() {
        let wav = square_wave_wav();
        let sample_count = (BOUNCE_SAMPLE_RATE * BOUNCE_DURATION_MS / 1_000) as usize;

        assert_eq!(&wav[..4], b"RIFF");
        assert_eq!(&wav[8..12], b"WAVE");
        assert_eq!(&wav[36..40], b"data");
        assert_eq!(
            u32::from_le_bytes(wav[40..44].try_into().unwrap()),
            (sample_count * 2) as u32
        );
        assert_eq!(wav.len(), 44 + sample_count * 2);
        assert_eq!(
            i16::from_le_bytes(wav[44..46].try_into().unwrap()),
            BOUNCE_AMPLITUDE
        );
        assert_eq!(
            i16::from_le_bytes(wav[44 + 20 * 2..46 + 20 * 2].try_into().unwrap()),
            -BOUNCE_AMPLITUDE
        );
    }

    #[test]
    fn impact_sound_is_distinct_and_sixty_milliseconds() {
        let bounce = square_wave_wav();
        let impact = hard_impact_wav();

        assert_eq!(
            wav_sample_count(&impact),
            (BOUNCE_SAMPLE_RATE * 60 / 1_000) as usize
        );
        assert_ne!(impact, bounce);
    }

    #[test]
    fn score_and_game_over_sounds_sweep_down_for_their_expected_durations() {
        let point = point_scored_wav();
        let game_over = game_over_wav();
        let window_samples = (BOUNCE_SAMPLE_RATE * 60 / 1_000) as usize;

        assert_eq!(
            wav_sample_count(&point),
            (BOUNCE_SAMPLE_RATE * SCORE_DURATION_MS / 1_000) as usize
        );
        assert_eq!(
            wav_sample_count(&game_over),
            (BOUNCE_SAMPLE_RATE * GAME_OVER_DURATION_MS / 1_000) as usize
        );
        assert!(
            sign_changes(&point, 0, window_samples)
                > sign_changes(
                    &point,
                    wav_sample_count(&point) - window_samples,
                    wav_sample_count(&point),
                )
        );
        assert!(
            sign_changes(&game_over, 0, window_samples)
                > sign_changes(
                    &game_over,
                    wav_sample_count(&game_over) - window_samples,
                    wav_sample_count(&game_over),
                )
        );
    }

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
            bounce_this_frame: None,
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
                GameMode::TwoPlayer,
                REBOUND_ANGLE_DEGREES[0],
            ),
            None
        );

        assert!(ball.vx < 0.0);
        assert!(ball.vy > 0.0);
        assert_eq!(ball.bounce_this_frame, Some(BounceKind::SideWall));
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
            bounce_this_frame: None,
            trail: Vec::new(),
        };

        assert_eq!(
            ball.update(
                0.04,
                paddle_y,
                paddle_y,
                RIGHT_MARKER_HEIGHT,
                false,
                false,
                GameMode::TwoPlayer,
                REBOUND_ANGLE_DEGREES[0],
            ),
            None
        );

        assert!(ball.moving);
        assert!(ball.vx < 0.0);
        assert!(ball.vx.hypot(ball.vy) > 300.0);
        assert!(ball.vx.hypot(ball.vy) <= MAX_BALL_SPEED);
        assert_eq!(ball.bounce_this_frame, Some(BounceKind::HardImpact));
    }

    #[test]
    fn selected_paddle_size_changes_the_collision_window() {
        let paddle_x = WINDOW_WIDTH as f32 - RIGHT_MARKER_OFFSET;
        let paddle_y = WINDOW_HEIGHT as f32 / 2.0;
        let mut normal_paddle_hit = Ball {
            x: paddle_x - BALL_SIZE,
            y: paddle_y + 30.0,
            vx: 300.0,
            vy: 0.0,
            spin: 0.0,
            speed_scale: 1.0,
            moving: true,
            missed_by: None,
            bounce_this_frame: None,
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
            bounce_this_frame: None,
            trail: Vec::new(),
        };

        let other_paddle_y = paddle_y - RIGHT_MARKER_HEIGHT;
        normal_paddle_hit.update(
            0.04,
            other_paddle_y,
            paddle_y,
            PADDLE_SIZES[1],
            false,
            false,
            GameMode::TwoPlayer,
            REBOUND_ANGLE_DEGREES[0],
        );
        small_paddle_miss.update(
            0.04,
            other_paddle_y,
            paddle_y,
            PADDLE_SIZES[0],
            false,
            false,
            GameMode::TwoPlayer,
            REBOUND_ANGLE_DEGREES[0],
        );

        assert!(normal_paddle_hit.vx < 0.0);
        assert_eq!(small_paddle_miss.missed_by, Some(Player::One));
    }

    #[test]
    fn paddle_hit_position_controls_pong_style_return_angle() {
        let mut center_hit = Ball::new();
        center_hit.vx = 300.0;
        center_hit.return_from_paddle(0.0, false, false, false, REBOUND_ANGLE_DEGREES[0]);
        assert!(center_hit.vx < 0.0);
        assert!(center_hit.vy.abs() < f32::EPSILON);

        let mut edge_hit = Ball::new();
        edge_hit.vx = 300.0;
        edge_hit.return_from_paddle(2.0, false, false, false, REBOUND_ANGLE_DEGREES[1]);
        let edge_angle = edge_hit.vy.atan2(-edge_hit.vx);
        assert!((edge_angle - REBOUND_ANGLE_DEGREES[1].to_radians()).abs() < 0.001);

        let mut shallow_hit = Ball::new();
        shallow_hit.vx = 300.0;
        shallow_hit.return_from_paddle(1.0, false, false, false, REBOUND_ANGLE_DEGREES[0]);
        let shallow_angle = shallow_hit.vy.atan2(-shallow_hit.vx);
        assert!((shallow_angle - REBOUND_ANGLE_DEGREES[0].to_radians()).abs() < 0.001);
        assert!(edge_angle > shallow_angle);
    }

    #[test]
    fn slice_is_fast_and_stays_flat() {
        let mut ball = Ball::new();
        ball.vx = 300.0;
        ball.return_from_paddle(1.0, true, false, false, REBOUND_ANGLE_DEGREES[1]);

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
            bounce_this_frame: None,
            trail: Vec::new(),
        };

        assert_eq!(
            ball.update(
                0.04,
                paddle_y,
                paddle_y,
                RIGHT_MARKER_HEIGHT,
                false,
                true,
                GameMode::TwoPlayer,
                REBOUND_ANGLE_DEGREES[0],
            ),
            None
        );
        let curved_vy = ball.vy;
        assert_eq!(
            ball.update(
                0.04,
                paddle_y,
                paddle_y,
                RIGHT_MARKER_HEIGHT,
                false,
                false,
                GameMode::TwoPlayer,
                REBOUND_ANGLE_DEGREES[0],
            ),
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
            bounce_this_frame: None,
            trail: Vec::new(),
        };

        assert_eq!(
            ball.update(
                0.04,
                paddle_y,
                first_paddle_y,
                RIGHT_MARKER_HEIGHT,
                false,
                false,
                GameMode::TwoPlayer,
                REBOUND_ANGLE_DEGREES[0],
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
            bounce_this_frame: None,
            trail: Vec::new(),
        };

        assert_eq!(
            ball.update(
                0.04,
                second_paddle_y,
                first_paddle_y,
                RIGHT_MARKER_HEIGHT,
                false,
                false,
                GameMode::TwoPlayer,
                REBOUND_ANGLE_DEGREES[0],
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
                GameMode::TwoPlayer,
                REBOUND_ANGLE_DEGREES[0],
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

    #[test]
    fn joystick_direction_uses_stick_with_dpad_deadzone_fallback() {
        assert_eq!(joystick_direction(0.75, false, false), 0.75);
        assert_eq!(joystick_direction(-0.75, false, false), -0.75);
        assert_eq!(joystick_direction(0.1, true, false), -1.0);
        assert_eq!(joystick_direction(0.1, false, false), 0.0);
    }

    #[test]
    fn game_color_options_provide_distinct_palettes() {
        let green = GameColor::from_index(0).palette();
        let amber = GameColor::from_index(1).palette();
        let white = GameColor::from_index(2).palette();

        assert_ne!(green.primary.r, amber.primary.r);
        assert_ne!(white.primary.r, amber.primary.r);
        assert!(matches!(GameColor::from_index(1), GameColor::Amber));
    }

    #[test]
    fn slider_clicks_select_nearest_two_or_three_position() {
        let rect = Rect::new(0.0, 0.0, 280.0, 88.0);
        assert_eq!(slider_choice_index(42.0, rect, 2), 0);
        assert_eq!(slider_choice_index(238.0, rect, 2), 1);
        assert_eq!(slider_choice_index(140.0, rect, 3), 1);
        assert_eq!(slider_choice_index(238.0, rect, 3), 2);
    }

    #[test]
    fn only_automatic_serve_restarts_a_stopped_nonfinal_ball() {
        let mut ball = Ball::new();
        assert!(auto_serve_if_ready(
            &mut ball,
            ServeMode::Automatic,
            false,
            GAME_SPEEDS[0],
        ));
        assert!(ball.moving);
        assert_eq!(ball.speed_scale, GAME_SPEEDS[0]);

        ball.reset();
        assert!(!auto_serve_if_ready(
            &mut ball,
            ServeMode::Manual,
            false,
            GAME_SPEEDS[0],
        ));
        assert!(!auto_serve_if_ready(
            &mut ball,
            ServeMode::Automatic,
            true,
            GAME_SPEEDS[0],
        ));
        assert!(!ball.moving);
    }

    #[test]
    fn starting_match_resets_scores_and_respects_serve_mode() {
        let mut ball = Ball::new();
        let mut scores = [GAME_POINT_LIMIT, 0];
        let mut game_over = true;

        start_match(
            &mut ball,
            &mut scores,
            &mut game_over,
            ServeMode::Manual,
            GAME_SPEEDS[1],
        );
        assert_eq!(scores, [0; 2]);
        assert!(!game_over);
        assert!(!ball.moving);

        scores[0] = 3;
        game_over = true;
        start_match(
            &mut ball,
            &mut scores,
            &mut game_over,
            ServeMode::Automatic,
            GAME_SPEEDS[1],
        );
        assert_eq!(scores, [0; 2]);
        assert!(!game_over);
        assert!(ball.moving);
    }

    #[test]
    fn single_player_ignores_the_second_paddle() {
        let first_paddle_y = WINDOW_HEIGHT as f32 / 2.0;
        let second_paddle_y = first_paddle_y - RIGHT_MARKER_HEIGHT;
        let mut ball = Ball {
            x: PADDLE_COLUMN_X - BALL_SIZE,
            y: second_paddle_y,
            vx: 300.0,
            vy: 0.0,
            spin: 0.0,
            speed_scale: 1.0,
            moving: true,
            missed_by: None,
            bounce_this_frame: None,
            trail: Vec::new(),
        };

        ball.update(
            0.04,
            second_paddle_y,
            first_paddle_y,
            RIGHT_MARKER_HEIGHT,
            false,
            false,
            GameMode::OnePlayer,
            REBOUND_ANGLE_DEGREES[0],
        );

        assert_eq!(ball.missed_by, Some(Player::One));
        assert!(ball.vx > 0.0);
    }

    #[test]
    fn single_player_counts_misses_and_ends_at_fifteen() {
        let mut scores = [0; 2];

        for miss in 1..=GAME_POINT_LIMIT {
            let game_over = award_point(&mut scores, Player::One, GameMode::OnePlayer);
            assert_eq!(scores[0], miss);
            assert_eq!(game_over, miss == GAME_POINT_LIMIT);
        }
    }

    #[test]
    fn two_player_match_ends_when_either_player_reaches_fifteen() {
        for player in [Player::One, Player::Two] {
            let mut scores = [0; 2];

            for point in 1..=GAME_POINT_LIMIT {
                let game_over = award_point(&mut scores, player, GameMode::TwoPlayer);
                assert_eq!(scores[player.score_index()], point);
                assert_eq!(game_over, point == GAME_POINT_LIMIT);
            }
        }
    }
}
