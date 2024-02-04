# RustyOGE
<div align="center">

![Crates.io](https://img.shields.io/crates/v/rusty_oge?color=orange)
![docs.rs](https://img.shields.io/docsrs/rusty_oge)
![GitHub repo size](https://img.shields.io/github/repo-size/DarkSeriusCode/RustyOGE)

</div>

Библиотека для решения задач из первой части ОГЭ по информатике за 2023 год. Задачи брались с [этого](https://inf-oge.sdamgia.ru/prob_catalog) сайта.

Также имеется [CLI](https://github.com/DarkSeriusCode/RustyOGE/tree/main/cli/README.md).

## Использование
Добавить библиотеку в проект:
```toml
[dependencies]
rusty_oge = "1.3.0"
```
или
`cargo add rusty_oge`

Примеры использования можно найти в [документации](https://docs.rs/rusty_oge/1.3.0)

## Тесты
Все тесты составлены на основе заданий с [сайта](https://inf-oge.sdamgia.ru/prob_catalog).

Чтобы запустить все тесты:
`cargo test`

Для запуска тестов конкретной задачи:
`cargo test --test moduleX_tests` X - номер задачи

Некоторые задания могут иметь прикреплённые файлы, скачать которые можно с помощью скрипта `tests/download_test_files.py`
Запуск скрипта без каких либо аргументов просто скачает все файлы, если какие-то уже есть - скачает недостающие.
Запуск стрипта с флагом `--force` принудительно скачает все файлы, даже если они уже скачаны.

## Решаемые задачи
✅ -- Программа решает задачу

❌ -- Программа не может решить задачу


|№    |Статус|
|-----|------|
|№1   |  ✅  |
|№2   |  ✅  |
|№3   |  ❌  |
|№4   |  ❌  |
|№5   |  ❌  |
|№6   |  ✅  |
|№7   |  ❌  |
|№8   |  ❌  |
|№9   |  ❌  |
|№10  |  ✅  |
|№11  |  ❌  |
|№12  |  ✅  |

