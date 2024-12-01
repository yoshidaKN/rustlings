# 分からなかったところ

## `vecs1.rs`

「ベクタ」はサイズを変更可能な配列である。スライスと同様、そのサイズはコンパイル時には不定だが、いつでも要素を追加したり削除したりすることができる。ベクタは以下の三要素で完全に決定できる。

- データへのポインタ
- 長さ
- 容量（あらかじめメモリ上にベクタのために確保された領域）

ベクタはその容量を超えない限りにおいて長くしていくことができる。

```rust
fn main(){
    // ベクタの初期化には`vec!`マクロが使用できる。
    let v: Vec<i32> = vec![10, 20, 30, 40];
    println!("Initial vector: {:?}", xs);

    // 新しい要素をベクタの最後に挿入することができる。
    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // 鍵括弧(`[]`)を用いてインデックスによる要素へのアクセスができる
    // （インデックスは0から開始する）
    println!("Second element: {}", xs[1]);

    // `pop`はベクタの最後の要素を削除すると同時に返す。
    println!("Pop last element: {:?}", xs.pop());

    // `Vector`は簡単にイテレートできます。
    println!("Contents of xs:");
    for x in xs.iter() {
        println!("> {}", x);
    }
}
```

## `vecs2.rs`

mapメソッドは各要素に関数を適用します：

```rust
fn main(){
    let a = [1, 2, 3];
    let mut iter = a.iter().map(|x| 2 * x);
}
```