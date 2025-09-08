struct ColorRegularStruct {
    // TODO: Додайте поля (fields), які очікує тест `regular_structs`.
    // Якого типу повинні бути поля? Які мінімальні та максимальні значення для RGB кольорів?
}

struct ColorTupleStruct(/* TODO: Додайте поля (fields), які очікує тест `tuple_structs` */);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // Ви можете тут експериментувати, якщо бажаєте.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Створіть екземпляр звичайної структури (struct).
        // let green =

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Створіть екземпляр структури-кортежу (tuple struct).
        // let green =

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Створіть екземпляр одиничної структури (unit struct).
        // let unit_struct =
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
