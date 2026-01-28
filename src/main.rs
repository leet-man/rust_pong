use macroquad::prelude::*;

//# Constants
    const PADDLE_WIDTH: f32 = 15.0;
    const PADDLE_HEIGHT: f32 = 80.0;
    const PADDLE_SPEED: f32 = 400.0;

    const AI_PADDLE_SPEED: f32 = PADDLE_SPEED;
    const AI_REACTION_TIME_MIN: f32 = 0.01;
    const AI_REACTION_TIME_MAX: f32 = 0.09;

    const BALL_SIZE: f32 = 12.0;
    const BALL_SPEED: f32 = 350.0;

//# Data structures for paddles and ball
struct Paddle {
    x: f32,
    y: f32,
}

struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

//# Window configuration
fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Pong".to_string(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

//# Main function
#[macroquad::main(window_conf)]
async fn main() {
    let screen_w = screen_width();
    let screen_h = screen_height();

    //## Mode selection
    let mut single_player = true;
    loop {
        clear_background(BLACK);
        draw_text("Press 1 for Single Player", screen_w / 2.0 - 180.0, screen_h / 2.0 - 20.0, 32.0, WHITE);
        draw_text("Press 2 for Two Player", screen_w / 2.0 - 160.0, screen_h / 2.0 + 20.0, 32.0, WHITE);
        if is_key_pressed(KeyCode::Key1) {
            single_player = true;
            break;
        }
        if is_key_pressed(KeyCode::Key2) {
            single_player = false;
            break;
        }
        next_frame().await;
    }

    //## Initial game state
    let mut left = Paddle {
        x: 40.0,
        y: screen_h / 2.0 - PADDLE_HEIGHT / 2.0,
    };

    let mut right = Paddle {
        x: screen_w - 40.0 - PADDLE_WIDTH,
        y: screen_h / 2.0 - PADDLE_HEIGHT / 2.0,
    };

    let mut ball = Ball {
        x: screen_w / 2.0,
        y: screen_h / 2.0,
        vx: BALL_SPEED,
        vy: BALL_SPEED * 0.6,
    };

    let mut left_score = 0;
    let mut right_score = 0;
    let mut ai_timer = 0.0;
    let mut ai_reaction_time = macroquad::rand::gen_range(AI_REACTION_TIME_MIN, AI_REACTION_TIME_MAX);
    let mut ai_offset = macroquad::rand::gen_range(-20.0, 20.0);

    //## Game loop
    loop {
        let dt = get_frame_time();

        //### Left paddle control
        if single_player {
            // AI Control for left paddle
            ai_timer += dt;
            if ball.vx < 0.0 {
                if ai_timer >= ai_reaction_time {
                    let target = ball.y + BALL_SIZE / 2.0 + ai_offset;
                    let paddle_center = left.y + PADDLE_HEIGHT / 2.0;
                    let diff = target - paddle_center;
                    let threshold = 6.0;

                    if diff.abs() > threshold {
                        if diff > 0.0 {
                            left.y += AI_PADDLE_SPEED * dt;
                        } else {
                            left.y -= AI_PADDLE_SPEED * dt;
                        }
                    }
                    ai_timer = 0.0;
                    ai_reaction_time = macroquad::rand::gen_range(AI_REACTION_TIME_MIN, AI_REACTION_TIME_MAX);
                    ai_offset = macroquad::rand::gen_range(-20.0, 20.0);
                }
            }
        } else {
            // Two-player: W/S for left paddle
            if is_key_down(KeyCode::W) {
                left.y -= PADDLE_SPEED * dt;
            }
            if is_key_down(KeyCode::S) {
                left.y += PADDLE_SPEED * dt;
            }
        }

        //### Player Control for right paddle
        if is_key_down(KeyCode::Up) {
            right.y -= PADDLE_SPEED * dt;
        }
        if is_key_down(KeyCode::Down) {
            right.y += PADDLE_SPEED * dt;
        }

        //### Clamp paddles to screen
        left.y = left.y.clamp(0.0, screen_h - PADDLE_HEIGHT);
        right.y = right.y.clamp(0.0, screen_h - PADDLE_HEIGHT);

        //### Update ball
        ball.x += ball.vx * dt;
        ball.y += ball.vy * dt;

        //### Ball collision with top and bottom
        if ball.y <= 0.0 {
            ball.y = 0.0;
            ball.vy = -ball.vy;
        }
        if ball.y + BALL_SIZE >= screen_h {
            ball.y = screen_h - BALL_SIZE;
            ball.vy = -ball.vy;
        }

        //### Rects for collision
        let left_rect = Rect::new(left.x, left.y, PADDLE_WIDTH, PADDLE_HEIGHT);
        let right_rect = Rect::new(right.x, right.y, PADDLE_WIDTH, PADDLE_HEIGHT);
        let ball_rect = Rect::new(ball.x, ball.y, BALL_SIZE, BALL_SIZE);

        //### Paddle collisions
        if ball_rect.overlaps(&left_rect) && ball.vx < 0.0 {
            ball.x = left.x + PADDLE_WIDTH;
            ball.vx = -ball.vx;
        }
        if ball_rect.overlaps(&right_rect) && ball.vx > 0.0 {
            ball.x = right.x - BALL_SIZE;
            ball.vx = -ball.vx;
        }

        //### Scoring
        if ball.x + BALL_SIZE < 0.0 {
            right_score += 1;
            reset_ball(&mut ball, screen_w, screen_h, 1.0);
        }
        if ball.x > screen_w {
            left_score += 1;
            reset_ball(&mut ball, screen_w, screen_h, -1.0);
        }

        //### Drawing
        clear_background(BLACK);

        //### Middle line
        for i in 0..20 {
            if i % 2 == 0 {
                draw_rectangle(
                    screen_w / 2.0 - 2.0,
                    i as f32 * (screen_h / 20.0),
                    4.0,
                    screen_h / 20.0 - 4.0,
                    GRAY,
                );
            }
        }

        //### Paddles
        draw_rectangle(left.x, left.y, PADDLE_WIDTH, PADDLE_HEIGHT, WHITE);
        draw_rectangle(right.x, right.y, PADDLE_WIDTH, PADDLE_HEIGHT, WHITE);

        //### Ball
        draw_rectangle(ball.x, ball.y, BALL_SIZE, BALL_SIZE, WHITE);

        //### Score
        let score_text = format!("{}   {}", left_score, right_score);
        let text_dim = measure_text(&score_text, None, 40, 1.0);
        draw_text(
            &score_text,
            screen_w / 2.0 - text_dim.width / 2.0,
            60.0,
            40.0,
            WHITE,
        );

        next_frame().await;
    }
}

//# Reset ball position and velocity
fn reset_ball(ball: &mut Ball, screen_w: f32, screen_h: f32, dir: f32) {
    use macroquad::rand::gen_range;
    use std::f32::consts::PI;

    //## Center the ball
    ball.x = screen_w / 2.0 - BALL_SIZE / 2.0;
    ball.y = screen_h / 2.0 - BALL_SIZE / 2.0;

    //## Random angle between -45 and 45 degrees
    let angle = gen_range(-PI / 4.0, PI / 4.0);

    //## Set velocity
    ball.vx = BALL_SPEED * dir * angle.cos();
    ball.vy = BALL_SPEED * angle.sin();
}
