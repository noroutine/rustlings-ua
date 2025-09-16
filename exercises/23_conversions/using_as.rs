// Приведення типів у Rust виконується за допомогою оператора `as`.
// Зауважте, що оператор `as` використовується не тільки для приведення типів. Він також допомагає
// з перейменуванням імпортів.

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    // TODO: Зробіть перетворення перед діленням.
    total / values.len()
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
