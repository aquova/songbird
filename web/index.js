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

    document.addEventListener("keydown", function(e) {
        gb.handle_key(e, true)
    })

    document.addEventListener("keyup", function(e) {
        gb.handle_key(e, false)
    })

    document.getElementById('fileinput').addEventListener("change", async function (e) {
        let filename = e.target.files[0].name
        // Note: Doesn't work if ROM not in same directory
        await load_js(gb, filename).then(() => {
            let title = gb.get_title()
            document.title = title
            mainloop(gb)
        })
    }, false)
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
