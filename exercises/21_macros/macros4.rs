// TODO: Виправте помилку компіляції, додавши один або два символи.
#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Подивіться на мій макрос!");
    }
    ($val:expr) => {
        println!("Подивіться на мій інший макрос: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
