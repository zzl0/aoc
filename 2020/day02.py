from utils import *


@dataclass
class Password:
    first: int
    second: int
    char: str
    password: str

    @classmethod
    def of(cls, s):
        # '1-3 b: cdefg' -> ('1', '3', 'b', 'cdefg')
        l, h, c, password = re.findall(r'[^-:\s]+', s)
        return cls(int(l), int(h), c, password)

    def isValid1(self) -> bool:
        return self.first <= self.password.count(self.char) <= self.second

    def isValid2(self) -> bool:
        return sum(self.password[i - 1] == self.char for i in (self.first, self.second)) == 1


def day2_1(passwords: List[Password]) -> int:
    return sum(p.isValid1() for p in passwords)


def day2_2(passwords: List[Password]) -> int:
    return sum(p.isValid2() for p in passwords)


if __name__ == "__main__":
    passwords = data(2, Password.of)
    print(f'day2_1: {day2_1(passwords)}')
    print(f'day2_2: {day2_2(passwords)}')

    # day2_1: 418
    # day2_2: 616
