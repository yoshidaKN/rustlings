// You can use the `use` keyword to bring module paths from modules from
// anywhere and especially from the standard library into your scope.
// `use` キーワードを使用すると、どこからでも、特に標準ライブラリのモジュールから
// モジュール パスをスコープに取り込むことができます。

// TODO: Bring `SystemTime` and `UNIX_EPOCH` from the `std::time` module into
// your scope. Bonus style points if you can do it with one line!
// TODO: `SystemTime` と `UNIX_EPOCH` を `std::time` モジュールからスコープに取り込みます。
// 1 行で実行できれば、ボーナス スタイル ポイントが付与されます。

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
