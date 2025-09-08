// Ви можете вносити шляхи модулів (module paths) у область видимості
// і давати їм нові імена за допомогою ключових слів `use` і `as`.

mod delicious_snacks {
    // TODO: Додайте наступні два оператори `use` після їх виправлення.
    // use self::fruits::PEAR as ???;
    // use self::veggies::CUCUMBER as ???;

    mod fruits {
        pub const PEAR: &str = "Груша";
        pub const APPLE: &str = "Яблуко";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Огірок";
        pub const CARROT: &str = "Морква";
    }
}

fn main() {
    println!(
        "улюблені закуски: {} і {},"
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    );
}
