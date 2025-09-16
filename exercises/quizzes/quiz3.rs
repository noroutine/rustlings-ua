// Цей квіз перевіряє:
// - Дженерики (Generics)
// - Трейти (Traits)
//
// Уявна магічна школа має нову систему генерації рапортів успішності, написану
// на Rust! Наразі система підтримує лише створення рапортів, де
// оцінка студента представлена числово (наприклад, 1.0 -> 5.5). Однак
// школа також виставляє літерні оцінки (A+ -> F-) і потребує можливості
// друкувати обидва типи рапортів!
//
// Зробіть необхідні зміни в структурі `ReportCard` і блоці impl,
// щоб підтримувати літерні рапорти додатково до числових.

// TODO: Налаштуйте структуру, як описано вище.
struct ReportCard {
    grade: f32,
    student_name: String,
    student_age: u8,
}

// TODO: Налаштуйте блок impl, як описано вище.
impl ReportCard {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade,
        )
    }
}

fn main() {
    // Ви можете експериментувати тут за бажанням.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}
