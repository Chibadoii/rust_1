use std::fs;

fn main() {
    let file_path: String = String :: from ("text.txt");
    // Считываем файл
    let contents = fs::read_to_string(file_path)
        .expect("Err");

    //Разделяем и заносим в vec 
    let collection = contents.split("\n").collect::<Vec<&str>>();

    // инициализация числового и строкового массива
    let num_arr = ["1","2","3","4","5","6","7","8","9","0"]; 
    let str_arr = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    
    let mut vremennaya = String::new();//Временная переменная для хранения символов в Стринге
    for i in 0..collection.len(){
        let mut var = collection[i].trim().to_string();//первая строка без пробелов
        let char_collection = var.split("").collect::<Vec<&str>>();//Посимвольно разделенная строка в векторе
        for c in 0..char_collection.len(){
            let mut chr = char_collection[c].to_string();//отдельный символ строки
                if chr.contains(num_arr[c]){
                    vremennaya = num_arr[c].to_string();
                    println!("Занесена первая цифровая часть");
                }
                else if chr.contains(str_arr[c]){
                    vremennaya = str_arr[c].to_string();
                    println!("Занесена первая буквенная часть");
                }else {
                    let b = c + 1;
                    chr.push_str(char_collection[b]);
                    println!("Добавлен симфол в тестируемую строку");
                }

            }
        }
        println!("{}", vremennaya);

    }