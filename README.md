# CHIP-8 Interpreter
This is my CHIP-8 interpreter written in rust. Just intended as a small project to learn some emulation dev skills. The following resources have been helpful, and are what the order of developement have been centered around:

+ [Timendus' CHIP-8 Test Suite](https://github.com/Timendus/chip8-test-suite)
+ [Cowgod's CHIP-8 Technical Reference v1.0](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)

My implementation currently passes `1-chip8-logo.ch8`, `2-ibm-logo.ch8`, `3-corax+.ch8`, `4-flags.ch8`, and `6-keypad.ch8` from Timendus' repo.

`7-beep.ch8`seems to work mostly. It prints beep and flashes the speaker icon as expected (I haven't implemented actual sound yet).

I'm currently working on getting `5-quirks.ch8`working. I am slightly confused by the `Disp. Clear` and `Clipping` quirks.

With regards to games, I have tested:
+ `petdog.ch8`by SystemLogoff, which works as expected.
+ `caveexplorer.ch8`by JohnEarnest, which seems to have some issues with my interpreter.
+ `piper.ch8`by Aeris, JordanMecom and LillianWang and `caveexplorer.ch8`by JohnEarnest actually seem to function properly when a clock speed is set around 1000Hz, but score displaying in the bottom of piper particularly doesn't work.
+ `1dcell.ch8`by SharpenedSpoon works when the speed is set significatnly higher.
+ `br8kout.ch8`by SharpenedSpoon works as expected.

These were all found at [John Earnest's CHIP-8 Archive](https://johnearnest.github.io/chip8Archive/?sort=platform).

Overall, it seems to run slower than other interpreters I've tried, and there is an issue where any score that should be displayed by a game seems to be stuck at `012`which is strange.

## Planned:
+ Audio.
+ CLAP interface for passing ROMs.
