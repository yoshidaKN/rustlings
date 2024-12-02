fn main() {
    let mut x = Vec::new();
    let y = &mut x;
    y.push(42);
    let z = &mut x;
    z.push(13);
    assert_eq!(x, [42, 13]);
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test. Don't add, change or remove any line.
    // TODO: TODO: テストの行の順番を変えることによってのみ、コンパイラー・エラーを修正する。行の追加、変更、削除はしないでください。
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42);
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
