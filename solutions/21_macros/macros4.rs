// Додали крапки з комою для розділення гілок макроса (macro arms).
#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Подивіться на мій макрос!");
    };
    ($val:expr) => {
        println!("Подивіться на мій інший макрос: {}", $val);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
