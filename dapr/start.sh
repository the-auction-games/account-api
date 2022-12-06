dapr run --app-id rustapi --app-port 8000 --dapr-http-port 3500 --components-path ./components -- cargo run
echo "Program closing in 10 seconds"
sleep 10