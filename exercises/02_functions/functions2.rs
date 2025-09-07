// TODO: Додай відсутній тип до аргумента `num` після двокрапки `:`.
fn call_me(num:) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
