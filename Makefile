CARGO = cargo
WASM_PACK = wasm-pack
WASM_TARGET = web
WIN_TARGET = x86_64-pc-windows-gnu
ARM_TARGET = arm-unknown-linux-gnueabihf
REL_FLAGS = --release

all: gtk wasm windows arm

gtk:
	cd gtk && \
	$(CARGO) build $(REL_FLAGS)

wasm:
	cd wasm && \
	$(WASM_PACK) build --target=$(WASM_TARGET) && \
	mv pkg/songbird_wasm_bg.wasm ../web && \
	mv pkg/songbird_wasm.js ../web

windows:
	export PKG_CONFIG_ALLOW_CROSS=1 && \
	case "${1}" in \
		(--fedora|--docker) \
			export MINGW_PREFIX=/usr/x86_64-w64-mingw32/sys-root/mingw \
		;; \
		(--arch|--manjaro|''|*) \
			export MINGW_PREFIX=/usr/x86_64-w64-mingw32 \
		;; \
	esac && \
	export PKG_CONFIG_PATH=$(MINGW_PREFIX)/lib/pkgconfig && \
	cd gtk && \
	$(CARGO) build --target=$(WIN_TARGET) $(REL_FLAGS)

arm:
	cd gtk && \
	$(CARGO) build --target=$(ARM_TARGET) $(REL_FLAGS)

clean: clean_core clean_gtk clean_wasm

clean_core:
	cd core && \
	$(CARGO) clean

clean_wasm:
	rm -f web/songbird_wasm_bg.wasm
	rm -f web/songbird_wasm.js
	rm -rf wasm/pkg
	cd wasm && \
	$(CARGO) clean

clean_gtk:
	cd gtk && \
	$(CARGO) clean && \
	rm -f songbird_gtk.exe

.PHONY: all gtk wasm windows arm debug clean
