# PVM primary vm architecture
This is a simple vm to allow the execution of trusted but unsure quality wise software,
this means that this vm can protect against accidental behaviours but is not intended to sandbox untrusted code.

# 1 - PVM Basic facts
PVM is:
* Little endian;
* 64-bit wide pointer and long int, with 48 significant addressing bits;
* v1 and v2 can be accessed in 8, 4(upper and lower), 2 and single byte granularity
* v3 and v4 are only accessed as 64 bits wide

# 2 - Registers file
General purpose RW registers:
* a
* x
* y
* b
* w
* z
* v3
* v4

Special purpose RO registers (some of these can have some of their bits changed via instructions)

* pc (program counter)
* sb (stack base)
* sp (stack pointer)
* fl (flags register 4 bytes wide, lower 2 used, top 2 reserved)


## 2.1 - Flags bitfield:
The bits are given in the order 0 --> 15.

| Bit  | Meaning                                                     |
|------|-------------------------------------------------------------|
| 0    | overflow (ro)                                               |
| 1    | compare result (0 if true 1 if not) (ro)                    |
| 2,3  | ring level (ro(w) writeable by ring 0 to go back to ring 3) |
| 4    | exception (ro)                                              |
| 5    | gpf (ro)                                                    |
| 6,7  | reserved                                                    |
| 8,15 | Interrupt Vector                                            |


# 3 - ALU
ALU commands are always at least 2 byte wide, if it interacts with registers 2 extra bytes should be read,

> Note:
> 
> all 0 instruction is invalid, trying to execute it triggers a handleable exception,
> an instruction in the form: 0xFFFFFFFF is a NOP.

## 3.1 - ALU command structure
These are the bits composing a command and their meanings 0 --> 15 order:

| Bits | Reference                      |
|------|--------------------------------|
| 0,3  | ALU Operation, see table 3.1.1 |
| 4,5  | Load type, see table 3.1.2     |
| 6,7  | Load Size, see table 3.1.3     |
| 8,B  | Source, see table 3.1.4        |
| C,F  | Destination, , see table 3.1.4 |


| A | B | C | D | Operation    |   |
|---|---|---|---|--------------|---|
| 0 | 0 | 0 | 0 | Sum          | 0 |
| 1 | 0 | 0 | 0 | Subtract     | 1 |
| 0 | 1 | 0 | 0 | Multiply     | 2 |
| 1 | 1 | 0 | 0 | Divide       | 3 |
| 0 | 0 | 1 | 0 | And          | 4 |
| 1 | 0 | 1 | 0 | Or           | 5 |
| 0 | 1 | 1 | 0 | Nand         | 6 |
| 1 | 1 | 1 | 0 | Nor          | 7 |
| 0 | 0 | 0 | 1 | Xor          | 8 |
| 1 | 0 | 0 | 1 | Bsl          | 9 |
| 0 | 1 | 0 | 1 | Bsr          | A |
| 1 | 1 | 0 | 1 | Rlr          | B |
| 0 | 0 | 1 | 1 | Pop stack    | C |
| 1 | 0 | 1 | 1 | Push stack   | D |
| 0 | 1 | 1 | 1 | Sw Interrupt | E |
| 1 | 1 | 1 | 1 | NOP          | F |
Table 3.1.1 - ALU operations

| A | B | Load type |   |
|---|---|-----------|---|
| 0 | 0 | Register  | 0 |
| 1 | 0 | Ram       | 1 |
| 0 | 1 | Immediate | 2 |
| 1 | 1 | None      | 3 |
Table 3.1.2 - Load operation types

| A | B | Load size   |   |
|---|---|-------------|---|
| 0 | 0 | Byte        | 0 |
| 1 | 0 | Word        | 1 |
| 0 | 1 | Double word | 2 |
| 1 | 1 | Quad word   | 3 |
Table 3.1.3 - Load operation sizes

| A | B | C | D | Register | Size (bytes) | Notes      | Access |
|---|---|---|---|----------|--------------|------------|--------|
| 0 | 0 | 0 | 0 | A        | 8            | Gp         | RW     |
| 1 | 0 | 0 | 0 | X        | 4            | Lower of A | RW     |
| 0 | 1 | 0 | 0 | Y        | 4            | Upper of A | RW     |
| 1 | 1 | 0 | 0 | B        | 8            | Gp         | RW     |
| 0 | 0 | 1 | 0 | W        | 4            | Lower of B | RW     |
| 1 | 0 | 1 | 0 | Z        | 4            | Upper of B | RW     |
| 0 | 1 | 1 | 0 | C        | 8            | Gp         | RW     |
| 1 | 1 | 1 | 0 | D        | 8            | Gp         | RW     |
| 0 | 0 | 0 | 1 | PC       |              | Prog Ctr   | R      |
| 1 | 0 | 0 | 1 | SB       |              | Stack Base | R      |
| 0 | 1 | 0 | 1 | SP       |              | Stack Ptr  | R      |
| 1 | 1 | 0 | 1 | FL       |              | Flags      | R      |
| 0 | 0 | 1 | 1 | Reserved |              | ####       | N/A    |
| 1 | 0 | 1 | 1 | Reserved |              | ####       | N/A    |
| 0 | 1 | 1 | 1 | Reserved |              | ####       | N/A    |
| 1 | 1 | 1 | 1 | Reserved |              | ####       | N/A    |
Table 3.1.4 - Registers and their access patterns


## 3.2 - Working with instructions
The structures in the instructions for this vm can be read with these masks
```c
// Operation
uint16_t operation   = 0x000F & instruction;
// Load type
uint16_t load_type   = (0x0030 & instruction) <<  4;
// Load lenght
uint16_t load_length = (0x00C0 & instruction) <<  6;
// Source
uint16_t source      = (0x0F00 & instruction) <<  8;
// Destination
uint16_t destination = (0xF000 & instruction) << 12;
```

Togheter with this spec there is a file called `decoder.py` that when run allows you to test decoding some instructions
