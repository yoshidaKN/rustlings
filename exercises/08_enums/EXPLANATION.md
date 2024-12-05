# 分からなかったところ

## `enums2.rs` & `enums3.rs` 

列挙型は、取りうる値を列挙することで、 型を定義することができる。例えば、色を受け取るような関数がある時に、

```rust
fn convert_color(color: String) -> (u8, u8, u8) {
    if color == "red" {
        return (255, 0, 0);
    } else if color == "green" {
        return (0, 255, 0);
    } else if color == "blue" {
        return (0, 0, 255);
    } else {
        panic!("unknown color");
    }
}
```

等とすると、 `else` 文のように知らない `color` 文字列を受け取った際にエラーを発しないと行けない。そこで、 `color` が受け取りうる値を定義するような型を定義することで、このような自体を防ぐことができる。

```rust
enum Message {
    red,
    green,
    blue,
}

fn convert_color(color: Color) -> (u8, u8, u8) {
    macth color {
        Color::red => (255, 0, 0),
        Color::green => (0, 255, 0),
        Color::blue => (0, 0, 255),
    }
}
```

ここまでは一般的なプログラミング言語と同じだが、Rustでは列挙体の要素に「データ」をもたせることができる。これは、以下のようなケースに有用である。例えば、IPアドレスがなりうる種類を表す列挙体 `IpAddrKind` を定義する。

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

現状では、 実際のIPアドレスのデータを保持する方法がないため、このままだと以下のように実装することになる。

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

`IpAddrKind` と `IpAddr` は統合できそうである。

Rustでは、各enumの列挙子に直接データを格納して、enumを構造体内に使うというよりもenumだけを使って、 同じ概念をもっと簡潔な方法で表現することができる。

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    //...
}
```

また、 IPv4は 255.255.255.255 の形で表現できるが、 IPv6は表記が異なる。その場合、列挙体の要素ごとに異なる型を持たせたい。 Rustでは、要素ごとに異なるデータ型を保持させることができる。

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    //...
}
```

また、 IPv4とIPv6で異なる構造体を持たせたい場合もある。

```rust
struct Ipv4Addr {
    // 省略
}

struct Ipv6Addr {
    // 省略
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

また、変数名付きタプルを持たせたい場合は、以下のようになる。

```rust
enum Message{
    Resize { width: u64, height: u64 },
    // ...
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        // ...
    // ...
}
```

## `enum3.rs`

