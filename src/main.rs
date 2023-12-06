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

fn print_matrix_in_hex(matrix: &Vec<Vec<u8>>,iy:usize) {
    // Выводим матрицу в 16-ричной системе исчисления
    println!("Матрица {} (16-ричная):", iy);
    for row in matrix {
        for &byte in row.iter() {
            print!("{:X} ", byte);
        }
        println!(); // Переход на новую строку
    }
}

fn print_matrix_in_binary(matrix: &Vec<Vec<u8>>,iy:usize) {
    
   
    // Выводим матрицу в двоичной системе исчисления
    println!("Матрица {} (2-ичная):", iy);
    for row in matrix {
        for &byte in row.iter() {
            print!("{:04b} ", byte);
        }
        println!(); // Переход на новую строку
    }
}

/*fn klyuch() {
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
    }
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
    print_matrix_in_binary(&matrix, iy);
}*/// Ваша функция для бинарного сложения без обрезки нулей
fn binary_addition(row1: &str, row2: &str) -> String {
    let mut carry = 0;
    let mut result = String::new();

    for (bit1, bit2) in row1.chars().rev().zip(row2.chars().rev()) {
        let digit1 = bit1.to_digit(2).unwrap();
        let digit2 = bit2.to_digit(2).unwrap();
        let sum = digit1 + digit2 + carry;
        carry = sum / 2;
        result.push_str(&(sum % 2).to_string());
    }

    if carry > 0 {
        result.push_str(&carry.to_string());
    }

    result.chars().rev().collect()
}
// Новая функция, которая принимает три матрицы и выводит их в двоичной системе исчисления
fn process_matrices(matrix1: &Vec<Vec<u8>>, matrix2: &Vec<Vec<u8>>, matrix3: &Vec<Vec<u8>>) {
    for i in 0..16
     {
        let row1 = matrix1[i]
            .iter()
            .map(|&num| format!("{:04b}", num))
            .collect::<Vec<_>>()
            .iter()
            .map(|s| s.as_str()) // Convert String to &str
            .collect::<Vec<_>>()
            .join("");

        let row2 = matrix2[i]
            .iter()
            .map(|&num| format!("{:04b}", num))
            .collect::<Vec<_>>()
            .iter()
            .map(|s| s.as_str()) // Convert String to &str
            .collect::<Vec<_>>()
            .join("");

        let binary_sum = binary_addition(&row1, &row2);

        // Разбиваем двоичную сумму на 8 столбцов
     // Разбиваем двоичную сумму на 8 столбцов
// Разбиваем двоичную сумму на 8 столбцов
    // Разбиваем двоичную сумму на 8 столбцов
        let columns: Vec<String> = binary_sum
        .chars()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect();

        // Convert Vec<String> to Vec<&str>
// Convert Vec<String> to Vec<&str>, skipping excess elements
// Convert Vec<String> to Vec<&str> and exclude the first element
// Convert Vec<String> to Vec<&str> and exclude the first element
let columns_str: Vec<&str> = columns.iter().skip(1).map(|s| s.as_str()).collect();

// Join the columns into a single string
let mut binary_sum_str = columns_str.join("");

// If there are more than 32 elements, keep the last 32 elements
// If there are more than 32 elements, keep the last 32 elements
if binary_sum_str.len() > 32 {
    binary_sum_str = binary_sum_str.chars().skip(binary_sum_str.len() - 32).collect();
}

// If there are less than 32 elements, pad with leading zeros
while binary_sum_str.len() < 32 {
    binary_sum_str.insert(0, '0');
}

// Выводим результат в одной строке с разбиением на 8 столбцов
print!("Binary Sum (Row {}): [", i + 1);

// Iterate over the characters and print in groups of 4
for (index, char) in binary_sum_str.chars().enumerate() {
    if index > 0 && index % 4 == 0 {
        print!(" ");
    }
    print!("{}", char);
}

println!("]");


}}

fn main() {
    let rows = 16;
    let cols = 8;
let iy=1;
    // Чтение первого файла и создание матрицы
    if let Some(mut matrix1) = read_file("D:/git/la2_hadzhyew/src/gen.bin", rows, cols) {
        // Вывод матрицы1 в 16-ричной и 2-ичной системе исчисления
        print_matrix_in_hex(&matrix1, iy);
        print_matrix_in_binary(&matrix1, iy);
    
        let num_rows = matrix1.len();
        let num_cols = matrix1[0].len();
    
        // Создаем новую матрицу с транспонированными строками исходной матрицы
        let transposed_matrix: Vec<Vec<u8>> = (0..num_rows)
            .map(|row_index| vec![matrix1[row_index][0]])
            .collect();
    
        // Выводим результат
     
    
        // Overwrite matrix1 with the transposed_matrix
        for i in 0..num_rows {
            matrix1[i][0] = transposed_matrix[i][0];
        }
    
        // Выводим обновленную матрицу1
   let iy=2;
        // Чтение второго файла и создание матрицы
        if let Some(matrix2) = read_file("D:/git/la2_hadzhyew/src/gen321.bin", 8, cols) {
            print_matrix_in_hex(&matrix2,iy);
            print_matrix_in_binary(&matrix2,iy);

            let num_rows = matrix2.len();
            let num_cols = matrix2[0].len();
            let trans_matrix: Vec<Vec<u8>> = (0..num_rows)
            .map(|row_index| {
                (0..num_cols)
                    .map(|col_index| {
                        matrix2[row_index][col_index]
                    })
                    .collect()
            })
            .collect();
    
        // Выводим результат
        for row in trans_matrix.iter() {
            let binary_row: String = row.iter().map(|&num| format!("{:04b}", num)).collect();
            println!("[{}]", binary_row);
        }



            // Создание третьей матрицы (вызов функции klyuch)
            let values: Vec<&str> = vec![
                "ED3E868D", "3EF0AB79", "C68B0175", "CEACE980",
                "E2CF119F", "277D712A", "F99E33A7", "9DCA77FA",
                "39B9F8F4", "8AFF1045", "D23BBE59", "E4CF5748",
                "3878AAEB", "6D4E2812", "FB7D36A8", "7E43433E"
            ];
            let rows = values.len();
            let cols = values[0].len() / 2;
            let mut matrix3: Vec<Vec<u8>> = Vec::with_capacity(rows);
            for row in values.iter() {
                let row_values: Vec<u8> = row.chars().collect::<Vec<char>>().chunks(2)
                    .flat_map(|hex_pair| {
                        let hex_str: String = hex_pair.into_iter().collect();
                        let hex_value = u8::from_str_radix(&hex_str, 16).expect("Invalid hexadecimal value");
                        vec![(hex_value >> 4) & 0xF, hex_value & 0xF]
                    })
                    .collect();
                matrix3.push(row_values);
            }

            // Вывод матриц2 в 16-ричной и 2-ичной системе исчисления
            let iy=3;
            // Вывод матриц3 в 16-ричной и 2-ичной системе исчисления
            print_matrix_in_hex(&matrix3,iy);
            print_matrix_in_binary(&matrix3,iy);

            // Вывод обработанных матриц в двоичной системе исчисления
            process_matrices(&matrix1, &trans_matrix, &matrix3);
        }
    }
}

