#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    // Приклад: 42 / 0
    DivideByZero,
    // Єдиний випадок для `i64`: `i64::MIN / -1`, тому що результат — `i64::MAX + 1`
    IntegerOverflow,
    // Приклад: 5 / 2 = 2.5
    NotDivisible,
}

fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }

    if a == i64::MIN && b == -1 {
        return Err(DivisionError::IntegerOverflow);
    }

    if a % b != 0 {
        return Err(DivisionError::NotDivisible);
    }

    Ok(a / b)
}

fn result_with_list() -> Result<Vec<i64>, DivisionError> {
    //                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    let numbers = [27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
    // Збирає у очікуваний тип повернення. Повертає першу помилку в
    // результатах ділення в разі, якщо десь була помилка.
    division_results.collect()
}

fn list_of_results() -> Vec<Result<i64, DivisionError>> {
    //               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    let numbers = [27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
    // Збирає у очікуваний тип повернення.
    division_results.collect()
}

fn main() {
    // Тут ви можете за бажанням поекспериментувати.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
        assert_eq!(divide(81, -1), Ok(-81));
        assert_eq!(divide(i64::MIN, i64::MIN), Ok(1));
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_integer_overflow() {
        assert_eq!(divide(i64::MIN, -1), Err(DivisionError::IntegerOverflow));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(divide(81, 6), Err(DivisionError::NotDivisible));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(result_with_list().unwrap(), [1, 11, 1426, 3]);
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(list_of_results(), [Ok(1), Ok(11), Ok(1426), Ok(3)]);
    }
}
