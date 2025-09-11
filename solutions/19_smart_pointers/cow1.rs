// Ця вправа досліджує розумний показчик `Cow` (Clone-On-Write). Він може
// обгорнути та надати незмінюваний доступ до запозичених даних і ліниво
// клонувати дані, коли потрібна змінюваність або володіння значенням.
// Цей тип розроблено для роботи з загальними запозиченими даними через
// трейт `Borrow`.

use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for ind in 0..input.len() {
        let value = input[ind];
        if value < 0 {
            // Клонує вектор, якщо значення запозичене.
            input.to_mut()[ind] = -value;
        }
    }
}

fn main() {
    // Тут ви можете за бажанням поекспериментувати.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() {
        // Клонування відбувається, оскільки `input` потрібно змінити.
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn reference_no_mutation() {
        // Клонування не відбувається, оскільки `input` не потрібно змінювати.
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Borrowed(_)));
        //                      ^^^^^^^^^^^^^^^^
    }

    #[test]
    fn owned_no_mutation() {
        // Ми також можемо передати `vec` без `&`, щоб `Cow` володів ним безпосередньо. У цьому
        // випадку зміна значення не відбувається (всі числа вже абсолютні) і, отже,
        // також немає клонування. Але `Cow` все ще володіє результатом, оскільки він ніколи
        // не був запозиченим чи зміненим.
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
        //                      ^^^^^^^^^^^^^
    }

    #[test]
    fn owned_mutation() {
        // Звичайно, це також стосується випадку, коли зміна дійсно відбувається (не всі
        // числа абсолютні). У цьому випадку виклик `to_mut()` у функції
        // `abs_all` повертає посилання на ті самі дані, що і раніше.
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
        //                      ^^^^^^^^^^^^^
    }
}
