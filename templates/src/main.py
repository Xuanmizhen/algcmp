# from array import array
from contextlib import suppress
# from fractions import Fraction
from functools import cache
from itertools import (chain, islice, product, starmap)
from math import isqrt
import operator
import sys


input = sys.stdin.readline
write = sys.stdout.write

if sys.version_info >= (3, 12):
    from itertools import batched
    from math import sumprod

else:
    def batched(iterable, n, *, strict=False):
        # batched('ABCDEFG', 3) → ABC DEF G
        if n < 1:
            raise ValueError('n must be at least one')
        iterator = iter(iterable)
        while batch := tuple(islice(iterator, n)):
            if strict and len(batch) != n:
                raise ValueError('batched(): incomplete batch')
            yield batch

    def sumprod(p, q):
        "Note that in this version, if the inputs do not have the same length, the behavior is not specified. Also of note is that the precision for float and mixed int/float inputs is not guaranteed."
        return sum(map(operator.mul, p, q))

    if sys.version_info >= (3, 10):
        from itertools import pairwise
    else:
        def pairwise(iterable):
            # pairwise('ABCDEFG') → AB BC CD DE EF FG

            iterator = iter(iterable)
            a = next(iterator, None)

            for b in iterator:
                yield a, b
                a = b


# ==== Data pipelines ====

def iter_index(iterable, value, start=0, stop=None):
    "Return indices where a value occurs in a sequence or iterable."
    # iter_index('AABCADEAF', 'A') → 0 1 4 7
    seq_index = getattr(iterable, 'index', None)
    if seq_index is None:
        iterator = islice(iterable, start, stop)
        for i, element in enumerate(iterator, start):
            if element is value or element == value:
                yield i
    else:
        stop = len(iterable) if stop is None else stop
        i = start
        with suppress(ValueError):
            while True:
                yield (i := seq_index(value, i, stop))
                i += 1


# ==== Matrix operations ====

def reshape(matrix, columns):
    "Reshape a 2-D matrix to have a given number of columns."
    # reshape([(0, 1), (2, 3), (4, 5)], 3) →  (0, 1, 2) (3, 4, 5)
    return batched(chain.from_iterable(matrix), columns, strict=True)

def transpose(matrix):
    "Swap the rows and columns of a 2-D matrix."
    # transpose([(1, 2, 3), (11, 22, 33)]) → (1, 11) (2, 22) (3, 33)
    return zip(*matrix, strict=True)

def matmul(m1, m2):
    "Multiply two matrices."
    # matmul([(7, 5), (3, 5)], [(2, 5), (7, 9)]) → (49, 80) (41, 60)
    n = len(m2[0])
    return batched(starmap(sumprod, product(m1, transpose(m2))), n)


# ==== Number theory ====

def sieve(n):
    "Primes less than n."
    # sieve(30) → 2 3 5 7 11 13 17 19 23 29
    if n > 2:
        yield 2
    data = bytearray((0, 1)) * (n // 2)
    for p in iter_index(data, 1, start=3, stop=isqrt(n) + 1):
        data[p*p : n : p+p] = bytes(len(range(p*p, n, p+p)))
    yield from iter_index(data, 1, start=3)

def factor(n):
    "Prime factors of n."
    # factor(99) → 3 3 11
    # factor(1_000_000_000_000_007) → 47 59 360620266859
    # factor(1_000_000_000_000_403) → 1000000000000403
    for prime in sieve(isqrt(n) + 1):
        while not n % prime:
            yield prime
            n //= prime
            if n == 1:
                return
    if n > 1:
        yield n

def is_prime(n):
    "Return True if n is prime."
    # is_prime(1_000_000_000_000_403) → True
    return n > 1 and next(factor(n)) == n

def totient(n):
    "Count of natural numbers up to n that are coprime to n."
    # https://mathworld.wolfram.com/TotientFunction.html
    # totient(12) → 4 because len([1, 5, 7, 11]) == 4
    for prime in set(factor(n)):
        n -= n // prime
    return n


# For more self-made iterator tools, see https://docs.python.org/3/library/itertools.html#itertools-recipes


@cache
def factorial_mod(n: int, mod: int):
    if n < 0 or mod <= 0:
        raise ValueError('factorial_mod() only defined for non-negative n and positive mod')
    return n * factorial_mod(n - 1, mod) % mod if n else 1 % mod

def cache_factorial_mod(stop: int, mod: int):
    for i in range(stop):
        factorial_mod(i, mod)

def perm_mod(n: int, k: int, mod: int):
    if n >= mod:
        raise NotImplementedError('perm_mod() not implemented for n >= mod')
    return factorial_mod(n, mod) * pow(factorial_mod(n - k, mod), -1, mod) % mod

def comb_mod(n: int, k: int, mod: int):
    return perm_mod(n, k, mod) * pow(factorial_mod(k, mod), -1, mod) % mod

def comb_mod2(n: int, k: int, mod: int):
    """
    卢卡斯定理版组合数，解决 n >= mod 的问题
    警告：以下代码由AI生成，未经测试和逻辑验证，可能存在错误，请谨慎使用
    """
    if k < 0 or k > n: return 0
    res = 1
    while n > 0 or k > 0:
        ni, ki = n % mod, k % mod
        if ki > ni: return 0
        # 直接复用缓存，不用操心性能
        res = res * factorial_mod(ni, mod) % mod
        res = res * pow(factorial_mod(ki, mod), -1, mod) % mod
        res = res * pow(factorial_mod(ni - ki, mod), -1, mod) % mod
        n //= mod; k //= mod
    return res


t = int(input())
for _ in range(t):
    pass
