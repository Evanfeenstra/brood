# brood wallet

IOTA Shimmer testnet wallet desktop application. Build with [Yew](https://github.com/yewstack/yew) and [Webview](https://github.com/webview/webview).

![](https://github.com/Evanfeenstra/brood/blob/master/frontend/img/screenshot.jpg?raw=true)

### features
- Send and receive IOTA shimmer testnet tokens
- Create new tokens on Shimmer
- Connect to your own Shimmer node

### future plans
- Connect to wasp node and deploy/manage smart contracts
- Refactor to 100% Rust (maybe with [tauri](https://github.com/tauri-apps/tauri))
- Integrate [Stronghold](https://github.com/iotaledger/stronghold.rs) to encrypt wallet state
- Integrate Goshimmer [webauth](https://github.com/iotaledger/goshimmer/tree/develop/plugins/webauth)
- Responsive design, mobile build

### start in dev mode

- `cd frontend app`
- `npm run start:dev`
In another terminal:
- `cd ../../`
- `export DEV=true`
- `go build`
- `./brood`

### build
- `chmod +x build.sh`
- `./build.sh`