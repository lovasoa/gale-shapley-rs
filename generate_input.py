"""
Usage: python3 generate_input.py 100 > input.txt
./galeshapley < input.txt
"""
import sys

N=int(sys.argv[1])
for i in range(2*N):
    print(i%N, end=': ')
    for j in range(N):
        print(j, end=' ')
    print()