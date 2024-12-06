# 分からなかったところ

## `hashmaps1.rs`

ハッシュマップは、連想配列とか辞書型とか呼ばれるデータ型である。新規ハッシュマップを生成する場合は、以下の通りである。

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

