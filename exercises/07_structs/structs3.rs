// Структури (structs) містять дані, але також можуть мати логіку. У цій вправі ми
// визначили структуру `Package` і хочемо протестувати деяку логіку, пов'язану з нею.

#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: u32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Self {
        if weight_in_grams < 10 {
            // В Rust обробляти помилки потрібно по-іншому, але ми
            // вивчимо обробку помилок пізніше.
            panic!("Неможливо відправити пакунок вагою менше 10 грамів");
        }

        Self {
            sender_country,
            recipient_country,
            weight_in_grams,
        }
    }

    // TODO: Додайте правильний тип повернення до сигнатури функції.
    fn is_international(&self) {
        // TODO: Прочитайте тести, які використовують цей метод, щоб дізнатися, коли пакунок
        // вважається міжнародним.
    }

    // TODO: Додайте правильний тип повернення до сигнатури функції.
    fn get_fees(&self, cents_per_gram: u32) {
        // TODO: Обчисліть плату за пакунок.
    }
}

fn main() {
    // Ви можете тут експериментувати, якщо бажаєте.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Іспанія");
        let recipient_country = String::from("Австрія");

        Package::new(sender_country, recipient_country, 5);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Іспанія");
        let recipient_country = String::from("Україна");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Канада");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Україна");
        let recipient_country = String::from("Україна");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}
