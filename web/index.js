import init, * as wasm from "./agba_wasm.js"

async function run() {
    await init()
    let gb = new wasm.GB();
    gb.init_cpu()

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
    // TODO: Need to run at 4.2 MHz
    gb.run()

    window.requestAnimationFrame(() => {
        mainloop(gb)
    })
}

run().catch(console.error)
