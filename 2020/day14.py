from utils import *

MASK, MEM = 1, 2


def parse_instruction(s):
    "Return (op, args)"
    if s.startswith('mask'):
        return MASK, (s.split('= ')[1],)
    else: # mem
        return MEM, tuple(map(int, re.findall(r'[0-9]+', s)))


def mask_num(mask, num):
    bin_str = f'{num:036b}'
    return int(''.join(m if m != 'X' else b for b, m in zip(bin_str, mask)), 2)


def mask_addr(mask, addr):
    def dfs(addr_str):
        if addr_str:
            first, rest = addr_str[0], addr_str[1:]
            rest_rs = dfs(rest)
            options = (first,) if first != 'X' else ('0', '1')
            return [a + b for a in options for b in rest_rs]
        return ['']

    bin_str = f'{addr:036b}'
    addr_str = [m if m != '0' else b for b, m in zip(bin_str, mask)]
    return [int(n, 2) for n in dfs(addr_str)]


def day14_1(instructions):
    mask, mem = None, {}
    for op, args in instructions:
        if op == MASK:
            mask = args[0]
        else: # MEM
            addr, num = args
            mem[addr] = mask_num(mask, num)
    return sum(mem.values())


def day14_2(instructions):
    mask, mem = None, {}
    for op, args in instructions:
        if op == MASK:
            mask = args[0]
        else: # MEM
            addr, num = args
            for _addr in mask_addr(mask, addr):
                mem[_addr] = num
    return sum(mem.values())


if __name__ == "__main__":
    instructions = data(14, parse_instruction)
    print(f'day14_1: {day14_1(instructions)}')
    print(f'day14_2: {day14_2(instructions)}')
    # day14_1: 6317049172545
    # day14_2: 3434009980379
    # python3 day14.py  0.15s user 0.01s system 96% cpu 0.171 total
