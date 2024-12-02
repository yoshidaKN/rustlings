# 分からなかったところ

## `move_semantics1.rs`

```rust
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    // コンパイルエラー！
    vec.push(88);

    vec
}

fn main(){
    let vec0 = vec![22, 44, 66];
    let vec1 = fill_vec(vec0);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}
```

ベクタ型変数の `push` を使う場合は、 `mut` キーワードを付けて可変変数として引数を受け取る必要がある。

```rust
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);
    vec
}
```

## `move_semantics2.rs` と `move_semantics3.rs`

「所有権の移動」に関する問題。

Rustである変数を他の変数に束縛するときは、 moveと呼ばれる所有権の移動が行われ、元の変数は無効化される。下記の例では `a` の値がmoveした後に再度 `a` を利用しようとしたため、エラーとなっている。moveによって所有権を失った値を利用するということは、既にメモリ解放が行われた値を参照する可能性があり、予期せぬ挙動の原因となるためコンパイルエラーとしている。

```rust
fn foo(b: i32) {
    /* 直接的にbを所有して利用 */
} // この関数がbの所有権を持っているのでDropされる

fn main() {
    let a = 42;
    foo(a);

    // コンパイルエラー！
    // a は fooに所有されてしまうので、使えなくなる
    println!("{}", a); // error[E0382]: use of moved value: `a`
}
```

`move_semantics2.rs` のコードは以下の通り（若干修正）。

```rust
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    let vec0 = vec![22, 44, 66];
    let vec1 = fill_vec(vec0);

    // コンパイルエラー！
    assert_eq!(vec0, [22, 44, 66]); 
    assert_eq!(vec1, [22, 44, 66, 88]);
}
```

`vec0` は `fill_vec` 関数に渡される際に所有権が移動している。 `i32` 型と異なり、`Vec<i32>` 型は `Copy` トレイトをデフォルト実装していない型のため、関数にそのまま所有権が移動する。

関数 `fill_vec` は受け取った `vec` の所有権を消費して新しいベクタを生成して返しており、その結果、`vec0` は `fill_vec` 呼び出し後に使えなくなり、`assert_eq!(vec0, ...)` の部分でコンパイルエラーが発生している。

解決策として、所有権を渡さずに参照できるようにする。 `fill_vec` は参照だけを受け取る引数とし、その内部で `vec` を複製して操作し、その値を返す。

```rust
fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    // 参照を元にベクタを複製
    let mut new_vec = vec.clone(); 
    // 操作
    new_vec.push(88);
    // 複製値を返す
    new_vec
}

fn main(){
    let vec0 = vec![22, 44, 66];
    let vec1 = fill_vec(&vec0); // 参照を渡す

    assert_eq!(vec0, [22, 44, 66]); 
    assert_eq!(vec1, [22, 44, 66, 88]);
}
```

もしくは、 `vec0` のクローン（複製値）を直接渡してしまう方法（問題的にはこっちが正しそう）。

```rust
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main(){
    let vec0 = vec![22, 44, 66];
    let vec1 = fill_vec(vec0.clone());

    assert_eq!(vec0, [22, 44, 66]); 
    assert_eq!(vec1, [22, 44, 66, 88]);
}
```

## `move_semantics4.rs`

ミュータブル参照に関する問題。

ミュータブル参照 `&mut` は値を書き換えるための参照。所有権は移動せず、元の値を安全に変更できるが、同時に1つのミュータブル参照しか作成できない。

また、ミュータブル参照は排他的な利用が保証される。例えば、ある変数のミュータブル参照が存在する間、その参照が指すデータに対して他の参照を作成したり、元のデータを直接操作することができない。つまり、排他的利用の制限により、同時に複数のミュータブル参照（ `&mut` ）を作成してはいけない。

```rust
fn main() {
    let mut x = 42;
    let r1 = &mut x; // ミュータブル参照 r1 を作成

    // コンパイルエラー！
    let r2 = &mut x; // 別のミュータブル参照 r2 を作成しようとする

    *r1 += 1;
    *r2 += 1;

    println!("{}", x);
}
```

この場合、 `r1` のライフタイムが有効な間に、別のミュータブル参照 `r2` を作成しているため、排他的利用の制限に違反しエラーとなる。

`move_semantics4.rs` を見る。

```rust
fn main() {
    let mut x = Vec::new();
    let y = &mut x;
    let z = &mut x;
    y.push(42);
    z.push(13);
    assert_eq!(x, [42, 13]);
}
```

このコードでは、 `let y = &mut x;` によって `x` のミュータブル参照 `y` が作成されている。そして、その後にもう一つのミュータブル参照 `z` を作成しようとしているため、Rustの借用ルールに違反してエラーとなる。

```rust
fn main() {
    // `x` はミュータブルなベクタ
    let mut x = Vec::new();  

    // `x` のミュータブル参照 `y` を作成
    let y = &mut x;          
    // `y` を使用してベクタに値を追加
    y.push(42);              

    // `y` のライフタイムが終了後に
    // `x` の新しいミュータブル参照 `z` を作成
    let z = &mut x;          
    // `z` を使用してベクタに値を追加
    z.push(13);              

    // 期待どおりの結果
    assert_eq!(x, [42, 13]); 
}
```

## `move_semantics5.rs`

借用に関する問題。

`get_char` 関数が呼び出されると、変数 `data` の所有権が関数に移動するため、その後利用することができない。しかし、 `get_char()` 関数自体は、`data` の末尾文字を返すだけの処理であり、値自体の操作を行うわけではないので、この場合は 参照 を渡すだけで十分である。このように所有権を必要としない操作をする場合に、関数に所有権を渡さずに安全に処理する方法を **借用** という。

```rust
// TODO: Fix the compiler errors without changing anything except adding or removing references (the character `&`).
// TODO: TODO: 参照（`&`文字）の追加や削除以外は何も変更せずにコンパイラー・エラーを修正する。

fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: &String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(data);

    string_uppercase(&data);
}
```

`get_char` に `data` の参照を渡す。更に、 `string_uppercase` 関数呼び出し以降は `data` を使っていないので、所有権をそのまま渡すので良い（と思う。その後も `data` の所有権を関数に握らせたくないならもっとコード書き換えないとだし。）

```rust
// Shouldn't take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}
```