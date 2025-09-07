# [Растуни](https://rustlings.rust-lang.org) 🦀

Ласкаво просимо до вправ Rustlings українською 🇺🇦 😃

Невеликі вправи, щоб звикнути до читання та написання коду на [Rust](https://www.rust-lang.org) - _Рекомендується паралельно з читанням [офіційної книги Rust](https://doc.rust-lang.org/book)_

Відвідай **веб-сайт** для демо, інформації про налаштування та більше: ➡️ [rustlings.rust-lang.org](https://rustlings.rust-lang.org)

_Доступний також [український переклад офіційної книги](https://rust-lang-ua.github.io/rustbook_ukrainian/), який ми теж рекомендуємо, але основним авторитетним джерелом про поточну версію мови є завжди оригінал англійською._

# Як почати

Звісно, тобі потрібен налаштований Rust та встановлений Rustlings як розказано нижче в розділі [Як встановити Rustlings](#як-встановити-rustlings).

Потім склонуй цей репозиторій, відкрий термінал у теці з репозиторієм та запусти `rustlings`, щоб почати працювати з вправами 🚀

# Як встановити Rustlings

Це переклад [офіційної інструкцієї](https://github.com/rust-lang/rustlings) ✅

## Встановлення Rust

Перед встановленням Rustlings ти повинен мати встановлену **останню версію Rust**.

Це дуже просто якщо в тебе **Unix-подібна система** (Linux/macOS),:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

_Для інших систем та особливих потреб дивись офіційні інструкції [www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)._

Це також встановить _Cargo_ - менеджер пакетів/проектів Rust.

> 🐧 Якщо ти використовуєш **Linux**, переконайся, що у тебе встановлено `gcc` (_для лінкера_).
>
> Debian: `sudo apt install gcc`\
> Fedora: `sudo dnf install gcc`

> 🍎 Якщо ти використовуєш **MacOS**, переконайся, що у тебе встановлено _Xcode та його інструменти розробника_: `xcode-select --install`

## Встановлення Rustlings

Наступна команда завантажить та скомпілює Rustlings:

```bash
cargo install rustlings
```

**Якщо встановлення не вдається...**

- Переконайся, що у тебе остання версія Rust, запустивши `rustup update`
- Спробуй додати прапорець `--locked`: `cargo install rustlings --locked`
- В іншому випадку, будь ласка, [повідом про проблему](https://github.com/rust-lang/rustlings/issues/new)

## Ініціалізація

Після встановлення Rustlings запусти наступну команду для ініціалізації директорії `rustlings/`:

```bash
rustlings init
```

**Якщо команда `rustlings` не знайдена...**

Ти, ймовірно, використовуєш Linux і встановив Rust через менеджер пакетів.

Cargo встановлює виконувані файли в директорію `~/.cargo/bin`.
На жаль, менеджери пакетів часто не додають `~/.cargo/bin` до твоєї змінної середовища `PATH`.

- Або додай `~/.cargo/bin` вручну до `PATH`
- Або видали Rust з менеджера пакетів і [встанови його офіційним способом через `rustup`](https://www.rust-lang.org/tools/install)

Тепер перейди в щойно ініціалізовану директорію та запусти Rustlings для подальших інструкцій щодо початку роботи з вправами:

```bash
cd rustlings/
rustlings
```

## Робоче середовище

### Редактор

Ми загалом рекомендуємо [VS Code](https://code.visualstudio.com/) з [плагіном rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
Але будь-який редактор, що підтримує [rust-analyzer](https://rust-analyzer.github.io/), повинен підійти для роботи з вправами.

### Термінал

Під час роботи з Rustlings, будь ласка, використовуй сучасний термінал для найкращого досвіду.
Стандартний термінал на Linux та Mac повинен бути достатнім.
