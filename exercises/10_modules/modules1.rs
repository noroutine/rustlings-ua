// TODO: Виправте помилку компілятора про виклик приватної функції.
mod sausage_factory {
    // Не дозволяйте нікому поза цим модулем бачити цю функцію!
    fn get_secret_recipe() -> String {
        String::from("Імбир")
    }

    fn make_sausage() {
        get_secret_recipe();
        println!("ковбаса!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
