// A basket of fruits in the form of a hash map needs to be defined. The key
// represents the name of the fruit and the value represents how many of that
// particular fruit is in the basket. You have to put at least 3 different
// types of fruits (e.g. apple, banana, mango) in the basket and the total count
// of all the fruits should be at least 5.

// ハッシュ マップの形式でフルーツのバスケットを定義する必要があります。
// キーはフルーツの名前を表し、値はバスケットに入っている特定のフルーツの数を表します。
// バスケットには少なくとも 3 種類のフルーツ (リンゴ、バナナ、マンゴーなど) を入れ、
// すべてのフルーツの合計数は少なくとも 5 個である必要があります。

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // TODO: Declare the hash map.
    let mut basket = HashMap::new();

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);

    // TODO: Put more fruits in your basket.
    basket.insert(String::from("apple"), 2);
    basket.insert(String::from("mongo"), 1);

    basket
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
