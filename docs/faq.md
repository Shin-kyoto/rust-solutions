# FAQ

## 01, 02

### main.rs以外をcargo runで実行するにはどうしたらいいの？

https://rs.nkmk.me/rust-cargo-src-bin/

> ターゲットにはsrc/binに置いたファイル名やCargo.tomlに記述した名前を指定できる。

> なお、src/main.rsはパッケージ名（Cargo.tomlの[package]のname）がターゲット名となる。ターゲットが複数ある場合は明示的にパッケージ名を指定する必要がある。

> src/binディレクトリに.rsファイルを置くと、ファイル名（拡張子なし）がターゲット名となる。

package名が`echor`の場合は，以下を叩けば，main.rsをbuild, 実行できる

```bash
cargo run --bin echor
```

