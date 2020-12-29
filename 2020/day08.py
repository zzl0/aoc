from utils import *


def parse_instructions(s):
    op, arg = s.split(' ')
    return op, int(arg)


def run(instructions, pc=0, acc=0):
    seen = set()
    while pc < len(instructions):
        if pc in seen:
            break
        seen.add(pc)
        op, arg = instructions[pc]
        if op == 'acc':
            acc += arg
        pc = next_pc(instructions, pc)
    return acc, pc


def next_pc(instructions, pc):
    op, arg = instructions[pc]
    return pc + (arg if op == 'jmp' else 1)


def day8_1(instructions):
    return run(instructions)[0]


def day8_2(instructions):
    loop_start = run(instructions)[1]
    pc = loop_start
    swap = {'jmp': 'nop', 'nop': 'jmp'}
    is_terminated = lambda pc: pc == len(instructions)
    while True:
        op, arg = instructions[pc]
        if op in swap:
            instructions[pc] = (swap[op], arg)
            if is_terminated(run(instructions, pc)[1]):
                return run(instructions)[0]
            instructions[pc] = (op, arg)
        pc = next_pc(instructions, pc)


if __name__ == "__main__":
    instructions = data(8, parse_instructions)
    print(f'day8_1: {day8_1(instructions)}')
    print(f'day8_2: {day8_2(instructions)}')
    # day8_1: 1675
    # day8_2: 1532
    # python3 day08.py  0.04s user 0.01s system 91% cpu 0.057 total
