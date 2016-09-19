pentry: Process Entry Inspection Library for Rust
=================================================

[mitchellh/go-ps](https://github.com/mitchellh/go-ps) のようにプロセスの実行ファイルを取得したかった。

使い方 - Usage -
----------------

### 依存関係の追加 - Add Dependencies -

```toml
[dependencies]
pentry = "0.1.*"
```

### サンプルコード - Samples -

#### 自分自身を取得するサンプル - Inspect SELF process -

```rust
extern crate pentry;

if let Ok(ps) = pentry::current() {
    println!("{:?}", ps);
}
```

#### 親プロセスを取得する - Inspect Parent Process -

```rust
extern crate pentry;

if let Ok(ps) = pentry::current() {
    println!("Current: {:?}", ps);

    // 1. Use pentry::find
    if let Ok(parent) = pentry::find(ps.ppid()) {
        println!("Parent: {:?}", parent);
    }

    // 2. Use `parent` member function.
    println!("Parent: {:?}", ps.parent().unwrap());
}
```

作者 - Author -
---------------

**TANAKA Kenichi aka chikuchikugonzalez (ちくちく('ω')ごんざれす)**

- [chiku2gonzalez on Twitter](https://twitter.com/chiku2gonzalez)
- [chikuchikugonzalez on Hatena Blog](http://chiku2gonzalez.hatenablog.com/)

ライセンス - LICENSE -
----------------------
[MIT License](http://chiku2gonzalez.bitbucket.org/license/MITv2016.txt)
