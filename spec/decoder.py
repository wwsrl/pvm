# Globals
operation_mask   = 0x000F
load_type_mask   = 0x0030
load_length_mask = 0x00C0
source_mask      = 0x0F00
destination_mask = 0xF000

alu_opcode_map = [
    "sum",
    "subtract",
    "multiply",
    "divide",
    "and",
    "or",
    "nand",
    "nor",
    "xor",
    "bsl",
    "bsr",
    "rlr",
    "pop stack",
    "push stack",
    "sw interrupt",
    "nop",
]

registers_map = [
    "a",
    "x",
    "y",
    "b",
    "w",
    "z",
    "c",
    "d",
    "pc",
    "sb",
    "sp",
    "fl",
    "reserved",
    "reserved",
    "reserved",
    "none",
]

ld_types = [
    "register",
    "ram",
    "immediate",
    "none",
]

ld_sizes = [
    "1 byte",
    "2 byte",
    "4 byte",
    "8 byte",
]

def parse_instruction(data: int):
    if data == 0xFFFF:
        print("NOP")
        return
    if data == 0:
        print("Invalid instruction")
        return

    op = (operation_mask & data)
    ld_type = (load_type_mask & data) >> 4
    ld_len = (load_length_mask & data) >> 6
    src = (source_mask & data) >> 8
    dst = (destination_mask & data) >> 12

    if src == dst:
        print("Invalid instruction, source and destination are equal")
        return
    if op == 15 and ld_type == 3:
        print("Invalid instruction, no alu op and no load")
        return
    if op != 15 and ld_type == 3:
        print("Invalid instruction, alu op and load conflict")
        return

    op_name = alu_opcode_map[op]
    ld_type_name = ld_types[ld_type]
    ld_size_name = ld_sizes[ld_len]
    src_name = registers_map[src]
    dst_name = registers_map[dst]

    print(bin(dst), bin(src), bin(ld_len), bin(ld_type), bin(op))
    print(f"{op_name} {ld_type_name} {ld_size_name} wide with src: {src_name} into {dst_name}")



if __name__ == "__main__":
    while True:
        instruction = input("Please enter an intruction (two bytes long in HEX): ")
        data = 0
        try:
            data = int(instruction, base=16)
        except ValueError as e:
            print(f"Invalid value {e}")
            continue
        parse_instruction(data)

