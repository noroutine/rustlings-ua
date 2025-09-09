// Ця вправа є зміненою версією вправи `errors4`. Вона використовує деякі
// концепції, до яких ми дійдемо пізніше в курсі, як от `Box` та
// трейт `From`. Неважливо розуміти їх детально прямо зараз, але
// ви можете почитати наперед, якщо хочете. Поки що, думайте про тип `Box<dyn ???>` як
// "я хочу будь-що, що робить ???" тип.
//
// Коротко кажучи, цей конкретний випадок використання boxes для того, коли ви хочете володіти
// значенням і вам важливо лише те, що воно є типом, який реалізує певний
// трейт. Для цього `Box` оголошується як тип `Box<dyn Trait>`, де
// `Trait` - це трейт, який компілятор шукає на будь-якому значенні, що використовується в
// тому контексті. Для цієї вправи цей контекст - це потенційні помилки, які
// можуть бути повернені в `Result`, тобто трейт Error

use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// Це необхідно, щоб `CreationError` міг реалізувати `Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "число негативне",
            CreationError::Zero => "число дорівнює нулю",
        };
        f.write_str(description)
    }
}

impl Error for CreationError {}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// TODO: Додайте правильний тип повернення `Result<(), Box<dyn ???>>`. Що ми можемо
// використовувати, щоб описати обидві помилки? Чи є трейт, який реалізують обидві помилки?
fn main() {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}
