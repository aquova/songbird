# sonGBird

Songbird Game Boy emulator.

You can download the most recent build from [here](https://cirrus-ci.com/github/aquova/songbird/master)

## CI Builds

- ![Build status](https://api.cirrus-ci.com/github/aquova/songbird.svg?task=win)
- ![Build status](https://api.cirrus-ci.com/github/aquova/songbird.svg?task=osx)
- ![Build status](https://api.cirrus-ci.com/github/aquova/songbird.svg?task=linux)
- ![Build status](https://api.cirrus-ci.com/github/aquova/songbird.svg?task=wasm)

## Resources

- [Blargg test roms](https://github.com/retrio/gb-test-roms)
- [Game Boy Opcode Table](http://pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
- [Another Opcode Table](https://izik1.github.io/gbops/)
- [Details about Opcode functionality](https://raw.githubusercontent.com/gb-archive/salvage/master/txt-files/gb-instructions.txt)
- [Info on Half Carry Flag](https://robdor.com/2016/08/10/gameboy-emulator-half-carry-flag/)
- [Good explaination of MBC](https://retrocomputing.stackexchange.com/questions/11732/how-does-the-gameboys-memory-bank-switching-work)
- [Documentation about making an emulator in JS](http://imrannazar.com/GameBoy-Emulation-in-JavaScript)
- [Test ROMs, other information](http://opusgames.com/games/GBDev/GBDev.html)

## TODO

- [ ] CPU
    - [x] Opcode functionality
    - [x] Interrupts
    - [x] Keyboard input
    - [ ] Pass Blargg tests
- [ ] Memory Bank Controllers
    - [x] None
    - [ ] MBC1
    - [ ] MBC2
    - [ ] MBC3
- [ ] PPU
    - [x] All PPU layers rendered
    - [ ] 8x16 sprites
- [ ] Audio
- [ ] Platforms
    - [ ] DMG support
    - [ ] CGB support
    - [ ] SGB support
- [x] CI
    - [x] Linux Builds
    - [x] MacOS Builds
    - [x] Windows Builds
    - [x] WebAssembly support
- [ ] Save support
- [ ] UI Menu
- [ ] Rebindable keys
