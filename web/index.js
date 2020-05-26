import init, * as wasm from "./agba_wasm.js"

document.addEventListener("keydown", function(e) {
    // Space bar
    if (e.keyCode == '32') {
        // Currently does nothing
    }
})

async function run() {
    await init()
    let gb = new wasm.GB();

    await load_js(gb, "opus5.gb").then(() => {
        let title = gb.get_title()
        document.title = title
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
    while (true) {
        let draw_time = gb.tick()
        if (draw_time) {
            gb.draw_screen()
            window.requestAnimationFrame(() => {
                mainloop(gb)
            })

            break
        }
    }
}

run().catch(console.error)
