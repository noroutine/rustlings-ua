mod delicious_snacks {
    // Додано `pub` і використано очікуваний псевдонім (alias) після `as`.
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

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
