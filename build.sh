echo "building wasm..."
npm --prefix ./frontend/app run build
echo "packing..."
packr2 clean
packr2 build frontend/app.go
echo "building binary..."
go build
echo "done!"