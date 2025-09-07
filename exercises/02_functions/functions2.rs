// TODO: Додай відсутній тип до аргумента `num` після двокрапки `:`.
fn call_me(num:) {
    for i in 0..num {
        println!("Дзинь-дзинь! Набираю номер {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
