# web

https://crates.io/crates/actix-web

## setup

```bash
cargo build
cargo run
```

## http

### websockets

https://github.com/actix/examples/tree/master/websockets

https://crates.io/crates/actix-ws

https://levelup.gitconnected.com/websockets-in-actix-web-full-tutorial-websockets-actors-f7f9484f5086

### websocat

```bash
# install
. .devcontainer/scripts/websocat.sh

# using `websocat` (https://github.com/vi/websocat)
websocat -v --ping-interval=2 ws://127.0.0.1:8080/ws
```
