use std::{
    env,
    path::{Path, PathBuf},
};

fn csfml_build_symlink() {
    if env::var("SYMLINK_CSFML").is_err() {
        return;
    }

    let Ok(out_dir) = env::var("OUT_DIR") else {
        return;
    };

    let out_build_directory = &format!("{out_dir}/build");
    let out_build_path = Path::new(out_build_directory);
    if !out_build_path.is_dir() {
        return;
    }

    let csfml_symlink_path = Path::new("./CSFML/build");
    let _ = std::fs::remove_file(csfml_symlink_path);
    #[cfg(unix)]
    let _ = std::os::unix::fs::symlink(out_build_path, &csfml_symlink_path);
    #[cfg(windows)]
    let _ = std::os::windows::fs::symlink_dir(out_build_path, &csfml_symlink_path);
}

fn static_link_windows(
    feat_window: bool,
    feat_audio: bool,
    feat_graphics: bool,
    env: WinEnv,
    build_lib_path: &Path,
) {
    let arch = match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
        Ok("x86") => "x86",
        Ok("x86_64") => "x64",
        _ => panic!("Failed to determine cpu arch (CARGO_CFG_TARGET_ARCH))"),
    };
    let sfml_extlibs_path: PathBuf = format!(
        "SFML/extlibs/libs-{seg}/{arch}",
        seg = env.sfml_extlib_name()
    )
    .into();
    // Copy only needed SFML extlibs to out dir to avoid linking every ext lib.
    // We don't need libFLAC, because we build it ourselves.
    for lib in ["freetype", "openal32", "vorbisenc", "vorbisfile", "vorbis"] {
        let lib_name = env.lib_filename(lib);
        let from = sfml_extlibs_path.join(&lib_name);
        let to = build_lib_path.join(&lib_name);
        let result = std::fs::copy(&from, &to);
        if let Err(e) = result {
            println!("cargo:warning=Failed to copy {from:?} to {to:?}: {e}",);
        }
    }
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
        unix_graphics_link_support_libs();
    }
    if feat_audio {
        // We link openal separately because Mac Os has its own OpenAL framework
        pkgconfig_probe_with_fallback("openal", "rustc-link-lib=dylib=openal");
        unix_audio_link_support_libs();
    }
}

fn pkgconfig_probe_with_fallback(lib: &str, fallback: &str) {
    if let Err(e) = pkg_config::probe_library(lib) {
        eprintln!("cargo:warning=pkg-config failed: {e}.\nTrying manual link");
        println!("cargo:{fallback}");
    }
}

/// Link supporting libraries for graphics on unix platforms (currently only freetype)
fn unix_graphics_link_support_libs() {
    pkgconfig_probe_with_fallback("freetype2", "rustc-link-lib=dylib=freetype");
}

fn unix_audio_link_support_libs() {
    pkgconfig_probe_with_fallback("vorbisenc", "rustc-link-lib=dylib=vorbisenc");
    pkgconfig_probe_with_fallback("vorbisfile", "rustc-link-lib=dylib=vorbisfile");
    pkgconfig_probe_with_fallback("vorbis", "rustc-link-lib=dylib=vorbis");
    // Odd that we have to do this, I thought that libflac-sys would do this for us
    println!("cargo:rustc-link-lib=static=FLAC");
    println!("cargo:rustc-link-lib=static=ogg");
}

enum WinEnv {
    Gnu,
    Msvc,
}

impl WinEnv {
    fn get() -> Option<Self> {
        match env::var("CARGO_CFG_TARGET_ENV").as_deref() {
            Ok("gnu") => Some(Self::Gnu),
            Ok("msvc") => Some(Self::Msvc),
            _ => None,
        }
    }
    fn sfml_extlib_name(&self) -> &'static str {
        match self {
            WinEnv::Gnu => "mingw",
            WinEnv::Msvc => "msvc",
        }
    }
    fn lib_ext(&self) -> &'static str {
        match self {
            WinEnv::Gnu => ".a",
            WinEnv::Msvc => ".lib",
        }
    }
    fn lib_prefix(&self) -> &'static str {
        match self {
            WinEnv::Gnu => "lib",
            WinEnv::Msvc => "",
        }
    }
    fn lib_filename(&self, lib: &str) -> String {
        [self.lib_prefix(), lib, self.lib_ext()].concat()
    }
}

fn main() {
    println!("cargo:rerun-if-changed=CSFML");
    println!("cargo:rerun-if-changed=SFML");
    let feat_audio = env::var("CARGO_FEATURE_AUDIO").is_ok();
    let feat_window = env::var("CARGO_FEATURE_WINDOW").is_ok();
    let feat_graphics = env::var("CARGO_FEATURE_GRAPHICS").is_ok();
    let mut cmake = cmake::Config::new("SFML");
    let win_env = WinEnv::get();
    let release_profile = env::var("PROFILE").is_ok_and(|prof| prof == "release");
    // Due to complications with static linking of MSVC runtime (debug version),
    // we cannot support debug builds of SFML.
    cmake.profile("Release");
    cmake
        .define("CMAKE_FIND_DEBUG_MODE", "TRUE") // I think I'll leave this on for now. Useful for debugging.
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON")
        .define("BUILD_SHARED_LIBS", "FALSE")
        .define("SFML_BUILD_NETWORK", "FALSE")
        .define("SFML_INSTALL_PKGCONFIG_FILES", "FALSE")
        // Disable "install" step
        .no_build_target(true);
    if !feat_audio {
        cmake.define("SFML_BUILD_AUDIO", "FALSE");
    } else {
        // Add search path for libFLAC built by libflac-sys
        let (libogg_loc, libflac_loc) = match win_env {
            Some(WinEnv::Msvc) => {
                let libflac_loc = if release_profile {
                    "/build/src/libFLAC/Release"
                } else {
                    "/build/src/libFLAC/Debug"
                };
                ("/lib", libflac_loc)
            }
            _ => ("/lib", "/build/src/libFLAC"),
        };
        match env::var("DEP_FLAC_ROOT") {
            Ok(libflac_root) => {
                cmake.define(
                    "CMAKE_PREFIX_PATH",
                    // We add both the path to libogg and libFLAC. Two separate paths, separated by `;`.
                    [&libflac_root, libogg_loc, ";", &libflac_root, libflac_loc].concat(),
                );
            }
            Err(e) => {
                println!(
                    "cargo:warning=Failed to get DEP_FLAC_ROOT: {e}.\n\
                          Now the build will horribly break.\n\
                          Except maybe on CI. ;)"
                );
            }
        }
    }
    if !feat_window {
        cmake.define("SFML_BUILD_WINDOW", "FALSE");
    }
    if !feat_graphics {
        cmake.define("SFML_BUILD_GRAPHICS", "FALSE");
    }
    let cmake_build_path = cmake.build();
    csfml_build_symlink();
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .flag_if_supported("--std=c++17")
        .define("CSFML_SYSTEM_EXPORTS", None)
        .define("CSFML_AUDIO_EXPORTS", None)
        .define("CSFML_WINDOW_EXPORTS", None)
        .define("CSFML_GRAPHICS_EXPORTS", None)
        .define("SFML_STATIC", None)
        .include("CSFML/src/")
        .include("SFML/include");
    build.files(
        [
            "CSFML/src/System/Clock.cpp",
            "CSFML/src/System/Sleep.cpp",
            "CSFML/src/System/InputStreamHelper.cpp",
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
                "CSFML/src/Audio/CustomSoundRecorder.cpp",
                "CSFML/src/Audio/CustomSoundStream.cpp",
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
                "CSFML/src/Window/Context.cpp",
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
                "CSFML/src/Graphics/CustomShape.cpp",
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
    // Need to probe Cargo's env as build.rs uses the default toolchain to
    // run the build meaning that #[cfg(..)]'s won't work
    let is_windows = env::var("CARGO_CFG_WINDOWS").is_ok();
    let is_unix = env::var("CARGO_CFG_UNIX").is_ok();
    let is_linux = env::var("CARGO_CFG_TARGET_OS")
        .map(|os| os == "linux")
        .unwrap_or(false);
    let is_macos = env::var("CARGO_CFG_TARGET_OS")
        .map(|os| os == "macos")
        .unwrap_or(false);
    if is_macos {
        link_mac_os_frameworks();
    }
    let link_search = if matches!(win_env, Some(WinEnv::Msvc)) {
        "build/lib/Release"
    } else {
        "build/lib"
    };
    let build_lib_path = cmake_build_path.join(link_search);
    println!(
        "cargo:rustc-link-search=native={}",
        build_lib_path.display()
    );
    println!("cargo:rustc-link-lib=static=rcsfml");
    link_sfml_subsystem("system");
    if is_unix && is_linux {
        static_link_linux(feat_window, feat_audio, feat_graphics);
    } else if is_windows {
        match win_env {
            Some(env) => {
                static_link_windows(feat_window, feat_audio, feat_graphics, env, &build_lib_path);
            }
            None => {
                panic!("Failed to determine windows environment (MSVC/Mingw)");
            }
        }
    } else if is_macos {
        // Link freetype for mac
        if feat_graphics {
            unix_graphics_link_support_libs();
        }
        if feat_audio {
            unix_audio_link_support_libs();
        }
        // SFML contains Objective-C source files on OSX
        // https://github.com/SFML/SFML/issues/2920
        println!("cargo::rustc-link-arg=-ObjC");
    } else {
        panic!("Uhhh... Can't determine your environment. Sorry.");
    }
    if feat_audio {
        link_sfml_subsystem("audio");
    }
    if feat_window {
        link_sfml_subsystem("window");
    }
    if feat_graphics {
        link_sfml_subsystem("graphics");
    }
}

fn link_sfml_subsystem(name: &str) {
    println!("cargo:rustc-link-lib=static=sfml-{name}-s");
}

fn link_mac_os_frameworks() {
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=CoreGraphics");
    println!("cargo:rustc-link-lib=framework=IOKit");
    println!("cargo:rustc-link-lib=framework=Carbon");
    println!("cargo:rustc-link-lib=framework=AppKit");
    println!("cargo:rustc-link-lib=framework=OpenAL");
}
