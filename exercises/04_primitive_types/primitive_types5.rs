fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // TODO: println が機能するように、1 つのステートメントで `cat` タプルを分解します。
    // let /* your pattern here */ = cat;

    let name = cat.0;
    let age = cat.1;

    println!("{name} is {age} years old");
}
