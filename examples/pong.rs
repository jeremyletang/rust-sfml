use rand::{thread_rng, Rng};
use sfml::{
    audio::{Sound, SoundBuffer, SoundSource},
    graphics::{
        CircleShape, Color, Font, RectangleShape, RenderTarget, RenderWindow, Shape, Text,
        Transformable,
    },
    system::{Clock, Time, Vector2f},
    window::{ContextSettings, Event, Key, Style},
};
use std::{env, f32::consts::PI};

fn main() {
    let mut rng = thread_rng();

    // Optional antialiasing
    let mut aa_level = 0;

    if let Some(arg) = env::args().nth(1) {
        match arg.parse::<u32>() {
            Ok(arg_as_num) => aa_level = arg_as_num,
            Err(e) => println!("Didn't set AA level: {}", e),
        }
    }

    // Define some constants
    let game_width = 800;
    let game_height = 600;
    let paddle_size = Vector2f::new(25., 100.);
    let ball_radius = 10.;

    // Create the window of the application
    let context_settings = ContextSettings {
        antialiasing_level: aa_level,
        ..Default::default()
    };
    let mut window = RenderWindow::new(
        (game_width, game_height),
        "SFML Pong",
        Style::CLOSE,
        &context_settings,
    );
    let context_settings = window.settings();
    if context_settings.antialiasing_level > 0 {
        println!("Using {}xAA", context_settings.antialiasing_level);
    }
    window.set_vertical_sync_enabled(true);

    // Load the sounds used in the game
    let ball_soundbuffer = SoundBuffer::from_file("resources/ball.wav").unwrap();
    let mut ball_sound = Sound::with_buffer(&ball_soundbuffer);

    // Create the left paddle
    let mut left_paddle = RectangleShape::new();
    left_paddle.set_size(paddle_size - 3.);
    left_paddle.set_outline_thickness(3.);
    left_paddle.set_outline_color(Color::BLACK);
    left_paddle.set_fill_color(Color::rgb(100, 100, 200));
    left_paddle.set_origin(paddle_size / 2.);

    // Create the right paddle
    let mut right_paddle = RectangleShape::new();
    right_paddle.set_size(paddle_size - 3.);
    right_paddle.set_outline_thickness(3.);
    right_paddle.set_outline_color(Color::BLACK);
    right_paddle.set_fill_color(Color::rgb(200, 100, 100));
    right_paddle.set_origin(paddle_size / 2.);

    // Create the ball
    let mut ball = CircleShape::default();
    ball.set_radius(ball_radius - 3.);
    ball.set_outline_thickness(3.);
    ball.set_outline_color(Color::BLACK);
    ball.set_fill_color(Color::WHITE);
    ball.set_origin((ball_radius / 2., ball_radius / 2.));

    // Load the text font
    let font = Font::from_file("resources/sansation.ttf").unwrap();

    // Initialize the pause message
    let mut pause_message = Text::default();
    pause_message.set_font(&font);
    pause_message.set_character_size(40);
    pause_message.set_position((170., 150.));
    pause_message.set_fill_color(Color::WHITE);
    pause_message.set_string("Welcome to SFML pong!\nPress space to start the game");

    // Define the paddles properties
    let mut ai_timer = Clock::start();
    let ai_time = Time::seconds(0.0333);
    let paddle_speed = 400.;
    let mut right_paddle_speed = 0.;
    let mut ball_speed = 400.;
    let mut ball_angle = 0.;

    let mut clock = Clock::start();
    let mut is_playing = false;
    let mut up = false;
    let mut down = false;

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                Event::KeyPressed {
                    code: Key::Space, ..
                } if !is_playing => {
                    // (re)start the game
                    is_playing = true;
                    ball_speed = 400.0;
                    ball_sound.set_pitch(1.0);
                    clock.restart();
                    // Reset the position of the paddles and ball
                    left_paddle.set_position((10. + paddle_size.x / 2., game_height as f32 / 2.));
                    right_paddle.set_position((
                        game_width as f32 - 10. - paddle_size.x / 2.,
                        game_height as f32 / 2.,
                    ));
                    ball.set_position((game_width as f32 / 2., game_height as f32 / 2.));
                    // Reset the ball angle
                    loop {
                        // Make sure the ball initial angle is not too much vertical
                        ball_angle = rng.gen_range(0.0..360.) * 2. * PI / 360.;

                        if ball_angle.cos().abs() >= 0.7 {
                            break;
                        }
                    }
                }
                Event::KeyPressed { code: Key::Up, .. } => up = true,
                Event::KeyReleased { code: Key::Up, .. } => up = false,
                Event::KeyPressed {
                    code: Key::Down, ..
                } => down = true,
                Event::KeyReleased {
                    code: Key::Down, ..
                } => down = false,
                _ => {}
            }
        }
        if is_playing {
            let delta_time = clock.restart().as_seconds();

            // Move the player's paddle
            if up && (left_paddle.position().y - paddle_size.y / 2. > 5.) {
                left_paddle.move_((0., -paddle_speed * delta_time));
            }
            if down && (left_paddle.position().y + paddle_size.y / 2. < game_height as f32 - 5.) {
                left_paddle.move_((0., paddle_speed * delta_time));
            }

            // Move the computer's paddle
            if ((right_paddle_speed < 0.) && (right_paddle.position().y - paddle_size.y / 2. > 5.))
                || ((right_paddle_speed > 0.)
                    && (right_paddle.position().y + paddle_size.y / 2. < game_height as f32 - 5.))
            {
                right_paddle.move_((0., right_paddle_speed * delta_time));
            }

            // Update the computer's paddle direction according to the ball position
            if ai_timer.elapsed_time() > ai_time {
                ai_timer.restart();
                if ball.position().y + ball_radius > right_paddle.position().y + paddle_size.y / 2.
                {
                    right_paddle_speed = paddle_speed;
                } else if ball.position().y - ball_radius
                    < right_paddle.position().y - paddle_size.y / 2.
                {
                    right_paddle_speed = -paddle_speed;
                } else {
                    right_paddle_speed = 0.;
                }
            }

            // Move the ball
            let factor = ball_speed * delta_time;
            ball.move_((ball_angle.cos() * factor, ball_angle.sin() * factor));

            // Check collisions between the ball and the screen
            if ball.position().x - ball_radius < 0. {
                is_playing = false;
                pause_message.set_string("You lost !\nPress space to restart or\nescape to exit");
            }
            if ball.position().x + ball_radius > game_width as f32 {
                is_playing = false;
                pause_message.set_string("You won !\nPress space to restart or\nescape to exit");
            }
            if ball.position().y - ball_radius < 0. {
                on_bounce(&mut ball_sound, &mut ball_speed);
                ball_angle = -ball_angle;
                let p = ball.position().x;
                ball.set_position((p, ball_radius + 0.1));
            }
            if ball.position().y + ball_radius > game_height as f32 {
                on_bounce(&mut ball_sound, &mut ball_speed);
                ball_angle = -ball_angle;
                let p = ball.position().x;
                ball.set_position((p, game_height as f32 - ball_radius - 0.1));
            }

            // Check the collisions between the ball and the paddles
            // Left Paddle
            let (ball_pos, paddle_pos) = (ball.position(), left_paddle.position());
            if ball_pos.x - ball_radius < paddle_pos.x + paddle_size.x / 2.
                && ball_pos.y + ball_radius >= paddle_pos.y - paddle_size.y / 2.
                && ball_pos.y - ball_radius <= paddle_pos.y + paddle_size.y / 2.
            {
                if ball_pos.y > paddle_pos.y {
                    ball_angle = PI - ball_angle + rng.gen_range(0.0..20.) * PI / 180.;
                } else {
                    ball_angle = PI - ball_angle - rng.gen_range(0.0..20.) * PI / 180.;
                }

                on_bounce(&mut ball_sound, &mut ball_speed);
                ball.set_position((
                    paddle_pos.x + ball_radius + paddle_size.x / 2. + 0.1,
                    ball_pos.y,
                ));
            }

            // Right Paddle
            let (ball_pos, paddle_pos) = (ball.position(), right_paddle.position());
            if ball_pos.x + ball_radius > paddle_pos.x - paddle_size.x / 2.
                && ball_pos.y + ball_radius >= paddle_pos.y - paddle_size.y / 2.
                && ball_pos.y - ball_radius <= paddle_pos.y + paddle_size.y / 2.
            {
                if ball_pos.y > paddle_pos.y {
                    ball_angle = PI - ball_angle + rng.gen_range(0.0..20.) * PI / 180.;
                } else {
                    ball_angle = PI - ball_angle - rng.gen_range(0.0..20.) * PI / 180.;
                }

                on_bounce(&mut ball_sound, &mut ball_speed);
                ball.set_position((
                    paddle_pos.x - ball_radius - paddle_size.x / 2. - 0.1,
                    ball_pos.y,
                ));
            }
        }
        // Clear the window
        window.clear(Color::rgb(50, 200, 50));

        if is_playing {
            // Draw the paddles and the ball
            window.draw(&left_paddle);
            window.draw(&right_paddle);
            window.draw(&ball);
        } else {
            // Draw the pause message
            window.draw(&pause_message);
        }

        // Display things on screen
        window.display()
    }
}

fn on_bounce(ball_sound: &mut Sound, ball_speed: &mut f32) {
    ball_sound.play();
    ball_sound.set_pitch(ball_sound.pitch() + 0.004);
    *ball_speed += 16.0;
}
