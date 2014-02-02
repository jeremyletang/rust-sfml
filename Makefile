# Rust-SFML - Copyright (c) 2013 Letang Jeremy.
#
# The Original software, SFML library, is provided by Laurent Gomila.
#
# This software is provided 'as-is', without any express or implied warranty.
# In no event will the authors be held liable for any damages arising from
# the use of this software.
#
# Permission is granted to anyone to use this software for any purpose,
# including commercial applications, and to alter it and redistribute it
# freely, subject to the following restrictions:
#
# 1. The origin of this software must not be misrepresented; you must not claim
#    that you wrote the original software. If you use this software in a product,
#    an acknowledgment in the product documentation would be appreciated but is
#    not required.
#
# 2. Altered source versions must be plainly marked as such, and must not be
#    misrepresented as being the original software.
#
# 3. This notice may not be removed or altered from any source distribution.

all: lib examples doc

lib:
	mkdir -p lib
	rustc --out-dir=lib src/rsfml/lib.rs

doc:
	mkdir -p doc
	rustdoc -o doc src/rsfml/lib.rs

examples:
	rustc -o bin/pong -L ./lib src/examples/pong/main.rs
	rustc -o bin/shape -L ./lib src/examples/shape/main.rs
	rustc -o bin/sound -L ./lib src/examples/sound/main.rs
	rustc -o bin/sound_capture -L ./lib src/examples/sound_capture/main.rs


clean:
	rm -rf lib
	rm -rf doc
	rm -rf bin/pong
	rm -rf bin/shape
	rm -rf bin/sound
	rm -rf bin/sound_capture
