fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

fn main() {
    string_slice("синій");

    string("червоний".to_string());

    string(String::from("привіт"));

    string("rust цікавий!".to_owned());

    // Тут обидва варіанти працюють.
    // `.into()` перетворює тип в очікуваний тип.
    // Якщо він викликається де очікується `String`, він перетворить `&str` на `String`.
    string("гарна погода".into());
    // Але якщо він викликається де очікується `&str`, то `&str` залишається `&str`, оскільки перетворення не потрібно.
    // Якщо ви приберете рядок `#[allow(…)]`, то Clippy скаже вам прибрати `.into()` нижче, оскільки це марне перетворення.
    #[allow(clippy::useless_conversion)]
    string_slice("гарна погода".into());

    string(format!("Станція {}", "Арсенальна"));

    // ПОПЕРЕДЖЕННЯ: Це байтове індексування, не символьне.
    // Символьне індексування можна виконати через `s.chars().nth(INDEX)`.
    string_slice(&String::from("абв")[0..1]);

    string_slice("  привіт там ".trim());

    string("Гарного понеділка!".replace("понеділ", "вівтор"));

    string("мОЯ КЛаВіша шИФт ЗАстрЯла".to_lowercase());
}
