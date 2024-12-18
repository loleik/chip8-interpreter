# CHIP-8 Interpreter
This is my CHIP-8 interpreter written in rust. Just intended as a small project to learn some emulation dev skills. The following resources have been helpful, and are what the order of developement have been centered around:

+ [Timendus' CHIP-8 Test Suite](https://github.com/Timendus/chip8-test-suite)
+ [Cowgod's CHIP-8 Technical Reference v1.0](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)

My implementation currently passes `1-chip8-logo.ch8`, `2-ibm-logo.ch8`, `3-corax+.ch8`, `4-flags.ch8`, and `6-keypad.ch8` from Timendus' repo.

`7-beep.ch8`seems to work mostly. It prints beep and flashes the speaker icon as expected (I haven't implemented actual sound yet).

I'm currently working on getting `5-quirks.ch8`working but I'm struggling a bit, even outside of the quirks themselves. There's strange behaviour with the loading bar for example, unless that's how it's intended to look.

With regards to games, I have tested:
+ `petdog.ch8`by SystemLogoff, which works as expected.
+ `caveexplorer.ch8`by JohnEarnest, which seems to have some issues with my interpreter.
+ `piper.ch8`by Aeris, JordanMecom and LillianWang, which panics due to `0xF029`not being implemented yet.
+ `1dcell.ch8`by SharpenedSpoon also panics due to `0xF029`.
These were all found at [John Earnest's CHIP-8 Archive](https://johnearnest.github.io/chip8Archive/?sort=platform).

## Planned:
+ Other opcodes, obviously.
+ Audio.
+ CHIP-8 font set.
+ CLAP interface for passing ROMs.
