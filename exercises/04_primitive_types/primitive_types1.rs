// Booleans (`bool`)

fn main() {
    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    // TODO: Define a boolean variable with the name `is_evening` before the `if` statement below.
    // The value of the variable should be the negation (opposite) of `is_morning`.
    // TODO: 以下の `if` ステートメントの前に、`is_evening` という名前のブール変数を定義します。
    // 変数の値は `is_morning` の否定 (反対) である必要があります。
    let is_evening: bool = !is_morning;
    if is_evening {
        println!("Good evening!");
    }
}
