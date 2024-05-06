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

### Rust では，if は式．(C では文)．if 文と if 式の違いは，式は値を返す，文は値を返さないという理解であっている？

<https://qiita.com/hinastory/items/e97d5459b9cda45758db#%E3%82%BB%E3%83%9F%E3%82%B3%E3%83%AD%E3%83%B3%E3%81%AE%E6%96%87%E5%8C%BA%E5%88%87%E3%82%8Asemicolon-statement-separation>

> Rust の関数は主に「文」で構成され、一番最後は「文」か「式」で終わります。Rust の文と文の区切りはセミコロン「;」です。従って最後にセミコロンが付くか否かで文か式かを区別できます。

```rust
fn test1() -> i32 {
    let mut x = 2; // 文
    x =  x * 3 ;   // 文
    x + 5          // 式（戻り値は11）
}

fn test2() -> () {
    let mut x = 2; // 文
    x =  x * 3 ;   // 文
    x + 5;         // 文（戻り値はユニット`()`）
}
```

<https://zenn.dev/toga/books/rust-atcoder/viewer/if>

> いまは「どちらか一方だけが実行されることが保証できる」場合の例としてこのようなコードを挙げました．しかし，今回の場合に関しては，実際はこのような書き方をせず，代わりに let abs = if x >= 0 { x } else { -x }; と書くのが普通です（次章で説明します）．

> このように書くと，ブロックが実行されたとき，ブロック内の最後の式の値がブロック全体の値となります．これを「ブロックが値を返す」と言います．

<https://qiita.com/namn1125/items/ccedf9cc06b8cef71557>

> let-else 文の詳細を説明する前に、まずは Rust の if-let 式について説明いたします。Rust は式指向言語のため if も標準で式になっています。よく他言語では三項演算子使用で宗教戦争が起きていますが「if"式"があれば争いなんて起きないのに...(ﾄｵｲﾒ」といつも思っています。
