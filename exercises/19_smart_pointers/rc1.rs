// У цій вправі ми хочемо виразити концепцію багатьох власників через
// тип `Rc<T>`. Це модель нашої сонячної системи - є тип `Sun` і
// кілька `Planet`. Планети отримують володіння над сонцем, що вказує,
// що вони обертаються навколо сонця.

use std::rc::Rc;

#[derive(Debug)]
struct Sun;

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Привіт з {self:?}!");
    }
}

fn main() {
    // Тут ви можете за бажанням поекспериментувати.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc1() {
        let sun = Rc::new(Sun);
        println!("reference count = {}", Rc::strong_count(&sun)); // 1 посилання

        let mercury = Planet::Mercury(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 2 посилання
        mercury.details();

        let venus = Planet::Venus(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 3 посилання
        venus.details();

        let earth = Planet::Earth(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 4 посилання
        earth.details();

        let mars = Planet::Mars(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 5 посилань
        mars.details();

        let jupiter = Planet::Jupiter(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 6 посилань
        jupiter.details();

        // TODO
        let saturn = Planet::Saturn(Rc::new(Sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 7 посилань
        saturn.details();

        // TODO
        let uranus = Planet::Uranus(Rc::new(Sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 8 посилань
        uranus.details();

        // TODO
        let neptune = Planet::Neptune(Rc::new(Sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 9 посилань
        neptune.details();

        assert_eq!(Rc::strong_count(&sun), 9);

        drop(neptune);
        println!("reference count = {}", Rc::strong_count(&sun)); // 8 посилань

        drop(uranus);
        println!("reference count = {}", Rc::strong_count(&sun)); // 7 посилань

        drop(saturn);
        println!("reference count = {}", Rc::strong_count(&sun)); // 6 посилань

        drop(jupiter);
        println!("reference count = {}", Rc::strong_count(&sun)); // 5 посилань

        drop(mars);
        println!("reference count = {}", Rc::strong_count(&sun)); // 4 посилання

        // TODO
        println!("reference count = {}", Rc::strong_count(&sun)); // 3 посилання

        // TODO
        println!("reference count = {}", Rc::strong_count(&sun)); // 2 посилання

        // TODO
        println!("reference count = {}", Rc::strong_count(&sun)); // 1 посилання

        assert_eq!(Rc::strong_count(&sun), 1);
    }
}
