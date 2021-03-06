# brood wallet

IOTA Shimmer testnet wallet desktop application. Build with [Yew](https://github.com/yewstack/yew) and [Webview](https://github.com/webview/webview).

![](https://github.com/Evanfeenstra/brood/blob/master/frontend/img/screenshot.jpg?raw=true)

**[Download Desktop App Here](https://github.com/Evanfeenstra/brood/releases)**

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
- In another terminal:
- `cd ../../`
- `export DEV=true`
- `go build`
- `./brood`

### build
- `chmod +x build.sh`
- `./build.sh`

### deploy
You can also run Brood as a web service from your goshimmer node, by adding this to your docker-compose:
```yml
brood:
    image: evanfeenstra/brood:latest
    container_name: brood
    hostname: brood
    restart: unless-stopped
    ports:
      - "0.0.0.0:3888:3888/tcp"
    networks:
      - outside
    volumes:
      - "/root/.config/brood:/root/.config/brood:rw"
```
Then visit your IP at port 3888

### or plain docker
Run on "shimmer" docker network
```bash
docker run -d --net=shimmer --name=brood -p 0.0.0.0:3888:3888/tcp -v /root/.config/brood:/root/.config/brood:rw docker.io/evanfeenstra/brood:latest
```

### or with go
```bash
# this removes the Webview dependency
mv main.no tmp.no && mv main.no main.go
# build
go build
# run on 3887 or any port you want
PORT=3887 ./brood
```

### support me!
Want to see more features in Brood Wallet? Send some IOTA my way...
```
XIELBIKVLGUOKXMQOJ9NEVYCBUKUVNHZPAXJQEMGBXTYV9RL9H9YIFCWUNUIO9XQTBSABZZZGUYQNWPJYHRTSXEOZW
```