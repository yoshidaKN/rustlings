# 分からなかったところ

## `strings1.rs`

Rustに登場する2つの文字列データ型について、違いを見ていく。

- `&str` 型
  - **文字列スライス**
  - **参照型** であり、文字列データへの不変の参照を表す
  - データはメモリ上の何処かにあるが、所有権は持たない
  - 固定長であり不変
  - `let s: &str = "hello";`
- `String` 型
  - **ヒープ上に格納される可変の文字列**
  - 所有型であり、自分が保持する文字列データを管理する
  - サイズが動的に変わることができ、文字列を追加・削除ができる
  - ヒープ領域を使うため、動的な文字列操作が可能
  - `let mut s: String = String::from("hello");`  
  `s.push_str(" world!"); // 文字列を追加`

※ `&str` は固定長かつ不変で所有権は持たない、 `String` は可変長かつ可変で所有権を持つ という違いは分かった。それ以上のことは後で勉強しよう...。


## `strings3.rs`

借用文字列の操作関数を定義する問題。

```rust
fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    // 戻り値型が &str なのでこれができる
    input.trim()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    // String型が戻り値なので、 String型に変換してから操作
    String::from(input) + " world!"
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    // String型が戻り値なので、 String型に変換してから操作
    String::from(input).replace("cars", "balloons")
}
```