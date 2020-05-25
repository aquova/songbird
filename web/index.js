import init, * as wasm from "./agba_wasm.js"

const COLORS = [
    "#000000",  // Black
    "#9494a5",  // Light Gray
    "#6b6b5a",  // Dark Gray
    "#ffffff"   // White
]

async function run() {
    await init()
    let gb = new wasm.GB();
    gb.init_cpu()

    await load_js(gb, "opus5.gb").then(() => {
        mainloop(gb)
    })
}

async function load_js(gb, game) {
    let res = await fetch(game)
    let buffer = await res.arrayBuffer()
    const rom = new DataView(buffer, 0, buffer.byteLength)
    gb.load_rom(rom)
}

function mainloop(gb) {
    // TODO: Need to run at 4.2 MHz
    let draw_time = gb.tick()
    if (draw_time) {
        console.log("Rendering screen")
        // draw_screen()
    }

    window.requestAnimationFrame(() => {
        mainloop(gb)
    })
}

run().catch(console.error)

// async function run() {
//     const WIDTH = 160
//     const HEIGHT = 144
//     // var scale = 5
//     var scale = 1

//     // Setup canvas
//     const canvas = document.getElementById("canvas")
//     canvas.width = WIDTH * scale
//     canvas.height = HEIGHT * scale
//     const ctx = canvas.getContext("2d")

//     // Draw canvas
//     function draw_screen() {
//         const gfx = new Uint8Array(exports.get_gfx())
//         const palette = new Uint8Array(exports.get_palette())

//         // Clear canvas
//         ctx.fillStyle = "blue"
//         ctx.fillRect(0, 0, WIDTH * scale, HEIGHT * scale)

//         for (var y = 0; y < HEIGHT; y++) {
//             for (var x = 0; x < WIDTH; x++) {
//                 var index = y * WIDTH + x
//                 var pixel = gfx[index]
//                 var color = COLORS[palette[pixel]]
//                 ctx.fillStyle = color
//                 ctx.fillRect(x * scale, y * scale, scale, scale)
//             }
//         }
//     }
