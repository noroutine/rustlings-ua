#[derive(Debug)]
enum Message {
    // TODO: Визначте декілька типів повідомлень, як використано нижче.
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
