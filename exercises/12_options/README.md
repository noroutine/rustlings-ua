# Опції (Options)

Тип Option представляє опціональне значення: кожен Option є або Some і містить значення, або None і не містить.
Типи Option дуже поширені в Rust коді, оскільки вони мають ряд використань:

- Початкові значення
- Повернені значення для функцій, які не визначені на всьому діапазоні вхідних даних (часткові функції)
- Повернене значення для повідомлення про прості помилки, де None повертається при помилці
- Опціональні поля структур
- Поля структур, які можуть бути поozziчені або «взяті»
- Опціональні аргументи функцій
- Нульові вказівники
- Обмін речей у складних ситуаціях

## Додаткова інформація

- [Option Enum Format](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-enum-definitions)
- [Option Module Documentation](https://doc.rust-lang.org/std/option/)
- [Option Enum Documentation](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)
