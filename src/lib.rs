//! Библиотека для решения задач ОГЭ по информатике 2023 год.
//!
//! ## Модули заданий
//! Для каждого задания существует отдельный модуль, содержащий всё необходимое для решения задачи.
//! Любой модуль содержит:
//! * Функцию `solve()`. Она решает задачу.
//! * Структуру `InputData`. Это входные данные задачи.
//! * Опционально перечисление `ProblemSpec`. Это детали решения задачи, они указывают библиотеке
//! что нужно найти/сделать в задаче, также могут содерать дополнительную информацию о задаче.
//! Все доступные модули можно найти ниже, кроме модуля для 11ых заданий.
//!
//! ## Примеры
//! В документации к каждому модулю можно найти пример его работы.

pub mod utils;
mod modules;

pub use modules::module1;
pub use modules::module2;
pub use modules::module3;
pub use modules::module4;
pub use modules::module5;
pub use modules::module6;
pub use modules::module7;
pub use modules::module9;
pub use modules::module10;
pub use modules::module12;

pub use utils::{SolveResult, SolveError};
