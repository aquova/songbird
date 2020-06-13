import init, * as wasm from "./songbird_wasm.js"

const WIDTH = 160
const HEIGHT = 144

const MAX_ROM = 32 * 1024 // Currently only support up to 32 KiB ROMs

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

    document.getElementById('fileinput').addEventListener("change", function (e) {
        let file = e.target.files[0]
        if (!file) {
            alert("Failed to read file")
            return
        } else if (file.size > MAX_ROM) {
            alert("The emulator does not currently support ROMs of that size. Sorry!")
            return
        }

        let fr = new FileReader()
        fr.onload = function (e) {
            let buffer = fr.result
            const rom = new Uint8Array(buffer)

            gb.reset()
            gb.load_rom(rom)
            let title = gb.get_title()
            document.title = title

            mainloop(gb)
        }

        fr.readAsArrayBuffer(file)
    }, false)
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
