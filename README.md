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
| day01| 85 µs | 106 µs |


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

```