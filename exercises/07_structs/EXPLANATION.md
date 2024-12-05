# 分からなかったところ

## `structs1.rs`

Rustの構造体について

### フィールド構造体

```rust
struct Point {
    x: f32,
    y: f32,
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
}
```

### タプル構造体

```rust
struct Pair(i32, f32);

fn main() {
    let x = 1;
    let y = 0.1;
    let pair = Pair(x, y);
}
```

### ユニット構造体

```rust
struct Unit;

fn main() {
    let _unit = Unit;
}
```

## `structs2.rs`

構造体の更新記法を用いて、別の構造体のフィールドの値を基に新しい構造体インスタンスを作成する方法

```rust
struct Point {
    x: f32,
    y: f32,
}

fn main() {
    // Pointインスタンスを作成
    let point: Point = Point { x: 10.3, y: 0.4 };

    // peterを元に、bottom_rightを作成
    let bottom_right = Point { x: 5.2, ..point };
}
```