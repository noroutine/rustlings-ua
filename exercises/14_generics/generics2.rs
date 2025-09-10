// Ця потужна обгортка надає можливість зберігати додатне ціле значення.
// TODO: Перепишіть її з використанням дженерика (generic), щоб вона підтримувала обгортання БУДЬ-ЯКОГО типу.
struct Wrapper {
    value: u32,
}

// TODO: Адаптуйте реалізацію структури, щоб вона була дженериком над обгорнутим значенням.
impl Wrapper {
    fn new(value: u32) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // Тут ви можете за бажанням поекспериментувати.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
