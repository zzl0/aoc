from utils import *

OPERATORS = {'+': operator.add, '*': operator.mul}


def eval_op(ops, args):
    op, b, a = ops.pop(), args.pop(), args.pop()
    args.append(OPERATORS[op](a, b))


def eval(exp, precedence):
    ops, args, i = [], [], 0
    while i < len(exp):
        c = exp[i]
        if c.isdigit():
            j = i + 1
            while j < len(exp) and exp[j].isdigit(): j += 1
            args.append(int(exp[i:j]))
            i = j - 1
        elif c in OPERATORS:
            if ops and ops[-1] != '(' and precedence[c] <= precedence[ops[-1]]:
                eval_op(ops, args)
            ops.append(c)
        elif c == '(':
            ops.append('(')
        else: # ')'
            while ops[-1] != '(': eval_op(ops, args)
            ops.pop() # pop '('
        i += 1
    while ops: eval_op(ops, args)
    return args[0]


def day18_1(exps):
    precedence = {'+': 0, '*': 0}
    return sum(eval(e, precedence) for e in exps)


def day18_2(exps):
    precedence = {'+': 1, '*': 0}
    return sum(eval(e, precedence) for e in exps)


if __name__ == "__main__":
    exps = data(18, lambda x: x.replace(' ', ''))
    print(f'day18_1: {day18_1(exps)}')
    print(f'day18_2: {day18_2(exps)}')
    # day18_1: 30753705453324
    # day18_2: 244817530095503
    # python3 day18.py  0.05s user 0.01s system 93% cpu 0.068 total
