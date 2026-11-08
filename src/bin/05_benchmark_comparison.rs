// Пример 5: Сводный бенчмарк всех 4 методов чтения
// -------------------------------------------------
// Автоматически генерирует тестовый файл (~7-8 МБ) и по очереди запускает
// все 4 подхода, выводя красивую сравнительную таблицу времени и системных вызовов.

use rust_bufreader_examples::{ensure_test_file, DEFAULT_FILE_PATH, DEFAULT_LINES};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read};
use std::time::Instant;

fn bench_unbuffered(path: &str) -> io::Result<(usize, std::time::Duration)> {
    let mut file = File::open(path)?;
    let start = Instant::now();
    let mut buf = [0u8; 1];
    let mut count = 0usize;
    while file.read(&mut buf)? > 0 {
        if buf[0] == b'\n' {
            count += 1;
        }
    }
    Ok((count, start.elapsed()))
}

fn bench_bufreader_lines(path: &str) -> io::Result<(usize, std::time::Duration)> {
    let file = File::open(path)?;
    let start = Instant::now();
    let reader = BufReader::new(file);
    let mut count = 0usize;
    for line in reader.lines() {
        let _ = line?;
        count += 1;
    }
    Ok((count, start.elapsed()))
}

fn bench_bufreader_reuse(path: &str) -> io::Result<(usize, std::time::Duration)> {
    let file = File::open(path)?;
    let start = Instant::now();
    let mut reader = BufReader::new(file);
    let mut line = String::with_capacity(256);
    let mut count = 0usize;
    while reader.read_line(&mut line)? > 0 {
        count += 1;
        line.clear();
    }
    Ok((count, start.elapsed()))
}

fn bench_read_entire_file(path: &str) -> io::Result<(usize, std::time::Duration)> {
    let start = Instant::now();
    let text = fs::read_to_string(path)?;
    let mut count = 0usize;
    for _ in text.lines() {
        count += 1;
    }
    Ok((count, start.elapsed()))
}

fn main() -> io::Result<()> {
    let file_size = ensure_test_file(DEFAULT_FILE_PATH, DEFAULT_LINES)?;
    let mb = file_size as f64 / (1024.0 * 1024.0);

    println!("================================================================================");
    println!(" 🦀 Сравнение способов чтения файлов в Rust (Подкаст «Rust в деталях», Выпуск 1)");
    println!("================================================================================");
    println!(" Тестовый файл:     '{}' ({:.2} МБ, {} строк)", DEFAULT_FILE_PATH, mb, DEFAULT_LINES);
    println!(" Внутренний буфер:  BufReader = 8 КБ (8192 байта по умолчанию)");
    println!("--------------------------------------------------------------------------------");
    println!(" Запуск замеров...\n");

    print!(" [1/4] Замер 1: Побайтово без буфера (Read)... (подождите ~8-12 сек)... ");
    io::Write::flush(&mut io::stdout())?;
    let (c1, t1) = bench_unbuffered(DEFAULT_FILE_PATH)?;
    println!("готово ({:.2?})", t1);

    print!(" [2/4] Замер 2: BufReader + .lines()... ");
    io::Write::flush(&mut io::stdout())?;
    let (c2, t2) = bench_bufreader_lines(DEFAULT_FILE_PATH)?;
    println!("готово ({:.2?})", t2);

    print!(" [3/4] Замер 3: BufReader + переиспользование строки... ");
    io::Write::flush(&mut io::stdout())?;
    let (c3, t3) = bench_bufreader_reuse(DEFAULT_FILE_PATH)?;
    println!("готово ({:.2?})", t3);

    print!(" [4/4] Замер 4: Весь файл в память (fs::read_to_string)... ");
    io::Write::flush(&mut io::stdout())?;
    let (c4, t4) = bench_read_entire_file(DEFAULT_FILE_PATH)?;
    println!("готово ({:.2?})\n", t4);

    assert_eq!(c1, c2);
    assert_eq!(c2, c3);
    assert_eq!(c3, c4);

    let speedup_lines = t1.as_secs_f64() / t2.as_secs_f64();
    let speedup_reuse = t1.as_secs_f64() / t3.as_secs_f64();
    let speedup_all = t1.as_secs_f64() / t4.as_secs_f64();

    println!("================================================================================");
    println!("                               ИТОГОВАЯ ТАБЛИЦА                                 ");
    println!("================================================================================");
    println!("{:<36} | {:<12} | {:<10} | {:<14}", "Подход", "Время", "Ускорение", "Syscalls / RAM");
    println!("-------------------------------------+--------------+------------+----------------");
    println!("{:<36} | {:<12.2?} | {:<10} | ~{:.0} syscalls", "1. Без буфера по 1 байту (Read)", t1, "базовое", file_size);
    println!("{:<36} | {:<12.2?} | {:<9.1}x | ~{} syscalls", "2. BufReader + lines()", t2, speedup_lines, file_size / 8192);
    println!("{:<36} | {:<12.2?} | {:<9.1}x | ~{} syscalls (0 alloc)", "3. BufReader + reuse String", t3, speedup_reuse, file_size / 8192);
    println!("{:<36} | {:<12.2?} | {:<9.1}x | 1 syscall ({:.1} МБ RAM)", "4. Весь файл в память (read_to_string)", t4, speedup_all, mb);
    println!("================================================================================\n");

    println!("💡 Вывод: BufReader с переиспользованием строки даёт оптимальный баланс —");
    println!("   ускорение в ~{:.0} раз при минимальном потреблении памяти!\n", speedup_reuse);

    Ok(())
}

