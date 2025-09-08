mod sausage_factory {
    fn get_secret_recipe() -> String {
        String::from("Імбир")
    }

    // Додано `pub` перед `fn`, щоб зробити функцію доступною поза модулем.
    pub fn make_sausage() {
        get_secret_recipe();
        println!("ковбаса!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
