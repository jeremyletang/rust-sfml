# Rust-SFML - Copyright (c) 2013 Letang Jeremy.
#
# The original software, SFML library, is provided by Laurent Gomila.
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

TARGET =		rsfml

ROOT_DIR =		.
SRC_DIR =		$(ROOT_DIR)/src
BUILD_DIR =		$(ROOT_DIR)/lib

DEMO =			demo
DEMO_DIR =		$(ROOT_DIR)/examples

RM =			rm -rf


all :
			@echo "Building $(TARGET)..."
			@mkdir -p $(BUILD_DIR)
			@rustc $(SRC_DIR)/$(TARGET).rc --out-dir=$(BUILD_DIR)
			@echo "Build success."

$(DEMO) :
			@echo "Building examples..."
			@echo "Building pong example..."
			@rustc $(DEMO_DIR)/pong/pong.rs -L $(BUILD_DIR)
			@echo "Building sound example..."
			@rustc $(DEMO_DIR)/sound/sound.rs -L $(BUILD_DIR)
			@echo "Building sound_capture example..."
			@rustc $(DEMO_DIR)/sound_capture/sound_capture.rs -L $(BUILD_DIR)
			@echo "Building examples success"


doc :
			@mkdir -p $(ROOT_DIR)/doc
			rustdoc --output-dir $(ROOT_DIR)/doc $(SRC_DIR)/$(TARGET).rc


clean :
			$(RM) $(BUILD_DIR)
			$(RM) $(DEMO_DIR)/pong/pong
			$(RM) $(DEMO_DIR)/sound/sound
			$(RM) $(DEMO_DIR)/sound_capture/sound_capture
			$(RM) $(ROOT_DIR)/doc