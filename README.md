sonGBird
========

[![Build status](https://api.cirrus-ci.com/github/aquova/songbird.svg?task=win)](https://cirrus-ci.com/github/aquova/songbird/master)
[![Build status](https://api.cirrus-ci.com/github/aquova/songbird.svg?task=osx)](https://cirrus-ci.com/github/aquova/songbird/master)
[![Build status](https://api.cirrus-ci.com/github/aquova/songbird.svg?task=linux)](https://cirrus-ci.com/github/aquova/songbird/master)
[![Build status](https://api.cirrus-ci.com/github/aquova/songbird.svg?task=wasm)](https://cirrus-ci.com/github/aquova/songbird/master)

A work in progress Game Boy emulator written in Rust for Windows/Mac/Linux and browsers via WebAssembly.

Latest builds can be downloaded [here](https://cirrus-ci.com/github/aquova/songbird/master)

Run the desktop version via:

```
$ cd gui && cargo run --release "path/to/game.gb"
```

## Resources

- [Blargg test roms](https://github.com/retrio/gb-test-roms)
- [RGBDS GBz80 Man Pages](https://rednex.github.io/rgbds/gbz80.7.html)
- [Game Boy Opcode Table](http://pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
- [Another Opcode Table](https://izik1.github.io/gbops/)
- [Details about Opcode functionality](https://raw.githubusercontent.com/gb-archive/salvage/master/txt-files/gb-instructions.txt)
- [Info on Half Carry Flag](https://robdor.com/2016/08/10/gameboy-emulator-half-carry-flag/)
- [Good explaination of MBC](https://retrocomputing.stackexchange.com/questions/11732/how-does-the-gameboys-memory-bank-switching-work)
- [Documentation about making an emulator in JS](http://imrannazar.com/GameBoy-Emulation-in-JavaScript)
- [Test ROMs, other information](http://opusgames.com/games/GBDev/GBDev.html)
