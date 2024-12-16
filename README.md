# CHIP-8 Interpreter
This is my CHIP-8 interpreter written in rust. Just intended as a small project to learn some emulation dev skills. The following resources have been helpful, and are what the order of developement have been centered around:

+ [Timendus' CHIP-8 Test Suite](https://github.com/Timendus/chip8-test-suite)
+ [Cowgod's CHIP-8 Technical Reference v1.0](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)

My implementation currently passes `1-chip8-logo.ch8`, `2-ibm-logo.ch8`, `3-corax+.ch8`, and `4-flags.ch8` from Timendus' repo.

I'm currently working on getting `5-quirks.ch8`working but I'm struggling. I think my lack of understanding of timing and sdl2 have all came togeter to cause issues here.

## Planned:
+ Other opcodes, obviously.
+ Input and audio.
+ CHIP-8 font set.
+ CLAP interface for passing ROMs.
