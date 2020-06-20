CARGO = cargo
WASM_PACK = wasm-pack
WASM_TARGET = web
WIN_TARGET = x86_64-pc-windows-gnu
REL_FLAGS = --release
DEBUG_FLAGS = --features "debug"

all: sdl wasm windows debug

sdl:
	cd sdl && \
	$(CARGO) build $(REL_FLAGS)

wasm:
	cd wasm && \
	$(WASM_PACK) build --target $(WASM_TARGET) && \
	mv pkg/songbird_wasm_bg.wasm ../web && \
	mv pkg/songbird_wasm.js ../web

windows:
	cd sdl && \
	$(CARGO) build --target $(WIN_TARGET) $(REL_FLAGS) && \
	mv ./target/x86_64-pc-windows-gnu/debug/songbird_sdl.exe .

debug:
	cd sdl && \
	$(CARGO) build $(DEBUG_FLAGS)

clean: clean_core clean_sdl clean_wasm clean_windows

clean_core:
	cd core && \
	$(CARGO) clean

clean_wasm:
	rm -f web/songbird_wasm_bg.wasm
	rm -f web/songbird_wasm.js
	rm -rf wasm/pkg
	cd wasm && \
	$(CARGO) clean

clean_windows:
	cd sdl && \
	$(CARGO) clean && \
	rm -f songbird_sdl.exe

clean_sdl:
	cd sdl && \
	$(CARGO) clean && \
	rm -f core

.PHONY: all sdl wasm windows debug clean
