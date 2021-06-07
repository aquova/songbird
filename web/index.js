import init, * as wasm from "./songbird_wasm.js"

const WIDTH = 160
const HEIGHT = 144

let scale = 1
let scale_elem = document.getElementById("scale")

let my_storage = window.localStorage
let anim_frame = 0

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
        // Stop previous game from rendering, if one exists
        if (anim_frame != 0) {
            window.cancelAnimationFrame(anim_frame)
        }

        let file = e.target.files[0]
        if (!file) {
            alert("Failed to read file")
            return
        }

        let fr = new FileReader()
        fr.onload = function (e) {
            let buffer = fr.result
            const rom = new Uint8Array(buffer)

            let force_dmg = document.getElementById("force_dmg").checked
            gb.reset()
            gb.load_rom(rom, force_dmg)
            load_save(gb)
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

            // Save game if contents have been modified
            if (gb.is_battery_dirty()) {
                save_game(gb)
            }

            anim_frame = window.requestAnimationFrame(() => {
                mainloop(gb)
            })

            break
        }
    }
}

/// Update Canvas
///
/// Resizes canvas based on user input
function update_canvas() {
    scale = Math.floor(parseInt(scale_elem.value))
    canvas.width = WIDTH * scale
    canvas.height = HEIGHT * scale
    // Set canvas to be white, so we can see it on page
    let ctx = canvas.getContext("2d")
    ctx.fillStyle = "#FFFFFF"
    ctx.fillRect(0, 0, scale * WIDTH, scale * HEIGHT)
}

/// Load save
///
/// Loads save data from local storage, sends it to emulator
function load_save(gb) {
    if (gb.has_battery()) {
        let title = gb.get_title()
        let save = my_storage.getItem(title)
        if (save != null) {
            console.log("Save data loaded")
            let u8 = from_base64(save)
            let data = new Uint8Array(u8)
            gb.load_save_data(data)
        } else {
            console.log("No save data found for " + title)
        }
    }
}

/// Save game
///
/// Gets external RAM data from emulator, saves it to local storage as Base64 encoding
function save_game(gb) {
    if (gb.has_battery()) {
        let ram_data = gb.get_save_data()
        let title = gb.get_title()

        let b64 = to_base64(ram_data)
        my_storage.setItem(title, b64)
    }
}

/// To Base64
///
/// Converts Uint8Array into a Base64-encoded string
function to_base64(u8) {
    return btoa(String.fromCharCode.apply(null, u8))
}

/// From Base64
///
/// Converts Base64-encoded string into Uint8Array
function from_base64(str) {
    return atob(str).split('').map(function(c) {
        return c.charCodeAt(0)
    })
}

run().catch(console.error)
