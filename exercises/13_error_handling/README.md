# Обробка помилок (Error handling)

Більшість помилок не настільки серйозні, щоб викликати паніку та вимагати повного зупинення програми.
Іноді, коли функція зазнає невдачі, це з причини, яку ви можете легко інтерпретувати та відповісти на неї.
Наприклад, якщо ви намагаєтеся відкрити файл, а ця операція не вдається, оскільки файл не існує, ви можете захотіти створити файл замість завершення процесу.
Такі помилки - некритичні, їх можна обробити й відновити функціональність (recover), на відміну від паніки (panic) - критичних (unrecoverable) помилок.

## Додаткова інформація

- [Обробка помилок (Error Handling)](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [Дженерики (Generics)](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Result](https://doc.rust-lang.org/rust-by-example/error/result.html)
- [Boxing errors](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html)
