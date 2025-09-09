// Використання універсальних типів помилок, таких як `Box<dyn Error>`, не рекомендується для
// коду бібліотек, де користувачі можуть хотіти приймати рішення на основі вмісту помилки
// замість виведення її чи поширення далі. Тут ми визначаємо
// власний тип помилки, щоб дозволити користувачам вирішити, що робити далі,
// коли наша функція повертає помилку.

use std::num::ParseIntError;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// Власний тип помилки, який ми будемо використовувати в `PositiveNonzeroInteger::parse`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> Self {
        Self::Creation(err)
    }

    fn from_parse_int(err: ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

// Як альтернативне рішення, реалізація трейта `From` дозволяє
// автоматичне перетворення з `ParseIntError` в `ParsePosNonzeroError`
// використовуючи оператор `?`, без потреби викликати `map_err`.
//
// ```
// let x: i64 = s.parse()?;
// ```
//
// Трейти, такі як `From`, будуть розглянуті в подальших вправах.
impl From<ParseIntError> for ParsePosNonzeroError {
    fn from(err: ParseIntError) -> Self {
        ParsePosNonzeroError::ParseInt(err)
    }
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(Self(x as u64)),
        }
    }

    fn parse(s: &str) -> Result<Self, ParsePosNonzeroError> {
        // Повернути відповідну помилку замість паніки, коли `parse()`
        // повертає помилку.
        let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parse_int)?;
        //                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        Self::new(x).map_err(ParsePosNonzeroError::from_creation)
    }
}

fn main() {
    // Ви можете тут експериментувати, якщо бажаєте.
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert!(matches!(
            PositiveNonzeroInteger::parse("не число"),
            Err(ParsePosNonzeroError::ParseInt(_)),
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            PositiveNonzeroInteger::parse("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative)),
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            PositiveNonzeroInteger::parse("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero)),
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42).unwrap();
        assert_eq!(x.0, 42);
        assert_eq!(PositiveNonzeroInteger::parse("42"), Ok(x));
    }
}
