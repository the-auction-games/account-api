dapr run --app-id userapi --app-port 8000 --dapr-http-port 3500 --components-path ./components -- cargo run
echo ""
echo "** PROGRAM CLOSED **"
