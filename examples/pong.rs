//! Example from SFML: Pong

extern crate rsfml;
extern crate rand;

use rsfml::graphics::{RenderWindow, Color, Font, Text, RectangleShape, CircleShape,
                      RenderTarget};
use rsfml::window::{VideoMode, ContextSettings, event, Close};
use rsfml::window::keyboard::{self, Key};
use rsfml::system::{Vector2f, Clock, Time};
use rsfml::audio::{SoundBuffer, Sound};

fn main () -> () {
    // Define some constants
    let pi: f32 = 3.14159;
    let game_width: u32 = 800;
    let game_height: u32 = 600;
    let paddle_size: Vector2f =  Vector2f::new(25., 100.);
    let ball_radius: f32 = 10.;

     // Create the window of the application
    let setting: ContextSettings = ContextSettings::default();
    let mut window: RenderWindow =
        match RenderWindow::new(VideoMode::new_init(game_width, game_height, 32),
                                "SFML Pong",
                                Close,
                                &setting) {
            Some(window) => window,
            None => panic!("Cannot create a new Render Window.")
        };
    window.set_vertical_sync_enabled(true);

    // Load the sounds used in the game
    let ball_soundbuffer = match SoundBuffer::new("resources/ball.wav") {
        Some(ball_soundbuffer)   => ball_soundbuffer,
        None                    => panic!("Cannot load Ball sound buffer.")
    };

    let mut ball_sound = match Sound::new_with_buffer(&ball_soundbuffer) {
        Some(sound)     => sound,
        None            => panic!("Error cannot create sound.")
    };
    //    ball_sound.set_buffer(&ball_soundbuffer);
    ball_sound.set_volume(100.);

    // Create the left paddle
    let mut left_paddle  = match RectangleShape::new() {
        Some(paddle)    => paddle,
        None            => panic!("Error, cannot create paddle")
    };
    left_paddle.set_size(&(paddle_size - 3f32));
    left_paddle.set_outline_thickness(3.);
    left_paddle.set_outline_color(&Color::black());
    left_paddle.set_fill_color(&Color::new_RGB(100, 100, 200));
    left_paddle.set_origin(&(paddle_size / 2f32));

    // Create the right paddle
    let mut right_paddle = match RectangleShape::new() {
        Some(paddle)    => paddle,
        None            => panic!("Error, cannot create paddle")
    };
    right_paddle.set_size(&(paddle_size - 3f32));
    right_paddle.set_outline_thickness(3.);
    right_paddle.set_outline_color(&Color::black());
    right_paddle.set_fill_color(&Color::new_RGB(200, 100, 100));
    right_paddle.set_origin(&(paddle_size / 2f32));

    // Create the ball
    let mut ball = match CircleShape::new() {
        Some(ball)    => ball,
        None          => panic!("Error, cannot create ball")
    };
    ball.set_radius(ball_radius as f32 - 3.);
    ball.set_outline_thickness(3.);
    ball.set_outline_color(&Color::black());
    ball.set_fill_color(&Color::white());
    ball.set_origin(&Vector2f::new(ball_radius / 2., ball_radius / 2.));

    // Load the text font
    let font = match Font::new_from_file("resources/sansation.ttf") {
        Some(font)    => font,
        None          => panic!("Error, cannot load font")
    };

     // Initialize the pause message
    let mut pause_message: Text = match Text::new() {
        Some(text) => text,
        None       => panic!("Error on creating text")
    };
    pause_message.set_font(&font);
    pause_message.set_character_size(40);
    pause_message.set_position(&(Vector2f::new(170., 150.)));
    pause_message.set_color(&Color::white());
    pause_message.set_string("Welcome to SFML pong!\nPress space to start the game");

    // Define the paddles properties
    let mut ai_timer =  Clock::new();
    let ai_time: Time  = Time::with_seconds(0.1);
    let paddle_speed = 400.;
    let mut right_paddle_speed  = 0.;
    let ball_speed   = 400.;
    let mut ball_angle: f32  = 0.; // to be changed later

    let mut clock = Clock::new();
    let mut is_playing = false;

    while window.is_open() {
        loop {
            match window.poll_event() {
                event::Closed => window.close(),
                event::KeyPressed{code, ..} => match code {
                    Key::Escape      => {window.close(); break},
                    Key::Space       => {
                        if !is_playing {
                            // (re)start the game
                            is_playing = true;
                            clock.restart();
                            // Reset the position of the paddles and ball
                            left_paddle.set_position(&Vector2f::new(10. + paddle_size.x / 2., game_height as f32 / 2.));
                            right_paddle.set_position(&Vector2f::new(game_width as f32 - 10. - paddle_size.x / 2., game_height as f32 / 2.));
                            ball.set_position(&Vector2f::new(game_width as f32 / 2., game_height as f32 / 2.));
                            // RANDOM HERE
                        }
                    },
                    _                      => {}
                } ,
                event::NoEvent => break,
                _ => {}
            }
        }
        if is_playing {
            let delta_time = clock.restart().as_seconds();

            // Move the player's paddle
            if keyboard::is_key_pressed(Key::Up) &&
               (left_paddle.get_position().y - paddle_size.y / 2. > 5.) {
                left_paddle.move2f(0., -paddle_speed * delta_time);
            }
            if keyboard::is_key_pressed(Key::Down) &&
               (left_paddle.get_position().y + paddle_size.y / 2. < game_height as f32 - 5.) {
                left_paddle.move2f(0., paddle_speed * delta_time);
            }

            // Move the computer's paddle
            if ((right_paddle_speed < 0.) && (right_paddle.get_position().y - paddle_size.y / 2. > 5.)) ||
                ((right_paddle_speed > 0.) && (right_paddle.get_position().y + paddle_size.y / 2. < game_height as f32 - 5.)) {
                right_paddle.move2f(0., right_paddle_speed * delta_time);
            }

            // Update the computer's paddle direction according to the ball position
            if ai_timer.get_elapsed_time().as_microseconds() > ai_time.as_microseconds() {
                ai_timer.restart();
                if ball.get_position().y + ball_radius > right_paddle.get_position().y + paddle_size.y / 2. {
                    right_paddle_speed = paddle_speed;
                }

                else if  ball.get_position().y - ball_radius < right_paddle.get_position().y - paddle_size.y / 2. {
                    right_paddle_speed = -paddle_speed;
                }

                else {
                    right_paddle_speed = 0.;
                }
            }

            // Move the ball
            let factor = ball_speed * delta_time;
            ball.move_(&Vector2f::new(ball_angle.cos() * factor, ball_angle.sin() * factor));

            // Check collisions between the ball and the screen
            if ball.get_position().x - ball_radius < 0. {
                is_playing = false;
                pause_message.set_string("You lost !\nPress space to restart or\nescape to exit");
            }
            if ball.get_position().x + ball_radius > game_width as f32 {
                is_playing = false;
                pause_message.set_string("You won !\nPress space to restart or\nescape to exit");
            }
            if ball.get_position().y - ball_radius < 0. {
                ball_sound.play();
                ball_angle = -ball_angle;
                let p = ball.get_position().x;
                ball.set_position(&Vector2f::new(p, ball_radius + 0.1));
            }
            if ball.get_position().y + ball_radius > game_height as f32 {
                ball_sound.play();
                ball_angle = -ball_angle;
                let p = ball.get_position().x;
                ball.set_position(&Vector2f::new(p, game_height as f32 - ball_radius - 0.1));
            }

            // Check the collisions between the ball and the paddles
            // Left Paddle
            if ball.get_position().x - ball_radius < left_paddle.get_position().x + paddle_size.x / 2. &&
                ball.get_position().x - ball_radius > left_paddle.get_position().x &&
                ball.get_position().y + ball_radius >= left_paddle.get_position().y - paddle_size.y / 2. &&
                ball.get_position().y - ball_radius <= left_paddle.get_position().y + paddle_size.y / 2. {
                if ball.get_position().y > left_paddle.get_position().y {
                    ball_angle = pi - ball_angle + (rand::random::<i32>() % 20) as f32 * pi / 180.;
                }
                else {
                    ball_angle = pi - ball_angle - (rand::random::<i32>() % 20) as f32 * pi / 180.;
                }

                ball_sound.play();
                let p = ball.get_position().y;
                ball.set_position(&Vector2f::new(left_paddle.get_position().x + ball_radius + paddle_size.x / 2. + 0.1, p));
            }

            // Right Paddle
            if ball.get_position().x + ball_radius > right_paddle.get_position().x - paddle_size.x / 2. &&
                ball.get_position().x + ball_radius < right_paddle.get_position().x &&
                ball.get_position().y + ball_radius >= right_paddle.get_position().y - paddle_size.y / 2. &&
                ball.get_position().y - ball_radius <= right_paddle.get_position().y + paddle_size.y / 2. {
                if ball.get_position().y > right_paddle.get_position().y {
                    ball_angle = pi - ball_angle + (rand::random::<i32>() % 20) as f32* pi / 180.;
                }
                else {
                    ball_angle = pi - ball_angle - (rand::random::<i32>() % 20) as f32* pi / 180.;
                }

                ball_sound.play();
                let p = ball.get_position().y;
                ball.set_position(&Vector2f::new(right_paddle.get_position().x - ball_radius - paddle_size.x / 2. - 0.1, p));
            }

            //let a = r.gen::<float>();
        }
        // Clear the window
        window.clear(&Color::new_RGB(50, 200, 50));

        if is_playing {
            // Draw the paddles and the ball
            window.draw(&left_paddle);
            window.draw(&right_paddle);
            window.draw(&ball);
        }
        else
        {
            // Draw the pause message
            window.draw(&pause_message);
        }

        // Display things on screen
        window.display()

    }

}
