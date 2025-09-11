// Під час компіляції Rust повинен знати, скільки місця в памʼяті займає тип. Це
// проблематично для рекурсивних типів, де значення може містити
// як свою частину інше значення того самого типу. Щоб обійти цю проблему, ми
// можемо використати `Box` — розумний показчик (smart pointer), який використовується
// для зберігання даних у купі (heap), що також дозволяє нам обгорнути рекурсивний тип.
//
// Рекурсивний тип, який ми реалізуємо в цій вправі, — це "список cons", структура
// даних, що часто зустрічається в функціональних мовах програмування. Кожен
// елемент у списку cons містить два елементи: значення поточного елемента та
// наступний елемент. Останній елемент — це значення, назване `Nil`.

#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn create_empty_list() -> List {
    List::Nil
}

fn create_non_empty_list() -> List {
    List::Cons(42, Box::new(List::Nil))
}

fn main() {
    println!("Це порожній список: {:?}", create_empty_list());
    println!(
        "Це непорожній список: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}
