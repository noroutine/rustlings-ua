fn main() {
    let is_morning = true;
    if is_morning {
        println!("Доброго ранку!");
    }

    let is_evening = !is_morning;
    if is_evening {
        println!("Доброго вечора!");
    }
}
