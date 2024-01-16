use std::fs;
use std::io::BufRead;


fn main() {
    let file_path: String = String::from("text.txt");
    // Считываем файл
    let contents = fs::read_to_string(file_path)
        .expect("Err");

    //Разделяем и заносим в vec 
    let collection = contents.split("\n").collect::<Vec<&str>>();
    // вектор вариантов подстановки
    let replacements = vec![
        ("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "9"), ("zero", "0")
    ];
    //println!("{:?}", collection);


    //let mut final_num = vec![];
    let mut words_in_string = vec![];// хранение слов из втроки

    for item in &collection {
        for i in 0..item.len() {
            let mut char_string = String::new();

            let mut var = &item.trim().to_string();//первая строка без пробелов
            let char_collection = &var.split("").collect::<Vec<&str>>();//Разделение строки посимвольно
            char_string.push_str(&char_collection[i]);// вывод символов в переменную по одному
            for (word, digit) in &replacements {

                if char_string.contains(word) || char_string.contains(digit) {
                    words_in_string.push(digit); // Добавляем стринг в вектор
                    println!("Число успешно добавлен в вектор");
                }else { println!("Err") };


            }
            let mut rev_char_string: String = var.chars().rev().collect();
            println!("{}", rev_char_string);
            let rev_char_collection = rev_char_string.split("").collect::<Vec<&str>>();
            rev_char_string.push_str(&rev_char_collection[i]);
            for il in 0..rev_char_string.len(){
                for (word, digit) in &replacements {
                    if rev_char_string.contains(word) || rev_char_string.contains(digit) {
                        words_in_string.push(digit); // Добавляем стринг в вектор//////////////////////////////////////////////
                        println!("Число успешно добавлен в вектор");
                    }else { println!("Err") };

                }


            }
            for i in &words_in_string {
                println!("{:?}", i);
            }


        }

    }
}




