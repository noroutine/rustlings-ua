fn main() {
    let cat = ("Пушок МакВусенко", 3.5);

    // Деструктуризація кортежу.
    let (name, age) = cat;

    println!("{name} має вік {age} років");
}
