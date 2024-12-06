# 分からなかったところ

## `modules1.rs`

まずどんなエラーが出るか調べてみる。

```rust
// TODO: Fix the compiler error about calling a private function.
mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
    // error[E0603]: function `make_sausage` is private
    // 15 |     sausage_factory::make_sausage();
    //                           ^^^^^^^^^^^^ private function
}
```

若干ネタバレ気味なTODOコメントにある通り、 `make_sausage` は非公開関数であるため、外部からアクセスできない。外部から関数を呼び出したい場合は、その関数が 公開関数 `pub` である必要がある。

## `modules2.rs`

`use` ステートメントの使い方について問われている問題。

`use` ステートメントは、別モジュール内のアイテムをスコープに取り込むことで、モジュール名を毎回書かずに使えるようにする。

```rust
mod my_module {
    pub fn say_hello() {
        println!("Hello, world!");
    }
}

use my_module::say_hello;

fn main() {
    say_hello(); // 直接呼び出せる
}
```

また、エイリアス `as` を使うことで、名前重複や簡略化ができる。

```rust
use std::collections::HashMap as Map;

fn main() {
    let mut my_map: Map<String, i32> = Map::new();
    my_map.insert("key".to_string(), 42);
    println!("{:?}", my_map);
}
```

問題を見る。

```rust
// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

mod delicious_snacks {
    // TODO: Add the following two `use` statements after fixing them.
    
    // before:
    // use self::fruits::PEAR as ???;
    // use self::veggies::CUCUMBER as ???;
    // after:
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    );
}
```

問題では、 `PEAR`をスコープに取り込み、`fruit`という新しい名前を付けている。更に `pub` にすることで、公開状態にしている。