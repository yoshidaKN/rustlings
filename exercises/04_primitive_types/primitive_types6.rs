fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // TODO: Use a tuple index to access the second element of `numbers` and assign it to a variable called `second`.
        // TODO: タプルインデックスを使用して`numbers`の2番目の要素にアクセスし、それを`second`という変数に割り当てます。
        // let second = ???;

        let second: i32 = numbers.1;

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}
