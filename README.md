# Gameboy Emulator
[![Build Status](https://travis-ci.org/Wilfred/gameboy_emulator.svg?branch=master)](https://travis-ci.org/Wilfred/gameboy_emulator)

A basic gameboy emulator in Rust. Currently, it's just a disassembler.

References:

* [GameBoy Emulation in JavaScript](http://imrannazar.com/GameBoy-Emulation-in-JavaScript)
* [Opcode map](http://imrannazar.com/Gameboy-Z80-Opcode-Map)
* [Programming the Z80](http://www.z80.info/zaks.html)
* [Gameboy Development](https://slashbinbash.wordpress.com/2013/09/10/gameboy-development/)
* [Everything You Always Wanted To Know About GAMEBOY but were afraid to ask](http://www.opusgames.com/games/GBDev/zips/Gbspec.txt)
* [ASMSchool](http://gameboy.mongenel.com/asmschool.html)
* [Z80 tools](http://www.z80.info/z80sdt.htm) (disassemblers etc)

License: GPL v2 or later.

## Building

```bash
$ cargo build
```

Usage as a disassembler:

```bash
$ cargo run -- --dis /path/to/foo.gb
```

Assessing progress:

```bash
$ cargo run -- --implemented
```
