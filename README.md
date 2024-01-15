# PoC for Live Collaboration via Operation Transformation in Rust
## About
Реализация алгоритма OT для синхронизации контента с хранением данных в памяти. 

## Benchmarks
- **Cpu**: AMD Ryzen 7 5800H
- **Mem**: 16 Gb 3200 MHz
- **os**: windows

Результаты benchmark'ов для `Insert.Apply` 

| Benchmark | Среднее время операции **Rust**|
| ---       | ---                                |
| BenchmarkInsert_Apply/Insert(2,_x)_to_short_content-8         | 80.800 ns/iter |
| BenchmarkInsert_Apply/Insert(2,_xxx)_to_short_content-8       | 80.875 ns/iter |
| BenchmarkInsert_Apply/Insert(0,_x)_to_short_content-8         | 77.508 ns/iter |
| BenchmarkInsert_Apply/Insert(0,_xxx)_to_short_content-8       | 79.690 ns/iter |
| BenchmarkInsert_Apply/Insert(10,_x)_to_short_content-8        | 77.624 ns/iter |
| BenchmarkInsert_Apply/Insert(10,_xxx)_to_short_content-8      | 78.196 ns/iter |
| BenchmarkInsert_Apply/Insert(2,_x)_to_long_content-8          | 312.87 ns/iter |
| BenchmarkInsert_Apply/Insert(2,_xxx)_to_long_content-8        | 343.84 ns/iter |
| BenchmarkInsert_Apply/Insert(0,_x)_to_long_content-8          | 348.94 ns/iter |
| BenchmarkInsert_Apply/Insert(0,_xxx)_to_long_content-8        | 348.94 ns/iter |
| BenchmarkInsert_Apply/Insert(6890,_x)_to_long_content-8       | 347.36 ns/iter |
| BenchmarkInsert_Apply/Insert(6890,_xxx)_to_long_content-8     | 343.75 ns/iter |
| BenchmarkInsert_Apply/Insert(0,_<long>)_to_short_content-8    | 37.138 ns/iter |
| BenchmarkInsert_Apply/Insert(3,_<long>)_to_short_content-8    | 36.937 ns/iter |
| BenchmarkInsert_Apply/Insert(3,_<long>)_to_short_content#01-8 | 37.295 ns/iter |


Результаты benchmark'ов для `Delete.Apply` 

| Benchmark | Среднее время операции **Rust**| 
| --- | --- |
| BenchmarkDelete_Apply/Delete(2,_1)_to_short_content-8    | 34.677 ns/iter |
| BenchmarkDelete_Apply/Delete(2,_3)_to_short_content-8    | 34.743 ns/iter |
| BenchmarkDelete_Apply/Delete(0,_1)_to_short_content-8    | 34.075 ns/iter |
| BenchmarkDelete_Apply/Delete(0,_3)_to_short_content-8    | 34.601 ns/iter |
| BenchmarkDelete_Apply/Delete(0,_10)_to_short_content-8   | 5.6319 ns/iter |
| BenchmarkDelete_Apply/Delete(10,_100)_to_long_content-8  | 84.052 ns/iter |
| BenchmarkDelete_Apply/Delete(10,_1000)_to_long_content-8 | 95.390 ns/iter |
| BenchmarkDelete_Apply/Delete(0,_100)_to_long_content-8   | 95.390 ns/iter |
| BenchmarkDelete_Apply/Delete(0,_1000)_to_long_content-8  | 94.392 ns/iter |
| BenchmarkDelete_Apply/Delete(0,_6890)_to_long_content-8  | 5.6328 ns/iter |
