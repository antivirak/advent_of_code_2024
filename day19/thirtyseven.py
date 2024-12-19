"""
https://adventofcode.com/2024/day/19
"""

from functools import lru_cache


def load_input() -> tuple[frozenset[str], list[str]]:
    with open('input.txt', 'r') as f:
        whole_input = f.read()
    input_split = whole_input.split('\n\n')
    towels_orig = frozenset(
        input_split[0].split(', '),
    )

    return towels_orig, input_split[1].split('\n')[:-1]


@lru_cache()
def can_split(s: str, substrings: frozenset[str]) -> bool:
    if s == "":
        return True

    s_len = len(s)
    for idx in range(s_len + 1):
        if s[:idx] in substrings and can_split(s[idx:], substrings):
            return True

    return False


@lru_cache()
def count_split(s, substrings: frozenset[str]) -> int:
    if s == "":
        return 1

    s_len = len(s)
    res = 0
    for idx in range(s_len + 1):
        if s[:idx] in substrings:
            res += count_split(s[idx:], substrings)

    return res


def main1() -> int:  # 259 too low - my "bruteforce" based on reverse sorting the towels almost worked
    """part1"""
    # could use trie
    towels, patterns = load_input()

    impossible = 0
    for pattern in patterns:
        if not can_split(pattern, towels):
            impossible += 1

    return len(patterns) - impossible


def main2() -> int:
    towels, patterns = load_input()

    res = 0
    for pattern in patterns:
        res += count_split(pattern, towels)

    return res


if __name__ == '__main__':
    print(main1())  # 260
    print(main2())  # 639963796864990
