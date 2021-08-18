use std::env;

fn main() {
    println!("cargo:rerun-if-changed=CSFML");
    println!("cargo:rerun-if-env-changed=SFML_INCLUDE_DIR");
    println!("cargo:rerun-if-env-changed=SFML_LIBS_DIR");
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .define("CSFML_SYSTEM_EXPORTS", None)
        .define("CSFML_AUDIO_EXPORTS", None)
        .define("CSFML_WINDOW_EXPORTS", None)
        .define("CSFML_GRAPHICS_EXPORTS", None)
        .include("CSFML/src/")
        .include("CSFML/include/");
    if let Ok(sfml_inc_dir) = env::var("SFML_INCLUDE_DIR") {
        println!("cargo:warning=Custom SFML include dir: {}", sfml_inc_dir);
        build.include(sfml_inc_dir);
    }
    let feat_audio = env::var("CARGO_FEATURE_AUDIO").is_ok();
    let feat_window = env::var("CARGO_FEATURE_WINDOW").is_ok();
    let feat_graphics = env::var("CARGO_FEATURE_GRAPHICS").is_ok();
    build.files(
        [
            "CSFML/src/SFML/System/Clock.cpp",
            "CSFML/src/SFML/System/Mutex.cpp",
            "CSFML/src/SFML/System/Sleep.cpp",
            "CSFML/src/SFML/System/Thread.cpp",
            "CSFML/src/SFML/System/Time.cpp",
        ]
        .iter(),
    );
    if feat_audio {
        build.files(
            [
                "CSFML/src/SFML/Audio/Listener.cpp",
                "CSFML/src/SFML/Audio/Music.cpp",
                "CSFML/src/SFML/Audio/Sound.cpp",
                "CSFML/src/SFML/Audio/SoundBuffer.cpp",
                "CSFML/src/SFML/Audio/SoundBufferRecorder.cpp",
                "CSFML/src/SFML/Audio/SoundRecorder.cpp",
                "CSFML/src/SFML/Audio/SoundStream.cpp",
            ]
            .iter(),
        );
    }
    if feat_window {
        build.files(
            [
                "CSFML/src/SFML/Window/Clipboard.cpp",
                "CSFML/src/SFML/Window/Context.cpp",
                "CSFML/src/SFML/Window/Cursor.cpp",
                "CSFML/src/SFML/Window/Joystick.cpp",
                "CSFML/src/SFML/Window/Keyboard.cpp",
                "CSFML/src/SFML/Window/Mouse.cpp",
                "CSFML/src/SFML/Window/Sensor.cpp",
                "CSFML/src/SFML/Window/Touch.cpp",
                "CSFML/src/SFML/Window/VideoMode.cpp",
                "CSFML/src/SFML/Window/Window.cpp",
            ]
            .iter(),
        );
    }
    if feat_graphics {
        build.files(
            [
                "CSFML/src/SFML/Graphics/BlendMode.cpp",
                "CSFML/src/SFML/Graphics/CircleShape.cpp",
                "CSFML/src/SFML/Graphics/Color.cpp",
                "CSFML/src/SFML/Graphics/ConvexShape.cpp",
                "CSFML/src/SFML/Graphics/Font.cpp",
                "CSFML/src/SFML/Graphics/Image.cpp",
                "CSFML/src/SFML/Graphics/Rect.cpp",
                "CSFML/src/SFML/Graphics/RectangleShape.cpp",
                "CSFML/src/SFML/Graphics/RenderTexture.cpp",
                "CSFML/src/SFML/Graphics/RenderWindow.cpp",
                "CSFML/src/SFML/Graphics/Shader.cpp",
                "CSFML/src/SFML/Graphics/Shape.cpp",
                "CSFML/src/SFML/Graphics/Sprite.cpp",
                "CSFML/src/SFML/Graphics/Text.cpp",
                "CSFML/src/SFML/Graphics/Texture.cpp",
                "CSFML/src/SFML/Graphics/Transform.cpp",
                "CSFML/src/SFML/Graphics/Transformable.cpp",
                "CSFML/src/SFML/Graphics/VertexArray.cpp",
                "CSFML/src/SFML/Graphics/VertexBuffer.cpp",
                "CSFML/src/SFML/Graphics/View.cpp",
            ]
            .iter(),
        );
    }
    build.compile("rcsfml");

    if let Ok(libs_dir) = env::var("SFML_LIBS_DIR") {
        println!(
            "cargo:warning=Adding custom SFML libs search path {}",
            libs_dir
        );
        println!("cargo:rustc-link-search=native={}", libs_dir);
    }
    println!("cargo:rustc-link-lib=static=rcsfml");
    println!("cargo:rustc-link-lib=dylib=sfml-system");
    if feat_audio {
        println!("cargo:rustc-link-lib=dylib=sfml-audio");
    }
    if feat_window {
        println!("LINKING WINDOW",);
        println!("cargo:rustc-link-lib=dylib=sfml-window");
    }
    if feat_graphics {
        println!("LINKING GRAPHICS",);
        println!("cargo:rustc-link-lib=dylib=sfml-graphics");
    }
}
