#!/usr/bin/env python3

import math
from typing import List

def primes_up_to(n: int) -> List[int]:
    """Return a list of prime numbers up to n (inclusive)."""
    if n < 2:
        return []
    sieve = [True] * (n + 1)
    sieve[0] = sieve[1] = False
    for p in range(2, int(math.isqrt(n)) + 1):
        if sieve[p]:
            for multiple in range(p * p, n + 1, p):
                sieve[multiple] = False
    return [i for i, is_prime in enumerate(sieve) if is_prime]

def main() -> None:
    print("Prime numbers up to 50:")
    print(primes_up_to(50))

if __name__ == "__main__":
    main()
