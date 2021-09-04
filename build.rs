use std::{env, path::PathBuf};

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
    build.files(["CSFML/src/System/Clock.cpp", "CSFML/src/System/Sleep.cpp"].iter());
    if feat_audio {
        build.files(
            [
                "CSFML/src/Audio/Listener.cpp",
                "CSFML/src/Audio/Music.cpp",
                "CSFML/src/Audio/Sound.cpp",
                "CSFML/src/Audio/SoundBuffer.cpp",
                "CSFML/src/Audio/SoundBufferRecorder.cpp",
                "CSFML/src/Audio/SoundRecorder.cpp",
                "CSFML/src/Audio/SoundStream.cpp",
            ]
            .iter(),
        );
    }
    if feat_window {
        build.files(
            [
                "CSFML/src/Window/Clipboard.cpp",
                "CSFML/src/Window/Context.cpp",
                "CSFML/src/Window/Cursor.cpp",
                "CSFML/src/Window/Joystick.cpp",
                "CSFML/src/Window/Keyboard.cpp",
                "CSFML/src/Window/Mouse.cpp",
                "CSFML/src/Window/Sensor.cpp",
                "CSFML/src/Window/Touch.cpp",
                "CSFML/src/Window/VideoMode.cpp",
                "CSFML/src/Window/Window.cpp",
            ]
            .iter(),
        );
    }
    if feat_graphics {
        build.files(
            [
                "CSFML/src/Graphics/CircleShape.cpp",
                "CSFML/src/Graphics/ConvexShape.cpp",
                "CSFML/src/Graphics/Font.cpp",
                "CSFML/src/Graphics/Image.cpp",
                "CSFML/src/Graphics/RectangleShape.cpp",
                "CSFML/src/Graphics/RenderTexture.cpp",
                "CSFML/src/Graphics/RenderWindow.cpp",
                "CSFML/src/Graphics/Shader.cpp",
                "CSFML/src/Graphics/Shape.cpp",
                "CSFML/src/Graphics/Sprite.cpp",
                "CSFML/src/Graphics/Text.cpp",
                "CSFML/src/Graphics/Texture.cpp",
                "CSFML/src/Graphics/Transform.cpp",
                "CSFML/src/Graphics/VertexBuffer.cpp",
                "CSFML/src/Graphics/View.cpp",
            ]
            .iter(),
        );
    }
    build.compile("rcsfml");

    let bindings = bindgen::Builder::default()
        .header("CSFML/bindgen-wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_args(["-I", "CSFML/include/", "-xc++"])
        .size_t_is_usize(true)
        .derive_hash(true)
        .derive_ord(true)
        .derive_eq(true)
        .generate()
        .expect("Failed to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

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
