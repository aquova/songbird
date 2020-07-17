CARGO = cargo
WASM_PACK = wasm-pack
WASM_TARGET = web
WIN_TARGET = x86_64-pc-windows-gnu
REL_FLAGS = --release
DEBUG_FLAGS = --features "debug"

all: gui wasm windows debug

gui:
	cd gui && \
	$(CARGO) build $(REL_FLAGS)

wasm:
	cd wasm && \
	$(WASM_PACK) build --target $(WASM_TARGET) && \
	mv pkg/songbird_wasm_bg.wasm ../web && \
	mv pkg/songbird_wasm.js ../web

windows:
	cd gui && \
	$(CARGO) build --target $(WIN_TARGET) $(REL_FLAGS)

debug:
	cd gui && \
	$(CARGO) build $(DEBUG_FLAGS)

clean: clean_core clean_gui clean_wasm clean_windows

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
	cd gui && \
	$(CARGO) clean && \
	rm -f songbird_gui.exe

clean_gui:
	cd gui && \
	$(CARGO) clean

.PHONY: all gui wasm windows debug clean
