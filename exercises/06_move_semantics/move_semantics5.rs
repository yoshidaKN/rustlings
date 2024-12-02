#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or removing references (the character `&`).
// TODO: TODO: 参照（`&`文字）の追加や削除以外は何も変更せずにコンパイラー・エラーを修正する。

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
