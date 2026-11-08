use std::fs::OpenOptions;
use std::io::{self, BufWriter, Write};
use std::path::Path;

pub const DEFAULT_FILE_PATH: &str = "sample_data.txt";
pub const DEFAULT_LINES: usize = 100_000;

/// Создаёт тестовый файл размером ~7-8 МБ, если он ещё не существует.
pub fn ensure_test_file(path: &str, line_count: usize) -> io::Result<u64> {
    let p = Path::new(path);
    if p.exists() {
        return Ok(p.metadata()?.len());
    }

    println!("Генерация тестового файла '{}' ({} строк)...", path, line_count);
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    let mut writer = BufWriter::new(file);
    for i in 1..=line_count {
        writeln!(
            writer,
            "Строка {:06}: Привет из подкаста «Rust в деталях»! Демонстрация работы BufReader и системных вызовов.",
            i
        )?;
    }
    writer.flush()?;

    let bytes = p.metadata()?.len();
    println!(
        "Файл '{}' успешно создан ({:.2} МБ).\n",
        path,
        bytes as f64 / (1024.0 * 1024.0)
    );
    Ok(bytes)
}
