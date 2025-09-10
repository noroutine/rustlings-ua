trait Licensed {
    fn licensing_info(&self) -> String {
        "Стандартна ліцензія".to_string()
    }
}

struct SomeSoftware;
struct OtherSoftware;

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// TODO: Виправте помилку компілятора, змінивши лише сигнатуру цієї функції.
fn compare_license_types(software1: ???, software2: ???) -> bool {
    software1.licensing_info() == software2.licensing_info()
}

fn main() {
    // Тут ви можете за бажанням поекспериментувати.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        assert!(compare_license_types(SomeSoftware, OtherSoftware));
    }

    #[test]
    fn compare_license_information_backwards() {
        assert!(compare_license_types(OtherSoftware, SomeSoftware));
    }
}
