# Solutions for Advent of code 2024

## How to use

Copy input for day into inputs/inputxx.txt
Implement solution in src/days/dayxx.rs

```bash
# test solution for a day
cargo test --lib day01
# run solution for a day
cargo run --bin day01
# run benchmark for a day
cargo bench day01
```

## Benchmarks

| day  |  part1 |  part2 |
| :----|-------:|-------:|
| day01|  85 µs |  106 µs |
| day02| 101 µs |  877 µs |
| day03| 483 µs |  424 µs |
| day04| 451 µs |  375 µs |
| day05| 999 µs | 3445 µs |


## Benchmark details

Macbook Pro 2019
2.8 Ghz Intel Core i7

```
day01_part1             time:   [83.500 µs 84.844 µs 87.276 µs]
                        change: [-24.038% -23.400% -22.658%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe

day01_part2             time:   [105.99 µs 106.47 µs 107.00 µs]
                        change: [-14.530% -13.960% -13.374%] (p = 0.00 < 0.05)
                        Performance has improved.

day02_part1             time:   [100.66 µs 101.04 µs 101.44 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

day02_part2             time:   [873.28 µs 876.96 µs 881.02 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

day03_part1             time:   [480.87 µs 483.00 µs 485.27 µs]
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe

day03_part2             time:   [421.70 µs 423.70 µs 425.79 µs]

day04_part1             time:   [449.03 µs 450.68 µs 452.45 µs]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild

day04_part2             time:   [368.79 µs 374.60 µs 381.58 µs]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high severe

day05_part1             time:   [993.46 µs 998.82 µs 1.0045 ms]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  2 (2.00%) high severe

day05_part2             time:   [3.4312 ms 3.4449 ms 3.4595 ms]
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high mild

```