// Час життя (lifetimes) також потрібен, коли структури містять посилання.

// TODO: Виправте помилки компілятора стосовно структури.
struct Book {
    author: &str,
    title: &str,
}

fn main() {
    let book = Book {
        author: "Джордж Орвелл",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
