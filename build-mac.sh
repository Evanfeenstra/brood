echo "building wasm..."
npm --prefix ./frontend/app run build
echo "packing..."
packr2 clean
packr2 build frontend/app.go
echo "building binary..."
rm brood.dmg
rm -rf brood.app
mkdir -p brood.app/Contents/MacOS
go build -o brood.app/Contents/MacOS/brood
cp frontend/img/Info.plist brood.app/Contents/Info.plist
mkdir -p brood.app/Contents/Resources
cp frontend/img/logo.icns brood.app/Contents/Resources/logo.icns
echo "zipping..."
hdiutil create -format UDZO -srcfolder brood.app brood.dmg
echo "done!"