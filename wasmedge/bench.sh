echo "Benchmark [Latency]"

echo "----------------------------"
echo "Benchmark on native docker"
rewrk -c 8 -t 4 -d 15s -h http://localhost:8080 --pct

echo "sleep 3 secs"
sleep 3

echo "----------------------------"
echo "Benchmark on wasmedge docker"
# wrk -t8 -c100 -d30s http://127.0.0.1:8090/index.html
rewrk -c 8 -t 4 -d 15s -h http://localhost:8081 --pct

echo "----------------------------"
echo "Benchmark on optimized wasmedge docker"
# wrk -t8 -c100 -d30s http://127.0.0.1:8090/index.html
rewrk -c 8 -t 4 -d 15s -h http://localhost:8082 --pct

echo "sleep 3 secs"
sleep 3

echo "----------------------------"
echo "Benchmark [Throughput (Calculating Fibonacchi 40)]"

echo "----------------------------"
echo "Benchmark on native docker"
rewrk -c 8 -t 4 -d 15s -h http://localhost:8080/heavy --pct

echo "sleep 3 secs"
sleep 3

echo "----------------------------"
echo "Benchmark on wasmedge docker"
# wrk -t8 -c100 -d30s http://127.0.0.1:8090/index.html
rewrk -c 8 -t 4 -d 15s -h http://localhost:8081/heavy --pct

echo "----------------------------"
echo "Benchmark on optimized wasmedge docker"
# wrk -t8 -c100 -d30s http://127.0.0.1:8090/index.html
rewrk -c 8 -t 4 -d 15s -h http://localhost:8082/heavy --pct
