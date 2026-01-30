use macroquad::prelude::*;

//# Constants
    const PADDLE_WIDTH: f32 = 15.0;
    const PADDLE_HEIGHT: f32 = 80.0;
    const PADDLE_SPEED: f32 = 400.0;

    const AI_PADDLE_SPEED: f32 = PADDLE_SPEED;

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
        window_title: "Ping!".to_string(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

//# Main function
#[macroquad::main(window_conf)]
async fn main() {
    let screen_w = screen_width();
    let screen_h = screen_height();
    let single_player;

    //## Title screen
    loop {
        clear_background(BLACK);

        //### ASCII Art for *PING!*
        let ping_art = [
            "       ____ ___ _   _  ____ _       ",
            " __/\\_|  _ \\_ _| \\ | |/ ___| |_ /\\__ ",
            " \\    / |_) | ||  \\| | |  _| \\    / ",
            " /_  _\\  __/| || |\\  | |_| |_/_  _\\ ",
            "   \\/ |_|  |___|_| \\_|\\____(_) \\/   ",
        ];
        let mut art_y = screen_h / 2.0 - 120.0;
        for line in &ping_art {
            let text_dim = measure_text(line, None, 40, 1.0);
            let art_x = screen_w / 2.0 - text_dim.width / 2.0;
            draw_text(line, art_x, art_y, 40.0, WHITE);
            art_y += 40.0;
        }

        //### Copyright and version
        let copyright = "© 2026 leet-man";
        draw_text(
            copyright,
            screen_w / 2.0 - measure_text(copyright, None, 24, 1.0).width / 2.0,
            screen_h - 60.0,
            24.0,
            GRAY,
        );
        let version = "vers 0.3.0";
        draw_text(
            version,
            screen_w / 2.0 - measure_text(version, None, 24, 1.0).width / 2.0,
            screen_h - 30.0,
            24.0,
            GRAY,
        );

        //### Player select
        draw_text(
            "Press 1 for Single Player",
            screen_w / 2.0 - 180.0,
            screen_h / 2.0 + 80.0,
            32.0,
            WHITE,
        );
        draw_text(
            "Press 2 for Two Player",
            screen_w / 2.0 - 160.0,
            screen_h / 2.0 + 120.0,
            32.0,
            WHITE,
        );

        //### Input handling for player selection
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

    //## Initial ball state
    let mut ball = Ball {
        x: screen_w / 2.0,
        y: screen_h / 2.0,
        vx: BALL_SPEED,
        vy: BALL_SPEED * 0.6,
    };

    //## Scores and AI variables
    let mut left_score = 0;
    let mut right_score = 0;

    //## Game loop
    loop {
        let dt = get_frame_time();

        //### Left paddle control
        if single_player {

            //#### AI Control for left paddle (edge bias)
            if ball.vx < 0.0 {

                // ##### Calculate target position with edge bias and random error
                let edge_bias = macroquad::rand::gen_range(-1.0, 1.0);
                let edge_offset = edge_bias * (PADDLE_HEIGHT / 2.0 - BALL_SIZE / 2.0) * 0.7;
                let random_error = macroquad::rand::gen_range(-15.0, 15.0);
                let target = ball.y + BALL_SIZE / 2.0 + edge_offset + random_error;
                let paddle_center = left.y + PADDLE_HEIGHT / 2.0;
                let diff = target - paddle_center;
                let threshold = 6.0;

                // ##### Move paddle towards ball
                if diff.abs() > threshold {
                    let lerp_factor = macroquad::rand::gen_range(0.10, 0.20);
                    let mut movement = diff * lerp_factor;

                    //###### Clamp to AI_PADDLE_SPEED * dt
                    let max_movement = AI_PADDLE_SPEED * dt;
                    if movement > max_movement {
                        movement = max_movement;
                    } else if movement < -max_movement {
                        movement = -max_movement;
                    }

                    left.y += movement;
                }
            }
        }
            else {
            //#### Two-player: W/S for left paddle
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
        handle_paddle_collision(&mut ball, &left, &left_rect, &ball_rect, true);
        handle_paddle_collision(&mut ball, &right, &right_rect, &ball_rect, false);

        //### Scoring
        if ball.x + BALL_SIZE < 0.0 {
            right_score += 1;
            reset_ball(&mut ball, screen_w, screen_h, 1.0);
            //#### Recenter paddles
            left.y = screen_h / 2.0 - PADDLE_HEIGHT / 2.0;
            right.y = screen_h / 2.0 - PADDLE_HEIGHT / 2.0;
        }
        if ball.x > screen_w {
            left_score += 1;
            reset_ball(&mut ball, screen_w, screen_h, -1.0);
            //#### Recenter paddles
            left.y = screen_h / 2.0 - PADDLE_HEIGHT / 2.0;
            right.y = screen_h / 2.0 - PADDLE_HEIGHT / 2.0;
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

    //## Random angle between -45 and 45 degrees, but not too close to 0 (horizontal)
    let min_angle = PI / 32.0;
    let angle = if gen_range(0.0, 1.0) < 0.5 {
        gen_range(-PI / 4.0, -min_angle)
    } else {
        gen_range(min_angle, PI / 4.0)
    };

    //## Set velocity
    ball.vx = BALL_SPEED * dir * angle.cos();
    ball.vy = BALL_SPEED * angle.sin();
}

//# Handle paddle collision
fn handle_paddle_collision(
    ball: &mut Ball,
    paddle: &Paddle,
    paddle_rect: &Rect,
    ball_rect: &Rect,
    is_left: bool,
) {
    //## Check for collision
    let hit = if is_left {
        ball_rect.overlaps(paddle_rect) && ball.vx < 0.0
    } else {
        ball_rect.overlaps(paddle_rect) && ball.vx > 0.0
    };

    //## If hit, adjust ball position and velocity
    if hit {
        if is_left {
            ball.x = paddle.x + PADDLE_WIDTH;
        } else {
            ball.x = paddle.x - BALL_SIZE;
        }
        ball.vx = -ball.vx;

        //### Adjust ball speed based on hit position
        let paddle_center = paddle.y + PADDLE_HEIGHT / 2.0;
        let ball_center = ball.y + BALL_SIZE / 2.0;
        let offset = (ball_center - paddle_center).abs();
        let norm = (offset / (PADDLE_HEIGHT / 2.0)).min(1.0);

        //### Multiplier: 0.5 at center, 1.5 at edge
        let speed_multiplier = 0.5 + norm;
        let min_speed = BALL_SPEED; // Use the original ball speed as the minimum
        let new_speed = (ball.vx.abs() * speed_multiplier).max(min_speed);
        ball.vx = ball.vx.signum() * new_speed;
    }
}
