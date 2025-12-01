
```sh
cargo new --lib ldpreload
```

```sh
LD_PRELOAD="./target/debug/libldpreload.so" ./cpp/main
```