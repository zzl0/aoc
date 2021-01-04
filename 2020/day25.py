from utils import *


def day25_1(public_keys):
    i, val, subject_num = 0, 1, 7
    while True:
        i += 1
        val = val * subject_num % 20201227
        if val in public_keys:
            break

    subject_num = public_keys[1] if public_keys[0] == val else public_keys[0]
    val = 1
    for _ in range(i):
        val = val * subject_num % 20201227
    return val


if __name__ == "__main__":
    public_keys = data(25, int)
    print(f'day25_1: {day25_1(public_keys)}')
