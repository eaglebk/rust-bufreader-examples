// Пример 3: BufReader + повторное использование буфера строки (.read_line)
// -----------------------------------------------------------------------
// Вместо создания нового String для каждой строки, мы передаём ссылку &mut line.
// В конце каждой итерации делаем line.clear(): ёмкость буфера (capacity) сохраняется,
// поэтому новых аллокаций памяти в куче (heap allocations) не происходит вообще!
//
// Это даёт максимальную производительность при потоковой обработке больших файлов.

use rust_bufreader_examples::{ensure_test_file, DEFAULT_FILE_PATH, DEFAULT_LINES};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

fn main() -> io::Result<()> {
    ensure_test_file(DEFAULT_FILE_PATH, DEFAULT_LINES)?;

    println!("============================================================");
    println!("Пример 3: BufReader + .read_line(&mut line) с переиспользованием");
    println!("Максимальная скорость: буферизация + 0 лишних аллокаций памяти");
    println!("============================================================");

    let file = File::open(DEFAULT_FILE_PATH)?;
    let start = Instant::now();

    let mut reader = BufReader::new(file);
    let mut line = String::with_capacity(256); // Заранее резервируем немного памяти
    let mut lines_count = 0usize;
    let mut total_bytes = 0usize;

    // read_line возвращает количество прочитанных байт. 0 означает EOF (конец файла).
    while reader.read_line(&mut line)? > 0 {
        lines_count += 1;
        total_bytes += line.len();

        // Очищаем содержимое строки, но сохраняем выделенную память под буфер!
        line.clear();
    }

    let duration = start.elapsed();
    println!("Прочитано строк:   {}", lines_count);
    println!("Прочитано байт:    {}", total_bytes);
    println!("Время выполнения:  {:.2?} (ещё быстрее, чем .lines())", duration);
    println!("Преимущество:      Одна строка на весь цикл — ноль мусора в памяти.\n");

    Ok(())
}

