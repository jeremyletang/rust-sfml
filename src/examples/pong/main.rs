/**
* Example from SFML : Pong
*/

extern mod rsfml;

use rsfml::graphics::render_window::*;
use rsfml::window::video_mode::*;
use rsfml::window::context_settings::*;
use rsfml::system::vector2::*;
use rsfml::window::event;
use rsfml::graphics::text;
use rsfml::graphics::color::*;
use rsfml::graphics::font;
use rsfml::audio::sound_buffer::*;
use rsfml::audio::sound::*;
use rsfml::graphics::rectangle_shape::*;
use rsfml::graphics::circle_shape::*;
use rsfml::system::clock::*;
use rsfml::system::time::*;
use rsfml::window::keyboard;

#[cfg(target_os="macos")]
#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    std::rt::start_on_main_thread(argc, argv, crate_map, main)
}

fn main () -> () {
    // Define some constants
    let pi : f32 = 3.14159;
    let gameWidth : uint = 800;
    let gameHeight : uint = 600;
    let paddleSize : Vector2f =  Vector2f::new(25., 100.);
    let ballRadius : f32 = 10.;

     // Create the window of the application
    let setting : ContextSettings = ContextSettings::default();
    let mut window : RenderWindow = match RenderWindow::new(VideoMode::new_init(gameWidth, gameHeight, 32), ~"SFML Pong", sfClose, &setting) {
        Some(window) => window,
        None => fail!("Cannot create a new Render Window.")
    };
    window.set_vertical_sync_enabled(true);

    // Load the sounds used in the game
    let ballSoundBuffer : @SoundBuffer = match SoundBuffer::new(~"resources/ball.wav") {
        Some(ballSoundBuffer)   => @ballSoundBuffer,
        None                    => fail!("Cannot load Ball sound buffer.")
    };

    let mut ballSound = match Sound::new(ballSoundBuffer) {
        Some(sound)     => sound,
        None            => fail!("Error cannot create sound.")
    };
//    ballSound.set_buffer(&ballSoundBuffer);
    ballSound.set_volume(100.);

    // Create the left paddle
    let mut leftPaddle  = match RectangleShape::new() {
        Some(paddle)    => paddle,
        None()          => fail!("Error, cannot create paddle")
    };
    leftPaddle.set_size(~(paddleSize - 3.));
    leftPaddle.set_outline_thickness(3.);
    leftPaddle.set_outline_color(~Color::black());
    leftPaddle.set_fill_color(~Color::new_RGB(100, 100, 200));
    leftPaddle.set_origin(~(paddleSize / 2.));
    
    // Create the right paddle
    let mut rightPaddle = match RectangleShape::new() {
        Some(paddle)    => paddle,
        None()          => fail!("Error, cannot create paddle")
    };
    rightPaddle.set_size(~(paddleSize - 3.));
    rightPaddle.set_outline_thickness(3.);
    rightPaddle.set_outline_color(~Color::black());
    rightPaddle.set_fill_color(~Color::new_RGB(200, 100, 100));
    rightPaddle.set_origin(~(paddleSize / 2.));

    // Create the ball
    let mut ball = match CircleShape::new() {
        Some(ball)    => ball,
        None()          => fail!("Error, cannot create ball")
    };
    ball.set_radius(ballRadius as float - 3.);
    ball.set_outline_thickness(3.);
    ball.set_outline_color(~Color::black());
    ball.set_fill_color(~Color::white());
    ball.set_origin(~Vector2f::new(ballRadius / 2., ballRadius / 2.));

    // Load the text font
    let font : @font::Font = match font::Font::new_from_file(~"resources/sansation.ttf") {
        Some(font)    => @font,
        None()          => fail!("Error, cannot load font")
    };

     // Initialize the pause message
    let mut pauseMessage : text::Text = match text::Text::new() {
        Some(text) => text,
        None => fail!(~"Error on creating text")
    };
    pauseMessage.set_font(font);
    pauseMessage.set_character_size(40);
    pauseMessage.set_position(~(Vector2f::new(170., 150.)));
    pauseMessage.set_color(~Color::white());
    pauseMessage.set_string(~"Welcome to SFML pong!\nPress space to start the game");

    // Define the paddles properties
    let mut AITimer =  Clock::new();
    let AITime : Time  = Time::with_seconds(0.1);
    let paddleSpeed = 400.;
    let mut rightPaddleSpeed  = 0.;
    let ballSpeed   = 400.;
    let mut ballAngle : f32  = 0.; // to be changed later

    let mut clock = Clock::new();
    let mut isPlaying = false;

    while window.is_open() {
        loop {
            match window.poll_event() {
                event::Closed => window.close(),
                event::KeyPressed(key, _, _, _, _) => match key {
                    keyboard::Escape      => {window.close(); break},
                    keyboard::Space       => {
                        if !isPlaying {
                            // (re)start the game
                            isPlaying = true;
                            clock.restart();
                            // Reset the position of the paddles and ball
                            leftPaddle.set_position(~Vector2f::new(10. + paddleSize.x / 2., gameHeight as f32 / 2.));
                            rightPaddle.set_position(~Vector2f::new(gameWidth as f32 - 10. - paddleSize.x / 2., gameHeight as f32 / 2.));
                            ball.set_position(~Vector2f::new(gameWidth as f32 / 2., gameHeight as f32 / 2.));
                            // RANDOM HERE
                        }
                    },
                    _                      => {}
                } ,
                event::NoEvent => break,
                _ => {}
            }
        }
        if isPlaying {
            let deltaTime = clock.restart().as_seconds();

            // Move the player's paddle
            if keyboard::is_key_pressed(keyboard::Up) &&
               (leftPaddle.get_position().y - paddleSize.y / 2. > 5.) {
                leftPaddle.move2f(0., -paddleSpeed * deltaTime);
            }
            if keyboard::is_key_pressed(keyboard::Down) &&
               (leftPaddle.get_position().y + paddleSize.y / 2. < gameHeight as f32 - 5.) {
                leftPaddle.move2f(0., paddleSpeed * deltaTime);
            }
            
            // Move the computer's paddle
            if ((rightPaddleSpeed < 0.) && (rightPaddle.get_position().y - paddleSize.y / 2. > 5.)) ||
                ((rightPaddleSpeed > 0.) && (rightPaddle.get_position().y + paddleSize.y / 2. < gameHeight as f32 - 5.)) {
                rightPaddle.move2f(0., rightPaddleSpeed * deltaTime);
            }
            
            // Update the computer's paddle direction according to the ball position
            if (AITimer.get_elapsed_time().as_microseconds() > AITime.as_microseconds())
            {
                AITimer.restart();
                if ball.get_position().y + ballRadius > rightPaddle.get_position().y + paddleSize.y / 2. {
                    rightPaddleSpeed = paddleSpeed;
                }
                
                else if  ball.get_position().y - ballRadius < rightPaddle.get_position().y - paddleSize.y / 2. {
                    rightPaddleSpeed = -paddleSpeed;
                }
                
                else {
                    rightPaddleSpeed = 0.;
                }
            }

            // Move the ball
            let factor = ballSpeed * deltaTime;
            ball.move(~Vector2f::new(ballAngle.cos() * factor, ballAngle.sin() * factor));
            
            // Check collisions between the ball and the screen
            if ball.get_position().x - ballRadius < 0. {
                isPlaying = false;
                pauseMessage.set_string(~"You lost !\nPress space to restart or\nescape to exit");
            }
            if ball.get_position().x + ballRadius > gameWidth as f32 {
                isPlaying = false;
                pauseMessage.set_string(~"You won !\nPress space to restart or\nescape to exit");
            }
            if ball.get_position().y - ballRadius < 0. {
                ballSound.play();
                ballAngle = -ballAngle;
                let p = ball.get_position().x;
                ball.set_position(~Vector2f::new(p, ballRadius + 0.1));
            }
            if ball.get_position().y + ballRadius > gameHeight as f32 {
                ballSound.play();
                ballAngle = -ballAngle;
                let p = ball.get_position().x;
                ball.set_position(~Vector2f::new(p, gameHeight as f32 - ballRadius - 0.1));
            }

            // Check the collisions between the ball and the paddles
            // Left Paddle
            if ball.get_position().x - ballRadius < leftPaddle.get_position().x + paddleSize.x / 2. && 
                ball.get_position().x - ballRadius > leftPaddle.get_position().x &&
                ball.get_position().y + ballRadius >= leftPaddle.get_position().y - paddleSize.y / 2. &&
                ball.get_position().y - ballRadius <= leftPaddle.get_position().y + paddleSize.y / 2. {
                if ball.get_position().y > leftPaddle.get_position().y {
                    ballAngle = pi - ballAngle + (std::rand::random::<int>() % 20) as f32 * pi / 180.;
                }
                else {
                    ballAngle = pi - ballAngle - (std::rand::random::<int>() % 20) as f32 * pi / 180.;
                }
                
                ballSound.play();
                let p = ball.get_position().y;
                ball.set_position(~Vector2f::new(leftPaddle.get_position().x + ballRadius + paddleSize.x / 2. + 0.1, p));
            }

            // Right Paddle
            if ball.get_position().x + ballRadius > rightPaddle.get_position().x - paddleSize.x / 2. &&
                ball.get_position().x + ballRadius < rightPaddle.get_position().x &&
                ball.get_position().y + ballRadius >= rightPaddle.get_position().y - paddleSize.y / 2. &&
                ball.get_position().y - ballRadius <= rightPaddle.get_position().y + paddleSize.y / 2. {
                if ball.get_position().y > rightPaddle.get_position().y {
                    ballAngle = pi - ballAngle + (std::rand::random::<int>() % 20) as f32* pi / 180.;
                }
                else {
                    ballAngle = pi - ballAngle - (std::rand::random::<int>() % 20) as f32* pi / 180.;
                }
                
                ballSound.play();
                let p = ball.get_position().y;  
                ball.set_position(~Vector2f::new(rightPaddle.get_position().x - ballRadius - paddleSize.x / 2. - 0.1, p));
            }
            
                //let a = r.gen::<float>();
                
        }
        // Clear the window
        window.clear(~Color::new_RGB(50, 200, 50));

        if isPlaying {
            // Draw the paddles and the ball
            window.draw(&leftPaddle);
            window.draw(&rightPaddle);
            window.draw(&ball);
        }
        else
        {
            // Draw the pause message
            window.draw(&pauseMessage);
        }

        // Display things on screen
        window.display()

    }

}