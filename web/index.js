import init, * as wasm from "./agba_wasm.js"

const WIDTH = 160
const HEIGHT = 144

let scale = 1
let scale_elem = document.getElementById("scale")

let canvas = document.getElementById("canvas")
update_canvas()

document.addEventListener("change", e => {
    update_canvas()
})

async function run() {
    await init()
    let gb = new wasm.GB();

    await load_js(gb, "opus5.gb").then(() => {
        let title = gb.get_title()
        document.title = title
        mainloop(gb)
    })

    document.addEventListener("keydown", function(e) {
        gb.handle_key(e, true)
    })

    document.addEventListener("keyup", function(e) {
        gb.handle_key(e, false)
    })
}

async function load_js(gb, game) {
    let res = await fetch(game)
    let buffer = await res.arrayBuffer()
    const rom = new DataView(buffer, 0, buffer.byteLength)
    gb.load_rom(rom)
}

function mainloop(gb) {
    while (true) {
        let draw_time = gb.tick()
        if (draw_time) {
            gb.draw_screen()
            // Rescale image if needed
            if (scale > 1) {
                let ctx = canvas.getContext('2d')
                ctx.imageSmoothingEnabled = false
                ctx.drawImage(canvas, 0, 0, WIDTH, HEIGHT, 0, 0, canvas.width, canvas.height)
            }

            window.requestAnimationFrame(() => {
                mainloop(gb)
            })

            break
        }
    }
}

function update_canvas() {
    scale = Math.floor(parseInt(scale_elem.value))
    canvas.width = WIDTH * scale
    canvas.height = HEIGHT * scale
}

run().catch(console.error)
