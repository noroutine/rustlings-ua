// Виклики цієї функції повинні бути замінені на виклики `string_slice` або `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Ось багато значень - деякі є `String`, деякі є `&str`.
// Ваша задача - замінити `placeholder(…)` на `string_slice(…)`
// або `string(…)` залежно від того, яким ви вважаєте кожне значення.
fn main() {
    placeholder("синій");

    placeholder("червоний".to_string());

    placeholder(String::from("привіт"));

    placeholder("rust цікавий!".to_owned());

    placeholder("гарна погода".into());

    placeholder(format!("Станція {}", "Арсенальна"));

    // ПОПЕРЕДЖЕННЯ: Це байтове індексування, не символьне.
    // Символьне індексування можна виконати через `s.chars().nth(INDEX)`.
    placeholder(&String::from("абв")[0..1]);

    placeholder("  привіт там ".trim());

    placeholder("Гарного понеділка!".replace("понеділ", "вівтор"));

    placeholder("мОЯ КЛаВіша шИФт ЗАстрЯла".to_lowercase());
}
