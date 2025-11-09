use macroquad::prelude::*;
use std::collections::VecDeque;

const TILE_SIZE: f32 = 20.0;
const GRID_WIDTH: usize = 20;
const GRID_HEIGHT: usize = 20;

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    body: VecDeque<(usize, usize)>,
    dir: Direction,
    next_dir: Direction,
}

struct Game {
    snake: Snake,
    food: (usize, usize),
    score: u32,
    high_score: u32,
    game_over: bool,
    paused: bool,
    frame_time: f64,
    speed: f64,
    loading_progress: f32,
}

impl Game {
    fn new() -> Self {
        let mut body = VecDeque::new();
        body.push_back((10, 10));
        body.push_back((9, 10));
        Game {
            snake: Snake {
                body,
                dir: Direction::Right,
                next_dir: Direction::Right,
            },
            food: (15, 15),
            score: 0,
            high_score: 0,
            game_over: false,
            paused: false,
            frame_time: 0.0,
            speed: 0.12,
            loading_progress: 0.0,
        }
    }

    fn reset(&mut self) {
        self.high_score = self.high_score.max(self.score);
        self.score = 0;
        let mut body = VecDeque::new();
        body.push_back((10, 10));
        body.push_back((9, 10));
        self.snake = Snake {
            body,
            dir: Direction::Right,
            next_dir: Direction::Right,
        };
        self.food = (
            rand::gen_range(3, GRID_WIDTH - 3),
            rand::gen_range(3, GRID_HEIGHT - 3),
        );
        self.game_over = false;
        self.paused = false;
        self.frame_time = 0.0;
        self.speed = 0.12;
    }

    fn handle_input(&mut self) {
        // Keyboard inputs for desktop
        if is_key_pressed(KeyCode::P) {
            self.paused = !self.paused;
        }

        if self.game_over {
            if is_key_pressed(KeyCode::Space) {
                self.reset();
            }
            return;
        }

        if is_key_pressed(KeyCode::Up) && self.snake.dir != Direction::Down {
            self.snake.next_dir = Direction::Up;
        } else if is_key_pressed(KeyCode::Down) && self.snake.dir != Direction::Up {
            self.snake.next_dir = Direction::Down;
        } else if is_key_pressed(KeyCode::Left) && self.snake.dir != Direction::Right {
            self.snake.next_dir = Direction::Left;
        } else if is_key_pressed(KeyCode::Right) && self.snake.dir != Direction::Left {
            self.snake.next_dir = Direction::Right;
        }

        // Touch/mouse inputs for mobile/desktop testing
        let touches = touches();
        let mouse_pressed = is_mouse_button_pressed(MouseButton::Left);
        let mouse_pos = mouse_position();

        let mut input_pos: Option<Vec2> = None;
        let mut input_started = false;

        if !touches.is_empty() {
            for touch in touches {
                if touch.phase == TouchPhase::Started {
                    input_pos = Some(touch.position);
                    input_started = true;
                    break;
                }
            }
        } else if mouse_pressed {
            input_pos = Some(vec2(mouse_pos.0, mouse_pos.1));
            input_started = true;
        }

        if let Some(pos) = input_pos {
            if self.loading_progress < 100.0 {
                return;
            }

            let sw = screen_width();
            let sh = screen_height();

            // Pause button area
            let pause_rect = Rect::new(sw - 60.0, 10.0, 50.0, 50.0);
            if pause_rect.contains(pos) && input_started {
                self.paused = !self.paused;
                return;
            }

            if self.game_over {
                if input_started {
                    self.reset();
                }
                return;
            }

            if self.paused {
                return;
            }

            // Direction based on touch/mouse position relative to center
            let center_x = sw / 2.0;
            let center_y = sh / 2.0;
            let dx = pos.x - center_x;
            let dy = pos.y - center_y;

            let new_dir = if dx.abs() > dy.abs() {
                if dx > 0.0 {
                    Direction::Right
                } else {
                    Direction::Left
                }
            } else {
                if dy > 0.0 {
                    Direction::Down
                } else {
                    Direction::Up
                }
            };

            let opposite = match self.snake.dir {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
            };

            if new_dir != opposite {
                self.snake.next_dir = new_dir;
            }
        }
    }

    fn update(&mut self, dt: f64) {
        if self.loading_progress < 100.0 {
            self.loading_progress = (self.loading_progress + (dt * 33.33) as f32).min(100.0);
            return;
        }

        if self.paused || self.game_over {
            return;
        }

        self.frame_time += dt;
        if self.frame_time < self.speed {
            return;
        }
        self.frame_time = 0.0;

        self.snake.dir = self.snake.next_dir;

        let (hx, hy) = self.snake.body[0];
        let (dx, dy): (i32, i32) = match self.snake.dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        let nx_i32 = hx as i32 + dx;
        let ny_i32 = hy as i32 + dy;
        if nx_i32 < 0 || nx_i32 >= GRID_WIDTH as i32 || ny_i32 < 0 || ny_i32 >= GRID_HEIGHT as i32 {
            self.game_over = true;
            return;
        }
        let nx = nx_i32 as usize;
        let ny = ny_i32 as usize;
        if self.snake.body.contains(&(nx, ny)) {
            self.game_over = true;
            return;
        }

        self.snake.body.push_front((nx, ny));

        if (nx, ny) == self.food {
            self.score += 1;
            self.speed = (self.speed * 0.94).max(0.06);
            self.food = (
                rand::gen_range(1, GRID_WIDTH - 1),
                rand::gen_range(1, GRID_HEIGHT - 1),
            );
            while self.snake.body.contains(&self.food) {
                self.food = (
                    rand::gen_range(1, GRID_WIDTH - 1),
                    rand::gen_range(1, GRID_HEIGHT - 1),
                );
            }
        } else {
            self.snake.body.pop_back();
        }
    }

    fn draw(&self) {
        if self.loading_progress < 100.0 {
            self.draw_loading();
            return;
        }

        let time = get_time();
        let pulse = ((time * 4.0) as f32).sin() * 0.08 + 1.0;

        // Dark forest background
        clear_background(Color::new(0.02, 0.08, 0.02, 1.0));

        // Grid
        for i in 1..GRID_WIDTH {
            let x = i as f32 * TILE_SIZE;
            draw_line(
                x,
                0.0,
                x,
                screen_height(),
                0.5,
                Color::new(0.1, 0.2, 0.1, 0.5),
            );
        }
        for i in 1..GRID_HEIGHT {
            let y = i as f32 * TILE_SIZE;
            draw_line(
                0.0,
                y,
                screen_width(),
                y,
                0.5,
                Color::new(0.1, 0.2, 0.1, 0.5),
            );
        }

        self.draw_snake();
        self.draw_food(pulse);

        // UI
        let score_text = format!("Score: {}  High: {}", self.score, self.high_score);
        draw_text(score_text.as_str(), 10.0, 35.0, 28.0, WHITE);

        // Pause button
        let sw = screen_width();
        draw_rectangle(sw - 60.0, 10.0, 50.0, 50.0, GRAY);
        draw_text("II", sw - 50.0, 40.0, 40.0, BLACK); // Pause symbol

        if self.paused {
            let msg = "PAUSED - Tap Pause to Resume";
            let width = measure_text(msg, None, 40u16, 1.0).width;
            draw_text(
                msg,
                screen_width() / 2.0 - width / 2.0,
                screen_height() / 2.0,
                40.0,
                YELLOW,
            );
        }

        if self.game_over {
            let msg = format!(
                "GAME OVER!\nScore: {}\nHigh: {}\nTap anywhere to restart",
                self.score, self.high_score
            );
            let font_size: f32 = 32.0;
            let lines = msg.lines().count() as f32;
            let mut y_offset = screen_height() / 2.0 - (lines * font_size / 2.0);
            for line in msg.lines() {
                let width = measure_text(line, None, font_size as u16, 1.0).width;
                draw_text(
                    line,
                    screen_width() / 2.0 - width / 2.0,
                    y_offset,
                    font_size,
                    YELLOW,
                );
                y_offset += font_size * 1.2;
            }
        }
    }

    fn draw_loading(&self) {
        clear_background(Color::new(0.02, 0.08, 0.02, 1.0));

        // Title with shadow for style
        let title = "KINGMAX GAME";
        let font_size = 60.0;
        let width = measure_text(title, None, font_size as u16, 1.0).width;
        let title_x = screen_width() / 2.0 - width / 2.0;
        let title_y = screen_height() / 3.0;

        // Shadow
        draw_text(title, title_x + 2.0, title_y + 2.0, font_size, BLACK);
        // Main text
        draw_text(title, title_x, title_y, font_size, YELLOW);

        // Loading bar
        let bar_width = screen_width() * 0.6;
        let bar_height = 30.0;
        let bar_x = screen_width() / 2.0 - bar_width / 2.0;
        let bar_y = screen_height() / 2.0;

        // Outline with thicker lines for style
        draw_rectangle_lines(
            bar_x - 2.0,
            bar_y - 2.0,
            bar_width + 4.0,
            bar_height + 4.0,
            4.0,
            DARKGREEN,
        );

        // Fill background
        draw_rectangle(
            bar_x,
            bar_y,
            bar_width,
            bar_height,
            Color::new(0.1, 0.1, 0.1, 1.0),
        );

        // Progress fill with gradient effect (simple two-color blend)
        let fill_width = bar_width * self.loading_progress / 100.0;
        draw_rectangle(bar_x, bar_y, fill_width, bar_height, GREEN);
        draw_rectangle(bar_x, bar_y, fill_width / 2.0, bar_height, LIME); // Lighter overlay for gradient feel

        // Percentage text
        let perc = format!("Loading {}%", self.loading_progress as i32);
        let perc_font_size = 30.0;
        let perc_width = measure_text(&perc, None, perc_font_size as u16, 1.0).width;
        draw_text(
            &perc,
            screen_width() / 2.0 - perc_width / 2.0,
            bar_y + bar_height + 40.0,
            perc_font_size,
            WHITE,
        );
    }

    fn draw_food(&self, pulse: f32) {
        let x = (self.food.0 as f32 + 0.5) * TILE_SIZE;
        let y = (self.food.1 as f32 + 0.5) * TILE_SIZE;
        let r = TILE_SIZE * 0.4 * pulse;

        // Apple shine layers
        draw_circle(x - r * 0.25, y - r * 0.35, r * 0.2, WHITE);
        draw_circle(x, y, r, RED);

        // Leaf
        let leaf_x = x + r * 0.5;
        let leaf_y = y - r * 0.7;
        draw_triangle(
            vec2(leaf_x, leaf_y),
            vec2(leaf_x + TILE_SIZE * 0.12, leaf_y - TILE_SIZE * 0.1),
            vec2(leaf_x + TILE_SIZE * 0.12, leaf_y + TILE_SIZE * 0.1),
            DARKGREEN,
        );

        // Stem
        let brown = Color::new(0.55, 0.27, 0.07, 1.0);
        draw_line(
            leaf_x - TILE_SIZE * 0.06,
            leaf_y + TILE_SIZE * 0.06,
            x + r * 0.1,
            y - r * 0.55,
            3.0,
            brown,
        );
    }

    fn draw_snake(&self) {
        if self.snake.body.is_empty() {
            return;
        }

        let time = get_time() as f32;
        let body_len = self.snake.body.len() as f32;

        for (i, &(gx, gy)) in self.snake.body.iter().enumerate() {
            let cx = (gx as f32 + 0.5) * TILE_SIZE;
            let cy = (gy as f32 + 0.5) * TILE_SIZE;

            let taper = (1.0 - (i as f32 / body_len) * 0.25).max(0.65);
            let mut seg_radius = TILE_SIZE * 0.38 * taper;
            if i == 0 {
                seg_radius *= 1.15; // Larger head
            }

            let green_intensity = if i == 0 {
                0.75
            } else {
                0.45 + 0.25 * (1.0 - i as f32 / body_len)
            };
            let color = Color::new(0.0, green_intensity, 0.0, 1.0);

            // Shadow
            draw_circle(
                cx + 1.5,
                cy + 1.5,
                seg_radius,
                Color::new(0.0, 0.0, 0.0, 0.4),
            );

            // Main body
            draw_circle(cx, cy, seg_radius, color);

            // Shine highlight
            draw_circle(
                cx - seg_radius * 0.25,
                cy - seg_radius * 0.35,
                seg_radius * 0.13,
                Color::new(1.0, 1.0, 1.0, 0.8),
            );

            if i == 0 {
                // Head eyes
                let eye_size = seg_radius * 0.22;
                let pupil_size = eye_size * 0.45;
                let eye_dist_x = seg_radius * 0.35;
                let eye_dist_y = seg_radius * 0.25;

                let eye1_x = cx - eye_dist_x * 0.6;
                let eye1_y = cy - eye_dist_y;
                let eye2_x = cx + eye_dist_x * 0.6;
                let eye2_y = cy - eye_dist_y;

                draw_circle(eye1_x, eye1_y, eye_size, WHITE);
                draw_circle(eye2_x, eye2_y, eye_size, WHITE);
                draw_circle(eye1_x + 1.0, eye1_y, pupil_size, BLACK);
                draw_circle(eye2_x + 1.0, eye2_y, pupil_size, BLACK);

                // Tongue (blinking)
                if (time % 1.2) < 0.4 {
                    let dir_vec = match self.snake.dir {
                        Direction::Up => vec2(0.0, -1.0),
                        Direction::Down => vec2(0.0, 1.0),
                        Direction::Left => vec2(-1.0, 0.0),
                        Direction::Right => vec2(1.0, 0.0),
                    };
                    let wiggle = vec2(
                        ((time * 8.0) as f32).sin() * 1.5,
                        ((time * 10.0) as f32).sin() * 0.8,
                    );
                    let tongue_start = vec2(cx, cy) + dir_vec * seg_radius * 0.35;
                    let tongue_end = tongue_start + dir_vec * seg_radius * 1.3 + wiggle;
                    draw_line(
                        tongue_start.x,
                        tongue_start.y,
                        tongue_end.x,
                        tongue_end.y,
                        4.0,
                        Color::new(1.0, 0.3, 0.3, 1.0),
                    );
                }
            } else {
                // Body scales
                let scale_count = 5;
                for s in 0..scale_count {
                    let angle =
                        (s as f32 / scale_count as f32) * std::f32::consts::PI * 2.0 + time * 0.5;
                    let scale_dist = seg_radius * 0.65;
                    let sx = cx + angle.cos() * scale_dist;
                    let sy = cy + angle.sin() * (scale_dist * 0.6);
                    let scale_color = Color::new(0.0, 0.35, 0.0, 1.0);
                    draw_line(cx, cy, sx, sy, 2.2, scale_color);
                }
            }
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Realistic Snake Game".to_string(),
        window_width: (GRID_WIDTH as f32 * TILE_SIZE) as i32,
        window_height: (GRID_HEIGHT as f32 * TILE_SIZE) as i32,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        game.handle_input();
        game.update(get_frame_time() as f64);
        game.draw();
        next_frame().await;
    }
}
