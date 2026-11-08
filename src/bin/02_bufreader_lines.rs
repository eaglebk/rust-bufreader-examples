// Пример 2: Чтение через BufReader + .lines()
// ---------------------------------------------
// BufReader считывает данные блоками по 8 КБ (8192 байта) во внутренний буфер в памяти.
// Вместо миллионов обращений к ядру ОС остаётся всего несколько сотен.
//
// Аналогия из подкаста:
// «BufReader — это кружка нормального размера. Налил из колонки сразу кусок и спокойно пьёшь».

use rust_bufreader_examples::{ensure_test_file, DEFAULT_FILE_PATH, DEFAULT_LINES};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

fn main() -> io::Result<()> {
    ensure_test_file(DEFAULT_FILE_PATH, DEFAULT_LINES)?;

    println!("============================================================");
    println!("Пример 2: Чтение через BufReader::new(file).lines()");
    println!("Стандартный, удобный и быстрый способ в Rust");
    println!("============================================================");

    let file = File::open(DEFAULT_FILE_PATH)?;
    let start = Instant::now();

    // Оборачиваем File в BufReader
    let reader = BufReader::new(file);

    let mut lines_count = 0usize;
    let mut total_chars = 0usize;

    // .lines() возвращает удобный итератор по строкам
    for line_result in reader.lines() {
        let line = line_result?;
        lines_count += 1;
        total_chars += line.len();
    }

    let duration = start.elapsed();
    println!("Прочитано строк:   {}", lines_count);
    println!("Суммарная длина:   {} символов", total_chars);
    println!("Время выполнения:  {:.2?} (ускорение в сотни раз!)", duration);
    println!("Особенность:       .lines() на каждой строке выделяет новую String.\n");

    Ok(())
}
