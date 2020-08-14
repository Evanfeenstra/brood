
### frontend

export RUSTFLAGS=--cfg=web_sys_unstable_apis

npm run start:dev

### server

export DEV=true

go build

./brood
