#!/usr/bin/env python
# Author: https://missing.csail.mit.edu/2020/debugging-profiling/

import sys, re

def grep(pattern, file):
    pattern = re.compile(pattern)
    with open(file, 'r') as f:
        print(file)
        for i, line in enumerate(f.readlines()):
            match = pattern.search(line)
            if match is not None:
                print("{}: {}".format(i, line), end="")

if __name__ == '__main__':
    times = int(sys.argv[1])
    pattern = sys.argv[2]
    for i in range(times):
        for file in sys.argv[3:]:
            grep(pattern, file)
