// Ця програма намагається використовувати завершену версію
// функції `total_cost` з попередньої вправи. Однак вона не працює!
// Чому? Що ми повинні зробити, щоб виправити це?

use std::num::ParseIntError;

// Не змінюйте цю функцію.
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// TODO: Виправить помилку компілятора, змінивши сигнатуру та тіло
// функції `main`.
fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // Не змінюйте цей рядок.
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("Ви не можете собі дозволити стільки!");
    } else {
        tokens -= cost;
        println!("У вас тепер {tokens} токенів.");
    }
}
