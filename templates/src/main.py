# from array import array
import itertools
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
        while batch := tuple(itertools.islice(iterator, n)):
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


t = int(input())
for _ in range(t):
    pass
