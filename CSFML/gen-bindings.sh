#!/bin/sh

OPTS="--no-doc-comments --with-derive-hash --with-derive-ord --with-derive-eq --size_t-is-usize -- -I include/ -xc++"
bindgen include/SFML/System.h $OPTS > ../src/ffi/system.rs
bindgen include/SFML/Audio.h $OPTS > ../src/ffi/audio.rs
bindgen include/SFML/Window.h $OPTS > ../src/ffi/window.rs
bindgen include/SFML/Graphics.h $OPTS > ../src/ffi/graphics.rs
