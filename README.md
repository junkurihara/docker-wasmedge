# docker-wasmedge

Sample project for docker with wasmedge development projects

## Benchmark result

See [`bench.sh`](./bench.sh) for the benchmark script.

### Latency

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

### Throughput

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
