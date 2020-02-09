async function run() {
    const WIDTH = 160
    const HEIGHT = 144
    var scale = 5
    var paused = false

    // Load .wasm module
    const res = await fetch("agba_wasm.wasm")
    const buffer = await res.arrayBuffer()
    const module = await WebAssembly.compile(buffer)
    const instance = await WebAssembly.instantiate(module)
    // Exported functions are in 'exports' object
    const exports = instance.exports
    // load_game()

    // Get memory arrays
    // const gfx = new Uint8Array(exports.memory.buffer, exports.get_gfx(), HEIGHT * WIDTH)
    // const ram = new Uint8Array(exports.memory.buffer, exports.get_memory(), 4096)

    // Setup canvas
    const canvas = document.getElementById("canvas")
    canvas.width = WIDTH * scale
    canvas.height = HEIGHT * scale
    const ctx = canvas.getContext("2d")

    // Load game
    function load_game() {
        exports.load()
        // Load game into buffer
        // fetch("games/" + game_title)
        //     .then(i => i.arrayBuffer())
        //     .then(buffer => {
        //         const rom = new DataView(buffer, 0, buffer.byteLength)
        //         // Reset, then load game data into RAM
        //         exports.reset()
        //         for (var i = 0; i < rom.byteLength; i++) {
        //             ram[0x200 + i] = rom.getUint8(i)
        //         }
        //         // Set random seed
        //         exports.set_seed(Math.random())
        //     })
    }

    // Draw canvas
    function draw_screen() {
        const gfx = new Uint8Array(exports.memory.buffer, exports.get_gfx(), HEIGHT * WIDTH)

        // Background color
        ctx.fillStyle = "black"
        ctx.fillRect(0, 0, WIDTH * scale, HEIGHT * scale)
        ctx.fillStyle = color.value
        // Draw pixel by pixel
        for (var i = 0; i < gfx.length; i++) {
            var x = i % WIDTH
            var y = Math.floor(i / WIDTH)
            var pixel = gfx[i]
            if (pixel == 1) {
                ctx.fillRect(x * scale, y * scale, scale, scale)
            }
        }
    }

    // Main loop
    function runloop() {
        if (!paused) {
            // TODO: Need to run at 4.2 MHz
            exports.tick()
            draw_screen()
        }
        window.requestAnimationFrame(runloop)
    }
    window.requestAnimationFrame(runloop)

}

run()
