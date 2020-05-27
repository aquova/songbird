# agba

My Game Boy emulator, written in Rust

You can download the most recent build from [here](https://cirrus-ci.com/github/aquova/agba/master)

## CI Builds

- ![Build status](https://api.cirrus-ci.com/github/aquova/agba.svg?task=osx)
- ![Build status](https://api.cirrus-ci.com/github/aquova/agba.svg?task=linux)
- ![Build status](https://api.cirrus-ci.com/github/aquova/agba.svg?task=wasm)

## TODO

- [ ] CPU
    - [x] Opcode functionality
    - [ ] Interrupts
    - [ ] Keyboard input
    - [ ] Pass Blargg tests
- [ ] Memory Bank Controllers
    - [x] None
    - [ ] MBC1
    - [ ] MBC2
    - [ ] MBC3
- [ ] PPU
    - [ ] All PPU layers rendered
    - [ ] 8x16 sprites
- [ ] Audio
- [ ] Platforms
    - [ ] DMG support
    - [ ] CGB support
    - [ ] SGB support
- [ ] CI
    - [x] Linux Builds
    - [x] MacOS Builds
    - [ ] Windows Builds
    - [x] WebAssembly support
- [ ] Save support
- [ ] UI Menu
- [ ] Rebindable keys

## Resources

- [Blargg test roms](https://github.com/retrio/gb-test-roms)
- [Game Boy Opcode Table](http://pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
- [Another Opcode Table](https://izik1.github.io/gbops/)
- [Details about Opcode functionality](https://raw.githubusercontent.com/gb-archive/salvage/master/txt-files/gb-instructions.txt)
- [Reference emulator](https://github.com/blackxparade/Rust-Boy)
- [Info on Half Carry Flag](https://robdor.com/2016/08/10/gameboy-emulator-half-carry-flag/)
- [Good explaination of MBC](https://retrocomputing.stackexchange.com/questions/11732/how-does-the-gameboys-memory-bank-switching-work)
- [Documentation about making an emulator in JS](http://imrannazar.com/GameBoy-Emulation-in-JavaScript)
- [Test ROMs, other information](http://opusgames.com/games/GBDev/GBDev.html)
