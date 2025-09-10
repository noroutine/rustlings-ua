trait Licensed {
    // TODO: Додайте реалізацію для `licensing_info`, щоб реалізатори трейта
    // (як дві структури нижче) могли спільно використовувати цю
    // поведінку за замовчуванням, не повторюючи функцію.
    // Ліцензійна інформація за замовчуванням має бути рядок "Стандартна ліцензія".
    fn licensing_info(&self) -> String;
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // Не редагуйте цей рядок.
impl Licensed for OtherSoftware {} // Не редагуйте цей рядок.

fn main() {
    // Тут ви можете за бажанням поекспериментувати.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = "Стандартна ліцензія";
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
