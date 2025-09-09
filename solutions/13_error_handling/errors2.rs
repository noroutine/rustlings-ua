// Скажімо, ми пишемо гру, де ви можете купувати предмети за токени. Всі предмети коштують
// 5 токенів, і щоразу, коли ви купуєте предмети, є комісія за обробку 1
// токен. Гравець гри введе, скільки предметів він хоче купити, і
// функція `total_cost` обчислить загальну вартість предметів. Оскільки
// гравець ввів кількість, ми отримуємо її як рядок. Він міг набрати
// що завгодно, не лише числа!
//
// Прямо зараз ця функція взагалі не обробляє випадки помилок. Те, що ми хочемо
// зробити: Якщо ми викличемо функцію `total_cost` на рядку, який не є
// числом, ця функція поверне `ParseIntError`. У цьому випадку ми хочемо
// негайно повернути цю помилку з нашої функції і не намагатися множити та
// додавати.
//
// Є принаймні два способи реалізувати це, обидва правильні. Але один
// набагато коротший!

use std::num::ParseIntError;

#[allow(unused_variables, clippy::question_mark)]
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // Додано `?` для поширення помилки.
    let qty = item_quantity.parse::<i32>()?;
    //                                         ^ додано

    // Еквівалентно цій детальній версії:
    let qty = match item_quantity.parse::<i32>() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    Ok(qty * cost_per_item + processing_fee)
}

fn main() {
    // Ви можете тут експериментувати, якщо бажаєте.
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("трицять").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }
}
