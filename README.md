# minigrep

Basic benchmarks (`cargo install hyperfine`):

`hyperfine --warmup 3 --prepare 'rm -f /tmp/grep.txt' 'grep to output.txt > /tmp/grep.txt' 'minigrep to output.txt' 'rg to output.txt' 'python3 minigrep.py 1 to output.txt'`

```
Summary:
  'rg to output.txt' ran
    1.47 ± 0.15 times faster than 'grep to output.txt > /tmp/grep.txt'
    3.32 ± 0.33 times faster than 'minigrep to output.txt'
    9.87 ± 0.88 times faster than 'python3 minigrep.py 1 to output.txt'
```

`hyperfine --warmup 3 --prepare 'rm -f /tmp/grep.txt' 'grep to output.txt > /tmp/grep.txt' 'minigrep to output.txt' 'python3 minigrep.py 1 to output.txt'`

```

Benchmark #1: grep to output.txt > /tmp/grep.txt
  Time (mean ± σ):      10.1 ms ±   0.6 ms    [User: 8.4 ms, System: 1.7 ms]
  Range (min … max):     9.8 ms …  16.1 ms    245 runs

  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet PC without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.

Benchmark #2: minigrep to output.txt
  Time (mean ± σ):      22.9 ms ±   1.1 ms    [User: 16.8 ms, System: 6.1 ms]
  Range (min … max):    21.9 ms …  28.9 ms    109 runs

  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet PC without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.

Benchmark #3: python3 minigrep.py 1 to output.txt
  Time (mean ± σ):      67.9 ms ±   2.4 ms    [User: 55.9 ms, System: 11.9 ms]
  Range (min … max):    66.0 ms …  77.4 ms    43 runs

  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet PC without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.

Summary
  'grep to output.txt > /tmp/grep.txt' ran
    2.26 ± 0.17 times faster than 'minigrep to output.txt'
    6.69 ± 0.46 times faster than 'python3 minigrep.py 1 to output.txt'
```
