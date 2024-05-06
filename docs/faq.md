# FAQ

## 01, 02

### main.rs 以外を cargo run で実行するにはどうしたらいいの？

<https://rs.nkmk.me/rust-cargo-src-bin/>

> ターゲットには src/bin に置いたファイル名や Cargo.toml に記述した名前を指定できる。

> なお、src/main.rs はパッケージ名（Cargo.toml の[package]の name）がターゲット名となる。ターゲットが複数ある場合は明示的にパッケージ名を指定する必要がある。

> src/bin ディレクトリに.rs ファイルを置くと、ファイル名（拡張子なし）がターゲット名となる。

package 名が`echor`の場合は，以下を叩けば，main.rs を build, 実行できる

```bash
cargo run --bin echor
```
