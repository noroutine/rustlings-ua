// Час життя (lifetimes) також потрібен, коли структури містять посилання.

struct Book<'a> {
    //     ^^^^ додано анотацію часу життя
    author: &'a str,
    //       ^^
    title: &'a str,
    //      ^^
}

fn main() {
    let book = Book {
        author: "Джордж Орвелл",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
