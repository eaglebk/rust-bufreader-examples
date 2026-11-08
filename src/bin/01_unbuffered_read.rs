// Пример 1: Небуферизованное чтение по 1 байту
// ----------------------------------------------
// Каждый вызов .read() — это реальный системный вызов ядра ОС (read(2) на Linux/macOS).
// Чтение 8 МБ побайтово совершает ~8 миллионов системных вызовов.
//
// Аналогия из подкаста:
// «Каждый раз, когда хочется пить, бегать на улицу к колонке за одним глотком воды».

use rust_bufreader_examples::{ensure_test_file, DEFAULT_FILE_PATH, DEFAULT_LINES};
use std::fs::File;
use std::io::{self, Read};
use std::time::Instant;

fn main() -> io::Result<()> {
    ensure_test_file(DEFAULT_FILE_PATH, DEFAULT_LINES)?;

    println!("============================================================");
    println!("Пример 1: Чтение файла по 1 байту (БЕЗ BufReader)");
    println!("Внимание: выполняется очень долго из-за миллионов syscalls!");
    println!("============================================================");

    let mut file = File::open(DEFAULT_FILE_PATH)?;
    let start = Instant::now();

    let mut byte_buffer = [0u8; 1];
    let mut total_bytes = 0usize;
    let mut newlines_count = 0usize;

    // Читаем строго по одному байту за раз
    while file.read(&mut byte_buffer)? > 0 {
        total_bytes += 1;
        if byte_buffer[0] == b'\n' {
            newlines_count += 1;
        }
    }

    let duration = start.elapsed();
    println!("Прочитано байт:   {}", total_bytes);
    println!("Прочитано строк:  {}", newlines_count);
    println!("Время выполнения: {:.2?} (катастрофически медленно)", duration);
    println!("Причина:          ~{} системных вызовов к ОС!\n", total_bytes);

    Ok(())
}

