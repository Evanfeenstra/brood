echo "building wasm..."
npm --prefix ./frontend/app run build
echo "packing..."
packr2 clean
packr2 build frontend/app.go
echo "building binary..."
# go build
# echo "done!"

mkdir -p brood.app/Contents/MacOS
go build -o brood.app/Contents/MacOS/brood
echo "done!"