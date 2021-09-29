use glu_sys as gl;
use sfml::{
    graphics::{
        Color, Font, IntRect, RenderTarget, RenderWindow, Sprite, Text, Texture, Transformable,
    },
    system::Clock,
    window::{ContextSettings, Event, Key, Style},
};
use std::{ffi::c_void, mem::size_of};

fn main() {
    let mut exit = false;
    let mut srgb = false;

    while !exit {
        let mut ctx_sett = ContextSettings::default();
        ctx_sett.set_depth_bits(24);
        ctx_sett.set_srgb_capable(srgb);

        let mut window = RenderWindow::new(
            (800, 600),
            "SFML graphics with OpenGL",
            Style::default(),
            &ctx_sett,
        );
        window.set_vertical_sync_enabled(true);

        let mut bg_tex = Texture::new().unwrap();
        bg_tex.set_srgb(srgb);
        bg_tex
            .load_from_file("resources/opengl-background.jpg", IntRect::default())
            .unwrap();
        let bg_sprite = Sprite::with_texture(&bg_tex);

        let font = Font::from_file("resources/sansation.ttf").unwrap();
        let mut text = Text::new("SFML / OpenGL demo", &font, 32);
        let mut srgb_instr = Text::new("Press space to toggle sRGB conversion", &font, 32);
        let mut mipmap_instr = Text::new("Press return to toggle mipmapping", &font, 32);
        text.set_fill_color(Color::rgba(255, 255, 255, 170));
        srgb_instr.set_fill_color(Color::rgba(255, 255, 255, 170));
        mipmap_instr.set_fill_color(Color::rgba(255, 255, 255, 170));
        text.set_position((250., 450.));
        srgb_instr.set_position((150., 500.));
        mipmap_instr.set_position((180., 550.));

        let mut texture = Texture::from_file("resources/texture.jpg").unwrap();
        texture.generate_mipmap();
        window.set_active(true);
        unsafe {
            gl::glEnable(gl::GL_DEPTH_TEST);
            gl::glDepthMask(gl::GL_TRUE as _);
            gl::glClearDepth(1.);
            gl::glDisable(gl::GL_LIGHTING);
            gl::glViewport(0, 0, window.size().x as _, window.size().y as _);
            gl::glMatrixMode(gl::GL_PROJECTION);
            gl::glLoadIdentity();
            let ratio = (window.size().x / window.size().y) as gl::GLdouble;
            gl::glFrustum(-ratio, ratio, -1., 1., 1., 500.);
            gl::glEnable(gl::GL_TEXTURE_2D);
            Texture::bind(&texture);
        }

        let cube: [f32; 180] = [
            -20., -20., -20., 0., 0., -20., 20., -20., 1., 0., -20., -20., 20., 0., 1., -20., -20.,
            20., 0., 1., -20., 20., -20., 1., 0., -20., 20., 20., 1., 1., 20., -20., -20., 0., 0.,
            20., 20., -20., 1., 0., 20., -20., 20., 0., 1., 20., -20., 20., 0., 1., 20., 20., -20.,
            1., 0., 20., 20., 20., 1., 1., -20., -20., -20., 0., 0., 20., -20., -20., 1., 0., -20.,
            -20., 20., 0., 1., -20., -20., 20., 0., 1., 20., -20., -20., 1., 0., 20., -20., 20.,
            1., 1., -20., 20., -20., 0., 0., 20., 20., -20., 1., 0., -20., 20., 20., 0., 1., -20.,
            20., 20., 0., 1., 20., 20., -20., 1., 0., 20., 20., 20., 1., 1., -20., -20., -20., 0.,
            0., 20., -20., -20., 1., 0., -20., 20., -20., 0., 1., -20., 20., -20., 0., 1., 20.,
            -20., -20., 1., 0., 20., 20., -20., 1., 1., -20., -20., 20., 0., 0., 20., -20., 20.,
            1., 0., -20., 20., 20., 0., 1., -20., 20., 20., 0., 1., 20., -20., 20., 1., 0., 20.,
            20., 20., 1., 1.,
        ];

        unsafe {
            gl::glEnableClientState(gl::GL_VERTEX_ARRAY);
            gl::glEnableClientState(gl::GL_TEXTURE_COORD_ARRAY);
            gl::glVertexPointer(
                3,
                gl::GL_FLOAT,
                5 * size_of::<gl::GLfloat>() as i32,
                cube.as_ptr() as *const c_void,
            );
            gl::glTexCoordPointer(
                2,
                gl::GL_FLOAT,
                5 * size_of::<gl::GLfloat>() as i32,
                cube.as_ptr().offset(3) as *const c_void,
            );

            // Disable normal and color vertex components
            gl::glDisableClientState(gl::GL_NORMAL_ARRAY);
            gl::glDisableClientState(gl::GL_COLOR_ARRAY);
        }

        window.set_active(false);
        let clock = Clock::start();
        let mut mipmap_enabled = true;

        while window.is_open() {
            while let Some(event) = window.poll_event() {
                match event {
                    Event::Closed
                    | Event::KeyPressed {
                        code: Key::Escape, ..
                    } => {
                        exit = true;
                        window.close();
                    }
                    Event::KeyPressed {
                        code: Key::Enter, ..
                    } => {
                        if mipmap_enabled {
                            texture = Texture::from_file("resources/texture.jpg").unwrap();
                            mipmap_enabled = false;
                            window.set_active(true);
                            Texture::bind(&texture);
                            window.set_active(false);
                        } else {
                            texture.generate_mipmap();
                            mipmap_enabled = true;
                        }
                    }
                    Event::KeyPressed {
                        code: Key::Space, ..
                    } => {
                        srgb = !srgb;
                        window.close();
                    }
                    Event::Resized { width, height } => {
                        window.set_active(true);
                        unsafe {
                            gl::glViewport(0, 0, width as _, height as _);
                        }
                        window.set_active(false);
                    }
                    _ => {}
                }
            }
            window.push_gl_states();
            window.draw(&bg_sprite);
            window.pop_gl_states();

            window.set_active(true);

            unsafe {
                gl::glClear(gl::GL_DEPTH_BUFFER_BIT);
                let x: f32 =
                    window.mouse_position().x as f32 * 200. / window.size().x as f32 - 100.;
                let y: f32 =
                    -window.mouse_position().y as f32 * 200. / window.size().y as f32 + 100.;

                gl::glMatrixMode(gl::GL_MODELVIEW);
                gl::glLoadIdentity();
                gl::glTranslatef(x, y, -100.);
                gl::glRotatef(clock.elapsed_time().as_seconds() * 50., 1., 0., 0.);
                gl::glRotatef(clock.elapsed_time().as_seconds() * 30., 0., 1., 0.);
                gl::glRotatef(clock.elapsed_time().as_seconds() * 90., 0., 0., 1.);
                gl::glDrawArrays(gl::GL_TRIANGLES, 0, 36);
            }
            window.set_active(false);
            window.push_gl_states();
            window.draw(&text);
            window.draw(&srgb_instr);
            window.draw(&mipmap_instr);
            window.pop_gl_states();
            window.display();
        }
    }
}
