CARGO = cargo
WASM_PACK = wasm-pack
WASM_TARGET = web
WIN_TARGET = x86_64-pc-windows-gnu

all: sdl wasm windows

sdl:
	cd sdl && \
	$(CARGO) build

wasm:
	cd wasm && \
	$(WASM_PACK) build --target $(WASM_TARGET) && \
	mv pkg/agba_wasm_bg.wasm ../web && \
	mv pkg/agba_wasm.js ../web

windows:
	cd sdl && \
	$(CARGO) build --target $(WIN_TARGET) && \
	mv ./target/x86_64-pc-windows-gnu/debug/agba_sdl.exe .

clean: clean_core clean_sdl clean_wasm clean_windows

clean_core:
	cd core && \
	$(CARGO) clean

clean_wasm:
	rm -f web/agba_wasm_bg.wasm
	rm -f web/agba_wasm.js
	rm -rf wasm/pkg
	cd wasm && \
	$(CARGO) clean

clean_windows:
	cd sdl && \
	$(CARGO) clean && \
	rm -f agba_sdl.exe

clean_sdl:
	cd sdl && \
	$(CARGO) clean

.PHONY: all sdl wasm windows clean
