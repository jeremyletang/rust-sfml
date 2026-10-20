# AGENTS.md

> **Any new agent MUST update this file if they feel their changes impact this summary, or if they add a new file.**

> **Any agent that modifies CSFML code MUST run `./format.sh` (from `CSFML/`) after finishing their changes.**

Summary of the `CSFML/` directory. It is a modified C++ shim over SFML 3.1 that exposes a C ABI for rust-sfml. Each `.cpp` provides `extern "C"` functions (or a C-ABI wrapper class). Each `.hpp` provides the C-compatible types and conversion helpers used by those `.cpp` files.

## Top level

- `README.md`: States this is a modified CSFML used internally by rust-sfml, and its design philosophy (minimal C++, safety checks in Rust).
- `format.sh`: Runs `clang-format` over `src/`.
- `license.txt`: CSFML license.

## src/Audio

- `PlaybackDevice.cpp`: Audio playback device enumeration and selection.
- `SoundBuffer.cpp`: `sf::SoundBuffer` wrapper (loaded audio data).
- `CustomSoundStream.cpp`: `sf::SoundStream` subclass that pulls samples from Rust callbacks.
- `SoundRecorder.cpp`: `sf::SoundRecorder` wrapper for audio capture.
- `Sound.cpp`: `sf::Sound` wrapper (playback of a sound buffer).
- `Listener.cpp`: Global 3D audio listener settings.
- `SoundBufferRecorder.cpp`: Recorder that stores captured audio in a `SoundBuffer`.
- `SoundChannel.hpp`: C enum of speaker channel layouts (`sfSoundChannel`).
- `SoundChannel.cpp`: C accessors for `std::vector<sf::SoundChannel>`.
- `SoundStatus.hpp`: C enum of playback states (stopped, paused, playing).
- `EffectProcessor.hpp`: C callback typedef for custom audio effect processing.
- `CustomSoundRecorder.cpp`: `sf::SoundRecorder` subclass that forwards captured samples to Rust callbacks.
- `SoundSourceCone.hpp`: C struct for the directional cone of a sound source, plus converters.
- `Music.cpp`: `sf::Music` wrapper (streamed audio from file/memory).

## src/Graphics

- `VertexBuffer.cpp`: `sf::VertexBuffer` wrapper.
- `TextEnums.hpp`: C enums for text line alignment and orientation.
- `Text.cpp`: `sf::Text` wrapper.
- `BlendMode.hpp`: C enums for blend factors and equations.
- `View.cpp`: `sf::View` wrapper (camera/viewport).
- `Sprite.cpp`: `sf::Sprite` wrapper.
- `Transform.cpp`: `sf::Transform` wrapper (2D matrix operations).
- `Rect.hpp`: C float/int rect structs and converters to and from `sf::Rect`.
- `StencilMode.hpp`: C enums for stencil comparison and update operations.
- `RenderTexture.cpp`: `sf::RenderTexture` wrapper (off-screen rendering).
- `Font.cpp`: `sf::Font` wrapper.
- `RectangleShape.cpp`: `sf::RectangleShape` wrapper.
- `Shader.cpp`: `sf::Shader` wrapper (GLSL shaders).
- `Image.cpp`: `sf::Image` wrapper (CPU-side pixel data).
- `CoordinateType.hpp`: C enum for normalized vs. pixel texture coordinates.
- `CircleShape.cpp`: `sf::CircleShape` wrapper.
- `CustomShape.cpp`: `sf::Shape` subclass whose points come from Rust callbacks.
- `ConvexShape.cpp`: `sf::ConvexShape` wrapper.
- `Color.hpp`: C color struct and converters to and from `sf::Color`.
- `PrimitiveType.hpp`: C enum for primitive types (points, lines, triangles, ...).
- `RenderWindow.cpp`: `sf::RenderWindow` wrapper.
- `RenderStates.hpp`: C struct bundling blend mode, transform, texture, shader, and stencil state.
- `Texture.cpp`: `sf::Texture` wrapper.
- `Transform.hpp`: C 3x3 transform struct and converters to and from `sf::Transform`.

## src/System

- `Vector3.hpp`: C `sfVector3*` structs and converters.
- `Vector2.hpp`: C `sfVector2*` structs and converters.
- `SfString.cpp`: C functions for creating and reading `sf::String`.
- `SfStdVector.cpp`: C functions for creating and reading `std::vector<std::string>`.
- `InputStreamHelper.cpp`: `sf::InputStream` implementation that forwards reads, seeks, and tells to Rust callbacks.
- `Buffer.cpp`: Destructor for the byte buffer (`sfBuffer`).
- `SfStdString.cpp`: C functions for creating and reading `std::string`.
- `Sleep.cpp`: Thread sleep function.
- `Version.cpp`: SFML version query.
- `Clock.cpp`: `sf::Clock` wrapper.
- `Buffer.hpp`: Declares `sfBuffer`, a `std::vector<uint8_t>` subclass.
- `InputStreamHelper.hpp`: Callback typedefs and the `sfInputStreamHelper` class declaration.

## src/Window

- `VideoMode.cpp`: `sf::VideoMode` queries (desktop and fullscreen modes).
- `Joystick.cpp`: Joystick state queries.
- `Mouse.hpp`: C enum of mouse buttons and wheels.
- `Cursor.cpp`: `sf::Cursor` wrapper (system and custom cursors).
- `Window.hpp`: C `sfState` enum (windowed or fullscreen) and its converter.
- `Cursor.hpp`: C enum of system cursor types.
- `Mouse.cpp`: Mouse state and cursor position functions.
- `Window.cpp`: `sf::Window` wrapper, including `createVulkanSurface`.
- `Vulkan.cpp`: Vulkan availability check, function address lookup, and the instance extensions that graphics rendering requires.
- `Sensor.hpp`: C enum of sensor types.
- `VideoMode.hpp`: C struct for video modes and converters.
- `Context.cpp`: OpenGL context creation and management.
- `Event.hpp`: C event structs and converters for window events.
- `Keyboard.cpp`: Keyboard state queries.
- `Clipboard.cpp`: Clipboard get and set functions.
- `Sensor.cpp`: Sensor state queries.
- `Keyboard.hpp`: C enum of keyboard keys.
- `Joystick.hpp`: C enum of joystick axes (`sfJoystickAxis`).
