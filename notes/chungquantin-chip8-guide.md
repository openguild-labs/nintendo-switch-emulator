#Tech/Hardware/Computer

## Overview

<!-- Cowgod's whitepaper seems contain wrong information -->
<!-- I saw many warning about the Cowgod's whitepaper, here's one -->
<!-- https://discord.com/channels/273534239310479360/478739360494387223/1133872714453614722 -->
- Specification: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.5
- Full flow with a demo: https://www.taniarascia.com/writing-an-emulator-in-javascript-chip8/
- Youtube breakdown: https://www.youtube.com/watch?v=YtSgV3gY3fs
- Rust guide: https://blog.scottlogic.com/2017/12/13/chip8-emulator-webassembly-rust.html
## Implementation

```rust
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const START_ADDR: u16 = 0x200;
// Notice the `0x200` offset, which is due to the ROMs being loaded at a 512 byte offset.

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;
const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 
    0x20, 0x60, 0x20, 0x20, 0x70, 
    0xF0, 0x10, 0xF0, 0x80, 0xF0, 
    0xF0, 0x10, 0xF0, 0x10, 0xF0, 
    0x90, 0x90, 0xF0, 0x10, 0x10, 
    0xF0, 0x80, 0xF0, 0x10, 0xF0, 
    0xF0, 0x80, 0xF0, 0x90, 0xF0, 
    0xF0, 0x10, 0x20, 0x40, 0x40, 
    0xF0, 0x90, 0xF0, 0x90, 0xF0, 
    0xF0, 0x90, 0xF0, 0x10, 0xF0, 
    0xF0, 0x90, 0xF0, 0x90, 0x90, 
    0xE0, 0x90, 0xE0, 0x90, 0xE0, 
    0xF0, 0x80, 0x80, 0x80, 0xF0, 
    0xE0, 0x90, 0x90, 0x90, 0xE0, 
    0xF0, 0x80, 0xF0, 0x80, 0xF0, 
    0xF0, 0x80, 0xF0, 0x80, 0x80, 
];
```

To draw the screen with fontset, we use

```rust
new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
```

What does that mean? So the declared FONTSET has `80 cells or 5 columns 16 rows`.
=> Each cell is 8 bit => 8 bit = 1 bytes => Requires `FONTSET_SIZE = 80 bytes` for screen initialization

```rust
const STACK_SIZE: usize = 16;
```

Chip8 emulator has 16 stack slots => 1 slot = 4096 / 16 = `256 bytes`

Number of registers are `const NUM_REGS: usize = 16;`

```rust
fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
```

Method to push and pop from the stack, increase or decrease the stack pointer

```rust
 pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDR as usize;
        let end = (START_ADDR as usize) + data.len();
        self.ram[start..end].copy_from_slice(data);
    }
```

Similar to boot loader in the context of OS programming, load the assigned stack memory (in this case start at `0x200` with allocated memory => similar to `mbrk` or `mmap`)

```rust
fn fetch(&mut self) -> u16 {
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;
        let op = (higher_byte << 8) | lower_byte;
        self.pc += 2;
        op
    }
```

The opcode is the one in between of the higher byte and the lower byte

## CPU implementation

Instruction will be handled following Instruction cycle `Fetch -> Decode -> Execute`

```rust
fn read_word(memory: [u8; 4096], index: u16) -> u16 {
    (memory[index as usize] as u16) << 8
        | (memory[(index + 1) as usize] as u16)
}

impl Cpu {
    pub fn execute_cycle(&mut self) {
        let opcode: u16 = read_word(self.memory, self.pc);
        self.process_opcode(opcode);
    }

    fn process_opcode(&mut self, opcode: u16) {
      ...
    }
}
```

## Fetch ROM and load to the emulator

```javascript
fetch(`roms/${rom}`)
  .then(i => i.arrayBuffer())
  .then(buffer => {
    const rom = new DataView(buffer, 0, buffer.byteLength);
    exports.reset();
    for (i = 0; i < rom.byteLength; i++) {
      programMemory[0x200 + i] = rom.getUint8(i);
    }
  });
```
