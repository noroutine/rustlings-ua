macro_rules! my_macro {
    () => {
        println!("Подивіться на мій макрос!");
    };
}

fn main() {
    my_macro!();
    //      ^
}
