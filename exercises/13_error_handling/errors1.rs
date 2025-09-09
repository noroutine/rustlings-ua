// TODO: Ця функція відмовляється генерувати текст для друку на бейджику, якщо
// ви передаєте їй порожній рядок. Було б краще, якби вона пояснювала, в чому проблема,
// замість простого повернення `None`. На щастя, Rust має подібну
// конструкцію до `Option`, яку можна використовувати для вираження умов помилок. Змініть
// сигнатуру функції та тіло, щоб повертати `Result<String, String>` замість
// `Option<String>`.
fn generate_nametag_text(name: String) -> Option<String> {
    if name.is_empty() {
        // Порожні імена не дозволені
        None
    } else {
        Some(format!("Привіт! Мене звати {name}"))
    }
}

fn main() {
    // Ви можете тут експериментувати, якщо бажаєте.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Привіт! Мене звати Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Порожні імена не дозволені"),
        );
    }
}
