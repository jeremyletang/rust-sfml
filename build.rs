use std::env;

fn static_link_windows(feat_window: bool, feat_audio: bool, feat_graphics: bool) {
    let env = match std::env::var("CARGO_CFG_TARGET_ENV").as_deref() {
        Ok("gnu") => "mingw",
        Ok("msvc") => "msvc",
        _ => {
            panic!("Failed to determine windows environment (CARGO_CFG_TARGET_ENV))")
        }
    };
    let arch = match std::env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
        Ok("x86") => "x86",
        Ok("x86_64") => "x64",
        _ => panic!("Failed to determine cpu arch (CARGO_CFG_TARGET_ARCH))"),
    };
    println!("cargo:rustc-link-search=native=SFML/extlibs/libs-{env}/{arch}");
    println!("cargo:rustc-link-lib=dylib=winmm");
    println!("cargo:rustc-link-lib=dylib=user32");
    if feat_window {
        println!("cargo:rustc-link-lib=dylib=opengl32");
        println!("cargo:rustc-link-lib=dylib=gdi32");
    }
    if feat_graphics {
        println!("cargo:rustc-link-lib=static=freetype");
    }
    if feat_audio {
        println!("cargo:rustc-link-lib=static=openal32");
        println!("cargo:rustc-link-lib=static=FLAC");
        println!("cargo:rustc-link-lib=static=vorbisenc");
        println!("cargo:rustc-link-lib=static=vorbisfile");
        println!("cargo:rustc-link-lib=static=vorbis");
        println!("cargo:rustc-link-lib=static=ogg");
    }
}

fn static_link_linux(feat_window: bool, feat_audio: bool, feat_graphics: bool) {
    println!("cargo:rustc-link-lib=dylib=udev");
    if feat_window {
        println!("cargo:rustc-link-lib=dylib=GL");
        println!("cargo:rustc-link-lib=dylib=X11");
        println!("cargo:rustc-link-lib=dylib=Xcursor");
        println!("cargo:rustc-link-lib=dylib=Xrandr");
    }
    if feat_graphics {
        println!("cargo:rustc-link-lib=dylib=freetype");
    }
    if feat_audio {
        println!("cargo:rustc-link-lib=dylib=openal");
        println!("cargo:rustc-link-lib=dylib=FLAC");
        println!("cargo:rustc-link-lib=dylib=vorbisenc");
        println!("cargo:rustc-link-lib=dylib=vorbisfile");
        println!("cargo:rustc-link-lib=dylib=vorbis");
        println!("cargo:rustc-link-lib=dylib=ogg");
    }
}

fn main() {
    println!("cargo:rerun-if-changed=CSFML");
    let feat_audio = env::var("CARGO_FEATURE_AUDIO").is_ok();
    let feat_window = env::var("CARGO_FEATURE_WINDOW").is_ok();
    let feat_graphics = env::var("CARGO_FEATURE_GRAPHICS").is_ok();
    let mut cmake = cmake::Config::new("SFML");
    cmake
        .define("BUILD_SHARED_LIBS", "FALSE")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("SFML_BUILD_NETWORK", "FALSE")
        .define("SFML_INSTALL_PKGCONFIG_FILES", "FALSE");
    if !feat_audio {
        cmake.define("SFML_BUILD_AUDIO", "FALSE");
    }
    if !feat_window {
        cmake.define("SFML_BUILD_WINDOW", "FALSE");
    }
    if !feat_graphics {
        cmake.define("SFML_BUILD_GRAPHICS", "FALSE");
    }
    let path = cmake.build();
    dbg!(&path);
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .flag_if_supported("--std=c++17")
        .define("CSFML_SYSTEM_EXPORTS", None)
        .define("CSFML_AUDIO_EXPORTS", None)
        .define("CSFML_WINDOW_EXPORTS", None)
        .define("CSFML_GRAPHICS_EXPORTS", None)
        .include("CSFML/src/")
        .include("SFML/include");
    build.define("SFML_STATIC", None).static_crt(true);
    build.files(
        [
            "CSFML/src/System/Clock.cpp",
            "CSFML/src/System/Sleep.cpp",
            "CSFML/src/System/InputStream.cpp",
            "CSFML/src/System/SfString.cpp",
            "CSFML/src/System/SfStdString.cpp",
            "CSFML/src/System/SfStdVector.cpp",
        ]
        .iter(),
    );
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

    println!(
        "cargo:rustc-link-search=native={}",
        path.join("build/lib").display()
    );
    println!("cargo:rustc-link-lib=static=rcsfml");
    // Need to probe Cargo's env as build.rs uses the default toolchain to
    // run the build meaning that #[cfg(..)]'s won't work
    let is_windows = env::var("CARGO_CFG_WINDOWS").is_ok();
    let is_unix = env::var("CARGO_CFG_UNIX").is_ok();
    let is_linux = env::var("CARGO_CFG_TARGET_OS")
        .map(|os| os == "linux")
        .unwrap_or(false);

    println!("cargo:rustc-link-lib=static=sfml-system-s");
    if is_unix && is_linux {
        static_link_linux(feat_window, feat_audio, feat_graphics);
    } else if is_windows {
        static_link_windows(feat_window, feat_audio, feat_graphics);
    }
    if feat_audio {
        println!("cargo:rustc-link-lib=static=sfml-audio-s");
    }
    if feat_window {
        println!("cargo:rustc-link-lib=static=sfml-window-s");
    }
    if feat_graphics {
        println!("cargo:rustc-link-lib=static=sfml-graphics-s");
    }
}
