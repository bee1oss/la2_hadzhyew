use std::fs::File;
use std::io::{Read};

fn read_file(file_name: &str, rows: usize, cols: usize) -> Option<Vec<Vec<u8>>> {
    // Открываем файл
    let file = match File::open(file_name.trim()) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Ошибка открытия файла!");
            return None;
        }
    };

    // Чтение файла
    let mut buffer = Vec::new();
    if let Ok(_) = file.take((rows * cols * 4) as u64).read_to_end(&mut buffer) {
        // Размещаем каждые 4 байта в матрице (половина байта)
        let mut matrix: Vec<Vec<u8>> = vec![vec![0; cols]; rows];
        let mut row_index = 0;
        let mut col_index = 0;

        for chunk in buffer.chunks(4) {
            // Обрабатываем каждые 4 байта (32 бита) одновременно
            let mut value = 0u32;

            for &byte in chunk {
                value = (value << 8) | byte as u32;
            }

            for i in (0..cols).rev() {
                let bits = (value >> (i * 4)) & 0xF;
                matrix[row_index][col_index] = bits as u8;
                col_index += 1;

                if col_index == cols {
                    col_index = 0;
                    row_index += 1;

                    if row_index == rows {
                        break;
                    }
                }
            }
        }

        // обмен столбцов, а то муть какая-то
        for row in &mut matrix {
            let mut ig = 6;
            for col in 0..(cols - 1) {
                if col + ig < cols {
                    row.swap(col, col + ig);
                }

                if col + ig == cols - 1 {
                    ig = 2;
                }
                if col == 3 {
                    break;
                }
            }
        }

        Some(matrix)
    } else {
        eprintln!("Ошибка чтения файла!");
        None
    }
}

fn print_matrix_in_hex(matrix: &Vec<Vec<u8>>) {
    // Выводим матрицу в 16-ричной системе исчисления
    println!("Матрица 1 (16-ричная):");
    for row in matrix {
        for &byte in row.iter() {
            print!("{:X} ", byte);
        }
        println!(); // Переход на новую строку
    }
}

fn print_matrix_in_binary(matrix: &Vec<Vec<u8>>) {
    // Выводим матрицу в двоичной системе исчисления
    println!("Матрица 1 (2-ичная):");
    for row in matrix {
        for &byte in row.iter() {
            print!("{:04b} ", byte);
        }
        println!(); // Переход на новую строку
    }
}
fn klyuch() {
    // Verilen değerleri içeren vektörü oluştur
    let values: Vec<&str> = vec![
        "ED3E868D", "3EF0AB79", "C68B0175", "CEACE980",
        "E2CF119F", "277D712A", "F99E33A7", "9DCA77FA",
        "39B9F8F4", "8AFF1045", "D23BBE59", "E4CF5748",
        "3878AAEB", "6D4E2812", "FB7D36A8", "7E43433E"
    ];

    // Vektörü matrise dönüştür
    let rows = values.len();
    let cols = values[0].len() / 2; // Her bir değer iki haneli hexadecimal sayı olduğu için

    let mut matrix: Vec<Vec<u8>> = Vec::with_capacity(rows);

    for row in values.iter() {
        let row_values: Vec<u8> = row.chars().collect::<Vec<char>>().chunks(2)
            .flat_map(|hex_pair| {
                let hex_str: String = hex_pair.into_iter().collect();
                let hex_value = u8::from_str_radix(&hex_str, 16).expect("Invalid hexadecimal value");
                vec![(hex_value >> 4) & 0xF, hex_value & 0xF] // 4-bit olarak al
            })
            .collect();
        matrix.push(row_values);
    }

    // Oluşturulan matrisi ekrana yazdır
    println!("Matris 1 (4-bit):");
    for row in matrix.iter() {
        for &nybble in row.iter() {
            print!("{:X} ", nybble);
        }
        println!();
    }    let values: Vec<&str> = vec![
        "ED3E868D", "3EF0AB79", "C68B0175", "CEACE980",
        "E2CF119F", "277D712A", "F99E33A7", "9DCA77FA",
        "39B9F8F4", "8AFF1045", "D23BBE59", "E4CF5748",
        "3878AAEB", "6D4E2812", "FB7D36A8", "7E43433E"
    ];/* */

    // Vektörü matrise dönüştür
    let rows = values.len();
    let cols = values[0].len() / 2; // Her bir değer iki haneli hexadecimal sayı olduğu için

    let mut matrix: Vec<Vec<u8>> = Vec::with_capacity(rows);

    for row in values.iter() {
        let row_values: Vec<u8> = row.chars().collect::<Vec<char>>().chunks(2)
            .flat_map(|hex_pair| {
                let hex_str: String = hex_pair.into_iter().collect();
                let hex_value = u8::from_str_radix(&hex_str, 16).expect("Invalid hexadecimal value");
                vec![(hex_value >> 4) & 0xF, hex_value & 0xF] // 4-bit olarak al
            })
            .collect();
        matrix.push(row_values);
    }
    print_matrix_in_binary(&matrix);
}


fn main() {
    let rows = 16; // Örnek olarak 8 satır
    let cols = 8; // Örnek olarak 8 sütun

    // Dosyadan okuma işlemini gerçekleştir
    if let Some(matrix1) = read_file("C:/Users/begah/OneDrive/Masaüstü/rust_project/la2_hadzhyew/src/gen.bin", rows, cols) {
        // 16-rih ve 2-ich matrisleri yazdır
        print_matrix_in_hex(&matrix1);
        print_matrix_in_binary(&matrix1);
    }

    // İkinci dosyadan okuma işlemini gerçekleştir
    if let Some(matrix2) = read_file("C:/Users/begah/OneDrive/Masaüstü/rust_project/la2_hadzhyew/src/gen321.bin", 8, 8) {
        // 16-rih ve 2-ich matrisleri yazdır
        print_matrix_in_hex(&matrix2);
        print_matrix_in_binary(&matrix2);
    }
    klyuch();
}
