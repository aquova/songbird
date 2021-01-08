CARGO = cargo
WASM_PACK = wasm-pack
WASM_TARGET = web
WIN_TARGET = x86_64-pc-windows-gnu
ARM_TARGET = arm-unknown-linux-gnueabihf
REL_FLAGS = --release
DEBUG_FLAGS = --features "debug"

all: imgui wasm windows arm term debug

imgui:
	cd imgui && \
	$(CARGO) build $(REL_FLAGS)

wasm:
	cd wasm && \
	$(WASM_PACK) build --target $(WASM_TARGET) && \
	mv pkg/songbird_wasm_bg.wasm ../web && \
	mv pkg/songbird_wasm.js ../web

windows:
	cd imgui && \
	$(CARGO) build --target $(WIN_TARGET) $(REL_FLAGS)

arm:
	cd imgui && \
	$(CARGO) build --target $(ARM_TARGET) $(REL_FLAGS)

term:
	cd term && \
	$(CARGO) build $(REL_FLAGS)

debug:
	cd imgui && \
	$(CARGO) build $(DEBUG_FLAGS)

clean: clean_core clean_imgui clean_wasm clean_term

clean_core:
	cd core && \
	$(CARGO) clean

clean_wasm:
	rm -f web/songbird_wasm_bg.wasm
	rm -f web/songbird_wasm.js
	rm -rf wasm/pkg
	cd wasm && \
	$(CARGO) clean

clean_imgui:
	cd imgui && \
	$(CARGO) clean && \
	rm -f songbird_imgui.exe
	rm -f imgui.ini

clean_term:
	cd term && \
	$(CARGO) clean

.PHONY: all imgui wasm windows arm term debug clean
