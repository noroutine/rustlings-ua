fn call_me(num: u8) {
    for i in 0..num {
        println!("Дзинь-дзинь! Набираю номер {}", i + 1);
    }
}

fn main() {
    // Щоб викликати `call_me` потрібно вказати аргумент.
    call_me(5);
}
