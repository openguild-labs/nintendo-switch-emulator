## Resources

- Nintendo Switch system software: https://en.wikipedia.org/wiki/Nintendo_Switch_system_software

Notable findings include that the Switch operating system is codenamed Horizon, that it is an evolution of the Nintendo 3DS system software, and that it implements a proprietary microkernel architecture.

## Instruction Set Architecture

- Basic ofs SIMD Programming: http://ftp.cvut.cz/kernel/people/geoff/cell/ps3-linux-docs/CellProgrammingTutorial/BasicsOfSIMDProgramming.html

## Computer & Operating Systems

- Little Computer 3: https://en.wikipedia.org/wiki/Little_Computer_3
- LC3b: https://users.ece.utexas.edu/~patt/07s.360N/handouts/360n.appC.pdf
- eduOS-rs: https://github.com/RWTH-OS/eduOS-rs

Educational purpose operating system written in Rust

## Virtualization & Emulators

- xhypervisor: https://github.com/RWTH-OS/xhypervisor
- hypervisor-rs: https://github.com/saurvs/hypervisor-rs

Apple Hypervisor written in Rust

- Ryujinx (MacOS / Linux / Windows): https://github.com/Ryujinx/Ryujinx

This breaks down the overall architecture of the NS, a good resource

- Yuzu (Linux / Windows): https://github.com/yuzu-emu/yuzu

## Applications

- Chip8 Emulator (in Javascript): https://github.com/taniarascia/chip8
- Chip8 Emulator (in Rust, WASM): https://github.dev/ColinEberhardt/wasm-rust-chip8/tree/master/src

## Nintendo Switch

- LibHac: https://github.com/Thealexbarney/LibHac

A library that reimplements parts of the Nintendo Switch OS

- Awesome Nintendo Switch: https://reswitched.github.io/awesome/
- Deep Sea Knowledge - Switch Game Modding: https://wiki.oatmealdome.me/Main_Page

## Tools

- hactool: https://github.com/SciresM/hactool

hactool is a cli, that allows you to view contents of NSP files, extract invidual files, or decrypt/encrypt files.

- radare2: https://github.com/radareorg/radare2

r2 is a complete rewrite of radare. It provides a set of libraries, tools and plugins to ease reverse engineering tasks. Distributed mostly under LGPLv3, each plugin can have different licenses (see r2 -L, rasm2 -L, ...).

- radare.book: https://book.rada.re/

Official guide book for Radare

- iaito: https://github.com/radareorg/iaito

Qt frontend for radare

- valgrind: https://valgrind.org/

Valgrind is an instrumentation framework for building dynamic analysis tools. There are Valgrind tools that can automatically detect many memory management and threading bugs, and profile your programs in detail. You can also use Valgrind to build new tools. (TLDR: Check memory allocation and debug memory leak in the low level environment)

## Articles

- Law of abstraction: https://www.joelonsoftware.com/2002/11/11/the-law-of-leaky-abstractions/

## Rust

- Effective Rust resource: https://www.lurklurk.org/effective-rust/

## Reference

- Chip8 technical reference: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.1
- GameBoy Dev technical documentation: https://gbdev.io/pandocs
- ARM vs x86: https://www.androidauthority.com/arm-vs-x86-key-differences-explained-568718/
- ARM technical reference: https://armv8-ref.codingbelief.com/en/chapter_d4/d4_the_aarch64_virtual_memory_system_archi.html