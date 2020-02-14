const COLORS = [
    "#000000",  // Black
    "#9494a5",  // Light Gray
    "#6b6b5a",  // Dark Gray
    "#ffffff"   // White
]

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

    // Setup canvas
    const canvas = document.getElementById("canvas")
    canvas.width = WIDTH * scale
    canvas.height = HEIGHT * scale
    const ctx = canvas.getContext("2d")

    load_game()

    // Load game
    function load_game() {
        // Load game into buffer
        fetch("../games/test_roms/opus5.gb")
            .then(i => i.arrayBuffer())
            .then(buffer => {
                const rom = new DataView(buffer, 0, buffer.byteLength)
                // Push data onto ROM vector
                for (var i = 0; i < rom.byteLength; i++) {
                    exports.push(rom.getUint8(i))
                }
                // Send data to emulator
                exports.load()
            })
    }

    // Draw canvas
    function draw_screen() {
        const gfx = new Uint8Array(exports.get_gfx())
        const palette = new Uint8Array(exports.get_palette())

        // Clear canvas
        ctx.fillStyle = "blue"
        ctx.fillRect(0, 0, WIDTH * scale, HEIGHT * scale)

        for (var y = 0; y < HEIGHT; y++) {
            for (var x = 0; x < WIDTH; x++) {
                var index = y * WIDTH + x
                var pixel = gfx[index]
                var color = COLORS[palette[pixel]]
                ctx.fillStyle = color
                ctx.fillRect(x * scale, y * scale, scale, scale)
            }
        }
    }

    // Main loop
    function runloop() {
        if (!paused) {
            // TODO: Need to run at 4.2 MHz
            var draw_time = exports.tick()
            // if (draw_time) {
            draw_screen()
            // }
        }
        window.requestAnimationFrame(runloop)
    }
    window.requestAnimationFrame(runloop)
}

run()
