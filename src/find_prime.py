def is_prime(n: int) -> bool:
    """Primality test using 6k+-1 optimization."""
    if n <= 3:
        return n > 1
    if not n%2 or not n%3:
        return False
    i = 5
    stop = int(n**0.5)
    while i <= stop:
        if not n%i or not n%(i + 2):
            return False
        i += 6
    return True


count = 0
nprime = 0
while True:
    if is_prime(count) is True:
        nprime += 1
        # print(f"{count} is prime!")
    count += 1
    if nprime == 100000:
        break