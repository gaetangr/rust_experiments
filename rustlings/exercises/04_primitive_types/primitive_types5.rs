fn main() {
    let cat = ("Furry McFurson", 3.5);

    let (name, age) = cat; // again, tuple unpacking is very similar in Python, here we destruct it in another tuple it seems
    println!("{name} is {age} years old");
}
