def sieve_of_eratosthenes(primes: list):
    primes[0] = False
    primes[1] = False
    for i in range(2, len(primes)):
        if i*i >= len(primes):
            break
        if primes[i]:
            for j in range(i*i, len(primes), i):
                primes[j] = False
    for i in range(len(primes)):
        print(f'{i}: {primes[i]}')
primes = []
number = 20
for i in range(0, number+1):
    primes.append(True)
sieve_of_eratosthenes(primes)
