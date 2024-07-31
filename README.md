# docker-wasmedge

Sample project for docker with wasmedge development projects

## Benchmark result

See [`bench.sh`](./bench.sh) for the benchmark script.

### amd64 native

- iMac 2020 (Intel Core i9, RAM128GB)
- wasmedge optimization level: 3

  ```bash:
  wasmedge compile --optimize 3 ./target/wasm32-wasi/release/docker-wasmedge.wasm docker-wasmedge-aot.wasm
  ```

**Due to the limitation (?) of the wasmedge runtime, we did the latency test with 8 connections as well as the throughput test.**

#### Latency

##### Release build

```bash:
rewrk -c 8 -t 4 -d 15s -h http://localhost:8080 --pct
Beginning round 1...
Benchmarking 8 connections @ http://localhost:8080 for 15 second(s)
Latencies:
    Avg      Stdev    Min      Max
    0.07ms   0.02ms   0.03ms   2.61ms
  Requests:
    Total: 1762487 Req/Sec: 117519.58
  Transfer:
    Total: 164.72 MB Transfer Rate: 10.98 MB/Sec
+ --------------- + --------------- +
|   Percentile    |   Avg Latency   |
+ --------------- + --------------- +
|      99.9%      |     0.39ms      |
|       99%       |     0.19ms      |
|       95%       |     0.13ms      |
|       90%       |     0.11ms      |
|       75%       |     0.09ms      |
|       50%       |     0.08ms      |
+ --------------- + --------------- +
```

##### Unoptimized wasmedge + native wasmedge runtime

```bash:
rewrk -c 8 -t 4 -d 15s -h http://localhost:8080 --pct
Beginning round 1...
Benchmarking 8 connections @ http://localhost:8080 for 15 second(s)
  Latencies:
    Avg      Stdev    Min      Max
    2.47ms   0.35ms   1.31ms   9.51ms
  Requests:
    Total:  48612  Req/Sec: 3240.92
  Transfer:
    Total: 4.54 MB Transfer Rate: 310.17 KB/Sec
+ --------------- + --------------- +
|   Percentile    |   Avg Latency   |
+ --------------- + --------------- +
|      99.9%      |     6.74ms      |
|       99%       |     4.92ms      |
|       95%       |     3.65ms      |
|       90%       |     3.21ms      |
|       75%       |     2.82ms      |
|       50%       |     2.62ms      |
+ --------------- + --------------- +
```

##### Optimized wasmedge + native wasmedge runtime

```bash:
rewrk -c 8 -t 4 -d 15s -h http://localhost:8080 --pct
Beginning round 1...
Benchmarking 8 connections @ http://localhost:8080 for 15 second(s)
  Latencies:
    Avg      Stdev    Min      Max
    0.19ms   0.08ms   0.07ms   3.32ms
  Requests:
    Total: 632487  Req/Sec: 42168.91
  Transfer:
    Total: 59.11 MB Transfer Rate: 3.94 MB/Sec
+ --------------- + --------------- +
|   Percentile    |   Avg Latency   |
+ --------------- + --------------- +
|      99.9%      |     0.67ms      |
|       99%       |     0.51ms      |
|       95%       |     0.39ms      |
|       90%       |     0.34ms      |
|       75%       |     0.30ms      |
|       50%       |     0.25ms      |
+ --------------- + --------------- +
```

#### Throughput

##### Release build

```bash:
rewrk -c 8 -t 4 -d 15s -h http://localhost:8080/heavy --pct
Beginning round 1...
Benchmarking 8 connections @ http://localhost:8080/heavy for 15 second(s)
  Latencies:
    Avg      Stdev    Min      Max
    14.84ms  0.82ms   2.37ms   24.83ms
  Requests:
    Total:  8083   Req/Sec: 538.91
  Transfer:
    Total: 773.57 KB Transfer Rate: 51.58 KB/Sec
+ --------------- + --------------- +
|   Percentile    |   Avg Latency   |
+ --------------- + --------------- +
|      99.9%      |     23.56ms     |
|       99%       |     20.16ms     |
|       95%       |     17.61ms     |
|       90%       |     16.67ms     |
|       75%       |     15.66ms     |
|       50%       |     15.17ms     |
+ --------------- + --------------- +
```

##### Unoptimized wasmedge + native wasmedge runtime

```bash:
rewrk -c 8 -t 4 -d 15s -h http://localhost:8080/heavy --pct
Beginning round 1...
Benchmarking 8 connections @ http://localhost:8080/heavy for 15 second(s)
  Latencies:
    Avg      Stdev    Min      Max
    1681.72ms  560.84ms  224.91ms  3206.13ms
  Requests:
    Total:   67    Req/Sec:  4.47
  Transfer:
    Total: 6.41 KB Transfer Rate: 437.73 B/Sec
+ --------------- + --------------- +
|   Percentile    |   Avg Latency   |
+ --------------- + --------------- +
|      99.9%      |    3206.13ms    |
|       99%       |    3206.13ms    |
|       95%       |    3159.34ms    |
|       90%       |    3130.06ms    |
|       75%       |    2353.89ms    |
|       50%       |    1957.32ms    |
+ --------------- + --------------- +
```

##### Optimized wasmedge + native wasmedge runtime

```bash:
rewrk -c 8 -t 4 -d 15s -h http://localhost:8080/heavy --pct
Beginning round 1...
Benchmarking 8 connections @ http://localhost:8080/heavy for 15 second(s)
  Latencies:
    Avg      Stdev    Min      Max
    16.44ms  5.11ms   2.38ms   36.74ms
  Requests:
    Total:  7292   Req/Sec: 486.14
  Transfer:
    Total: 697.87 KB Transfer Rate: 46.53 KB/Sec
+ --------------- + --------------- +
|   Percentile    |   Avg Latency   |
+ --------------- + --------------- +
|      99.9%      |     36.23ms     |
|       99%       |     32.55ms     |
|       95%       |     29.63ms     |
|       90%       |     29.03ms     |
|       75%       |     23.18ms     |
|       50%       |     18.76ms     |
+ --------------- + --------------- +
```

---

### amd64 docker

- iMac 2020 (Intel Core i9, RAM128GB)
- Docker for Mac v4.32.0
- wasmedge optimization level: 3

  ```bash:
  wasmedge compile --optimize 3 ./target/wasm32-wasi/release/docker-wasmedge.wasm docker-wasmedge-aot.wasm
  ```

#### Latency

We evaluated the latency of the server by simply sending a request to get a kind of echo response, where 8 requests were sent concurrently. The latency can be estimated as the number of requests processed per second.

##### Native build + docker

```bash:
Benchmark [Latency]
----------------------------
Benchmark on native docker
Beginning round 1...
Benchmarking 8 connections @ http://localhost:8080 for 15 second(s)
  Latencies:
    Avg      Stdev    Min      Max
    0.94ms   0.35ms   0.27ms   5.78ms
  Requests:
    Total: 127591  Req/Sec: 8506.51
  Transfer:
    Total: 11.92 MB Transfer Rate: 814.11 KB/Sec
+ --------------- + --------------- +
|   Percentile    |   Avg Latency   |
+ --------------- + --------------- +
|      99.9%      |     4.17ms      |
|       99%       |     2.55ms      |
|       95%       |     1.98ms      |
|       90%       |     1.71ms      |
|       75%       |     1.39ms      |
|       50%       |     1.17ms      |
+ --------------- + --------------- +
```

##### Unpoptimized wasmedge + docker

```bash:
Benchmark on wasmedge docker
Beginning round 1...
Benchmarking 8 connections @ http://localhost:8081 for 15 second(s)
  Latencies:
    Avg      Stdev    Min      Max
    42.24ms  2.01ms   5.37ms   46.07ms
  Requests:
    Total:  2834   Req/Sec: 188.96
  Transfer:
    Total: 271.74 KB Transfer Rate: 18.12 KB/Sec
+ --------------- + --------------- +
|   Percentile    |   Avg Latency   |
+ --------------- + --------------- +
|      99.9%      |     45.92ms     |
|       99%       |     44.58ms     |
|       95%       |     43.95ms     |
|       90%       |     43.56ms     |
|       75%       |     43.21ms     |
|       50%       |     42.85ms     |
+ --------------- + --------------- +
```

##### Optimized wasmedge + docker

```bash:
----------------------------
Benchmark on optimized wasmedge docker
Beginning round 1...
Benchmarking 8 connections @ http://localhost:8082 for 15 second(s)
  Latencies:
    Avg      Stdev    Min      Max
    42.06ms  2.06ms   4.17ms   44.76ms
  Requests:
    Total:  2848   Req/Sec: 189.88
  Transfer:
    Total: 273.16 KB Transfer Rate: 18.21 KB/Sec
+ --------------- + --------------- +
|   Percentile    |   Avg Latency   |
+ --------------- + --------------- +
|      99.9%      |     44.76ms     |
|       99%       |     44.28ms     |
|       95%       |     43.81ms     |
|       90%       |     43.46ms     |
|       75%       |     43.09ms     |
|       50%       |     42.64ms     |
+ --------------- + --------------- +
```

#### Throughput

We evaluated the throughput of the server by calculating the Fibonacci number of 30 for each request by sending 8 requests concurrently. The throughput can be estimated as the number of requests processed per second.

##### Native build + docker

```bash:
Benchmark [Throughput (Calculating Fibonacchi 40)]
----------------------------
Benchmark on native docker
Beginning round 1...
Benchmarking 8 connections @ http://localhost:8080/heavy for 15 second(s)
  Latencies:
    Avg      Stdev    Min      Max
    19.51ms  2.24ms   4.19ms   45.40ms
  Requests:
    Total:  6147   Req/Sec: 409.81
  Transfer:
    Total: 588.29 KB Transfer Rate: 39.22 KB/Sec
+ --------------- + --------------- +
|   Percentile    |   Avg Latency   |
+ --------------- + --------------- +
|      99.9%      |     42.70ms     |
|       99%       |     28.77ms     |
|       95%       |     25.74ms     |
|       90%       |     24.61ms     |
|       75%       |     22.68ms     |
|       50%       |     20.95ms     |
+ --------------- + --------------- +
```

##### Unoptimized wasmedge + docker

```bash
Benchmark on wasmedge docker
Beginning round 1...
Benchmarking 8 connections @ http://localhost:8081/heavy for 15 second(s)
  Latencies:
    Avg      Stdev    Min      Max
    2007.72ms  676.28ms  263.39ms  3850.83ms
  Requests:
    Total:   55    Req/Sec:  3.67
  Transfer:
    Total: 5.26 KB Transfer Rate: 359.32 B/Sec
+ --------------- + --------------- +
|   Percentile    |   Avg Latency   |
+ --------------- + --------------- +
|      99.9%      |    3850.83ms    |
|       99%       |    3850.83ms    |
|       95%       |    3847.74ms    |
|       90%       |    3806.71ms    |
|       75%       |    2813.67ms    |
|       50%       |    2354.49ms    |
+ --------------- + --------------- +
```

##### Optimized wasmedge + docker

```bash
enchmark on optimized wasmedge docker
Beginning round 1...
Benchmarking 8 connections @ http://localhost:8082/heavy for 15 second(s)
  Latencies:
    Avg      Stdev    Min      Max
    2073.12ms  703.04ms  289.42ms  3991.88ms
  Requests:
    Total:   53    Req/Sec:  3.53
  Transfer:
    Total: 5.07 KB Transfer Rate: 346.27 B/Sec
+ --------------- + --------------- +
|   Percentile    |   Avg Latency   |
+ --------------- + --------------- +
|      99.9%      |    3991.88ms    |
|       99%       |    3991.88ms    |
|       95%       |    3964.78ms    |
|       90%       |    3901.07ms    |
|       75%       |    2903.74ms    |
|       50%       |    2443.95ms    |
+ --------------- + --------------- +
```

---

<!-- ### aarch64

- Macbook Pro 2021 (M1 Max, RAM64GB)
- Docker for Mac v4.32.0

#### Latency

We evaluated the latency of the server by simply sending a request to get a kind of echo response, where 512 requests were sent concurrently. The latency can be estimated as the number of requests processed per second.

```bash
Benchmark [Latency]
----------------------------
Benchmark on native docker
Beginning round 1...
Benchmarking 512 connections @ http://localhost:8080 for 15 second(s)
  Latencies:
    Avg      Stdev    Min      Max
    6.42ms   2.06ms   0.65ms   108.28ms
  Requests:
    Total: 1193769 Req/Sec: 79598.40
  Transfer:
    Total: 111.50 MB Transfer Rate: 7.43 MB/Sec
+ --------------- + --------------- +
|   Percentile    |   Avg Latency   |
+ --------------- + --------------- +
|      99.9%      |     21.56ms     |
|       99%       |     16.07ms     |
|       95%       |     12.63ms     |
|       90%       |     11.07ms     |
|       75%       |     9.15ms      |
|       50%       |     7.83ms      |
+ --------------- + --------------- +

747 Errors: error shutting down connection: Socket is not connected (os error 57)

sleep 3 secs
----------------------------
Benchmark on wasmedge docker
Beginning round 1...
Benchmarking 512 connections @ http://localhost:8081 for 15 second(s)
  Latencies:
    Avg      Stdev    Min      Max
    158.10ms  23.63ms  0.66ms   235.10ms
  Requests:
    Total:  48226  Req/Sec: 3216.14
  Transfer:
    Total: 4.43 MB Transfer Rate: 302.57 KB/Sec
+ --------------- + --------------- +
|   Percentile    |   Avg Latency   |
+ --------------- + --------------- +
|      99.9%      |    218.88ms     |
|       99%       |    200.59ms     |
|       95%       |    182.08ms     |
|       90%       |    178.07ms     |
|       75%       |    174.57ms     |
|       50%       |    169.53ms     |
+ --------------- + --------------- +

927 Errors: error shutting down connection: Socket is not connected (os error 57)
1 Errors: connection closed
```

#### Throughput

We evaluated the throughput of the server by calculating the Fibonacci number of 30 for each request by sending 8 requests concurrently. The throughput can be estimated as the number of requests processed per second.

```bash
Benchmark [Throughput (Calculating Fibonacchi 40)]
----------------------------
Benchmark on native docker
Beginning round 1...
Benchmarking 8 connections @ http://localhost:8080/heavy for 15 second(s)
  Latencies:
    Avg      Stdev    Min      Max
    20.09ms  1.56ms   11.56ms  43.72ms
  Requests:
    Total:  5969   Req/Sec: 397.97
  Transfer:
    Total: 571.25 KB Transfer Rate: 38.09 KB/Sec
+ --------------- + --------------- +
|   Percentile    |   Avg Latency   |
+ --------------- + --------------- +
|      99.9%      |     37.96ms     |
|       99%       |     27.12ms     |
|       95%       |     24.48ms     |
|       90%       |     23.18ms     |
|       75%       |     21.52ms     |
|       50%       |     20.81ms     |
+ --------------- + --------------- +

sleep 3 secs
----------------------------
Benchmark on wasmedge docker
Beginning round 1...
Benchmarking 8 connections @ http://localhost:8081/heavy for 15 second(s)
  Latencies:
    Avg      Stdev    Min      Max
    1604.36ms  520.08ms  246.67ms  3015.86ms
  Requests:
    Total:   70    Req/Sec:  4.67
  Transfer:
    Total: 6.70 KB Transfer Rate: 457.38 B/Sec
+ --------------- + --------------- +
|   Percentile    |   Avg Latency   |
+ --------------- + --------------- +
|      99.9%      |    3015.86ms    |
|       99%       |    3015.86ms    |
|       95%       |    2988.36ms    |
|       90%       |    2978.69ms    |
|       75%       |    2221.69ms    |
|       50%       |    1845.02ms    |
+ --------------- + --------------- +
```

### amd64

- iMac 2020 (Intel Core i9, RAM128GB)
- Docker for Mac v4.32.0

#### Latency

```bash:
Benchmark [Latency]
----------------------------
Benchmark on native docker
Beginning round 1...
Benchmarking 512 connections @ http://localhost:8080 for 15 second(s)
  Latencies:
    Avg      Stdev    Min      Max
    11.27ms  2.78ms   1.48ms   45.72ms
  Requests:
    Total: 680579  Req/Sec: 45379.06
  Transfer:
    Total: 63.55 MB Transfer Rate: 4.24 MB/Sec
+ --------------- + --------------- +
|   Percentile    |   Avg Latency   |
+ --------------- + --------------- +
|      99.9%      |     29.90ms     |
|       99%       |     23.35ms     |
|       95%       |     18.81ms     |
|       90%       |     17.11ms     |
|       75%       |     14.90ms     |
|       50%       |     13.27ms     |
+ --------------- + --------------- +

662 Errors: error shutting down connection: Socket is not connected (os error 57)

sleep 3 secs
----------------------------
Benchmark on wasmedge docker
Beginning round 1...
Benchmarking 512 connections @ http://localhost:8081 for 15 second(s)
  Latencies:
    Avg      Stdev    Min      Max
    245.41ms  40.06ms  5.94ms   318.63ms
  Requests:
    Total:  30992  Req/Sec: 2066.72
  Transfer:
    Total: 2.84 MB Transfer Rate: 193.85 KB/Sec
+ --------------- + --------------- +
|   Percentile    |   Avg Latency   |
+ --------------- + --------------- +
|      99.9%      |    311.63ms     |
|       99%       |    294.21ms     |
|       95%       |    278.44ms     |
|       90%       |    274.95ms     |
|       75%       |    271.82ms     |
|       50%       |    264.18ms     |
+ --------------- + --------------- +

684 Errors: error shutting down connection: Socket is not connected (os error 57)

```

#### Throughput

```bash:
----------------------------
Benchmark [Throughput (Calculating Fibonacchi 40)]
----------------------------
Benchmark on native docker
Beginning round 1...
Benchmarking 8 connections @ http://localhost:8080/heavy for 15 second(s)
  Latencies:
    Avg      Stdev    Min      Max
    16.09ms  0.39ms   3.95ms   18.69ms
  Requests:
    Total:  7454   Req/Sec: 496.95
  Transfer:
    Total: 713.37 KB Transfer Rate: 47.56 KB/Sec
+ --------------- + --------------- +
|   Percentile    |   Avg Latency   |
+ --------------- + --------------- +
|      99.9%      |     18.34ms     |
|       99%       |     17.69ms     |
|       95%       |     17.05ms     |
|       90%       |     16.80ms     |
|       75%       |     16.50ms     |
|       50%       |     16.31ms     |
+ --------------- + --------------- +

sleep 3 secs
----------------------------
Benchmark on wasmedge docker
Beginning round 1...
Benchmarking 8 connections @ http://localhost:8081/heavy for 15 second(s)
  Latencies:
    Avg      Stdev    Min      Max
    1810.39ms  606.45ms  242.47ms  3440.35ms
  Requests:
    Total:   61    Req/Sec:  4.07
  Transfer:
    Total: 5.91 KB Transfer Rate: 403.62 B/Sec
+ --------------- + --------------- +
|   Percentile    |   Avg Latency   |
+ --------------- + --------------- +
|      99.9%      |    3440.35ms    |
|       99%       |    3440.35ms    |
|       95%       |    3409.81ms    |
|       90%       |    3391.69ms    |
|       75%       |    2517.18ms    |
|       50%       |    2108.27ms    |
+ --------------- + --------------- +
``` -->
