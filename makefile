
RED=\033[0;31m
NC=\033[0m # No Color

TARGET_OS := $(shell uname)
ifeq ($(TARGET),)
    TARGET := $(TARGET_OS)
endif

ifeq ($(TARGET),Linux)
    EXT := so
	TARGET_BUILD := x86_64-unknown-linux-gnu
else ifeq ($(TARGET),Darwin)
    EXT := dylib
	TARGET_BUILD := x86_64-apple-darwin
else ifeq ($(TARGET),Windows_NT)
    EXT := dll
	TARGET_BUILD := x86_64-pc-windows-gnu
else ifeq ($(TARGET),)
    $(error Unsupported target OS: $(TARGET))
endif



debug : 
	cp $(DISCORD_GAME_SDK_PATH)/lib/x86_64/discord_game_sdk.so $(DISCORD_GAME_SDK_PATH)/lib/x86_64/libdiscord_game_sdk.so
	export LD_LIBRARY_PATH=$(LD_LIBRARY_PATH):$(DISCORD_GAME_SDK_PATH)/lib/x86_64

	@echo "$(RED)<==building for $(TARGET)==> $(NC)"	
	cargo build --target $(TARGET_BUILD)

release :
	cp $(DISCORD_GAME_SDK_PATH)/lib/x86_64/discord_game_sdk.so $(DISCORD_GAME_SDK_PATH)/lib/x86_64/libdiscord_game_sdk.so
	export LD_LIBRARY_PATH=$(LD_LIBRARY_PATH):$(DISCORD_GAME_SDK_PATH)/lib/x86_64

	@echo "$(RED)<==building for $(TARGET)==> $(NC)"
	cargo build --release --target $(TARGET_BUILD)
	
prebuild:
	@printf "prebuild!\n"
	ifeq ($(TARGET),Linux)
	$(shell cp $DISCORD_GAME_SDK_PATH/lib/x86_64/{,lib}discord_game_sdk.so)
	$(shell export LD_LIBRARY_PATH=${LD_LIBRARY_PATH:+${LD_LIBRARY_PATH}:}$DISCORD_GAME_SDK_PATH/lib/x86_64)
	
	

	else
	@echo (error prebuild: Unsupported OS $(TARGET))
	endif

clean :
	cargo clean
	

run : debug
	$(shell luajit ./discord_integration.lua)