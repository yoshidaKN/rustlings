# 分からなかったところ

## primitive_types3.rs

配列はTという単一の型のオブジェクトの集合である。長さはコンパイル時には決定されていて、 `[T; length]` という形で指定できる。

```rust
fn main(){
    // 固定長の配列（型シグネチャは冗長なので、なくても可）
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // すべての要素を0にする場合
    let ys: [i32; 500] = [0; 500];
}
```

## primitive_types4.rs

スライスは配列の一部を指すことができる。 `[starting_index..ending_index]` の形をとる。 `starting_index` はスライスの先頭の位置を表し、`ending_index` はスライスの末尾の1つ先の位置を表す。

```rust
fn main(){
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];
    
    // インデックスは０から
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // スライス
    println!("slice 2..4: {:?}", &xs[2..4]);
}
```

## primitive_types5.rs

タプルは異なる型の値の集合である。括弧 `(T1, T2, ...)` を用いて生成する。

```rust
fn main() {
    let cat = ("Furry McFurson", 3.5);

    let name = cat.0;
    let age = cat.1;

    println!("{name} is {age} years old");
}
```