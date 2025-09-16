// AsRef і AsMut дозволяють дешеві перетворення посилання-у-посилання. Прочитайте більше
// про них за адресами https://doc.rust-lang.org/std/convert/trait.AsRef.html і
// https://doc.rust-lang.org/std/convert/trait.AsMut.html відповідно.

// Отримати кількість байтів (не символів) у наданому аргументі
// (`.len()` повертає кількість байтів у рядку).
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().len()
}

// Отримати кількість символів (не байтів) у наданому аргументі.
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Підносить число до квадрату використовуючи `as_mut()`.
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    let arg = arg.as_mut();
    *arg *= *arg;
}

fn main() {
    // Ви можете експериментувати тут за бажанням.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
