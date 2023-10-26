# armv8
> The Arm architecture instructions can be describes as SEE.(Simple Sequential Execution)<br />
> This mean thats the processor behaves as if the processor fetched, decoded and executed one instruction at  <br />
> a time.

## Registers

Arm architecture exposes **31** general purpose registers.<br />
Each register can be used as a **64-bit** X register(X0..X30) or as **32-bit** register.(W0..W30)<br />
> Register is a small, fast storage location in the CPU used to hold data for operations.

Using **X** registers will result in 64-bit calculations.<br />
```
ADD X0, X1, X2
```
Using **W** registers will result in 32-bit calculations.
```
ADD WO, W1, W2
```

*WARNING*<br />
When a **W** register is written as seen in the example above, the top 32-bits left of the 64-bits register are <br />
zeroed.

### Registers for floating point and vector
There is a separatere set of **32** registers used for *floating point* and *vector operations*.<br />
These registers are **128-bit**.<br />
Here is the list of this registers:
- Bx 8-bit
- Hx 16-bit
- Sx 32-bit
- Dx 64-bit
- Qx 128-bit

### Zeroed Registers 
The zero registers, **ZXR**, and **WZR**, always read as 0 and ignore writes.

### Link Register
**X30** is used as the Link Register.<br />

A link register is used to store the return address when a subroutine or function is called.<br />
Main purpose is to save memory cost by avoiding using **SP**(Stack Pointer).

E.g<br />
1. Function or subroutine is called, the addr of the instruction immediatly following the call is saved in the link<br />register.
1. Function execute
1. Once the function finishes, instead of popping an address off the stack, the return address is taken directly<br />
from the link register, and execution resumes at that address.

*WHY IT'S LESS EXPENSIVE TO USE LR ?*<br />
- **Single Register Write :** When a subroutine is called, the return address can be stored directly in the<br />
Link Register(LR) with a single register write.<br />
Otherwise using the Stack to save the return address involves multiple steps...

## System registers
System registers are used to configure the processor and to control systems suchs as the MMU and execption handling<br />

*WARNING*<br />
System registers cannot be used directly by data processing or load/store instructions.<br />
The content of a system register need to be read into an **X** register, operated on, and then writting back to<br />
the system register.<br />

### Read & Write System Registers
Reads the system register into Xd :
```
MRS Xd, <system register>
```
<br />

Write Xn to the system register :
```
MRS <system register>, Xn
```

### System Registers Naming/ELs
System registers are specified by name, for example **SCTLR_EL1**.<br />
System register names end with **_ELx**.

The suffix **_ELx** determn the minimum privlige level required to access that register. <br />
The **x** in **_ELx**, is a placeholder for the actual privilege level number.


## MMU
> MMU stands for **Memory Management Unit**.<br />

MMU is a critical component in many computer systems, especially in those that run modern operating systems.

### MMU Purpose
Main purpose is to translate virtual addresses into physical addresses (actual locations in the computer's RAM or main memory).<br />
This translation allows for several important features in modern computing.

### Virtual Memory
**Virtual memory** allows a computer to use more memory than it physically has by using a portion of the hard
drive as if it were **RAM**.<br />
This can be done by swapping data between physical RAM and the hard drive as needed.

### Memory Protection
**MMU** provides memory protection.<br />
Each program running on the computer is given its own set of virtual addresses, isolated from other programs.<br />
This prevents one program from accidentally altering the memory contents of another program.

### Memory Isolation
With the **MMU**, the operating system can allocate specific specific regions of memory for specific tasks and<br />
ensure that process cannot interfere with each other.

### Page Tables
The **MMU** uses data structures called page tables to keep track of the relationship between virtual 
and physical addresses.<br /> 
OS is responsible for maintaining these tables.
