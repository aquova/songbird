CARGO = cargo
WASM_PACK = wasm-pack
WASM_TARGET = web

all: sdl wasm

sdl:
	@cd sdl && \
	$(CARGO) build

wasm:
	@cd wasm && \
	$(WASM_PACK) build --target $(WASM_TARGET) && \
	@mv pkg/agba_wasm_bg.wasm ../web && \
	@mv pkg/agba_wasm.js ../web

clean: clean_core clean_sdl clean_wasm

clean_core:
	@cd core && \
	$(CARGO) clean

clean_wasm:
	rm -f web/agba_wasm_bg.wasm
	rm -f web/agba_wasm.js
	rm -rf wasm/pkg
	@cd wasm && \
	$(CARGO) clean

clean_sdl:
	@cd sdl && \
	$(CARGO) clean

.PHONY: all sdl wasm clean
