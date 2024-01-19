use std::fs;


fn main() {
    let file_path: String = String::from("text.txt");
    // Считываем файл
    let contents = fs::read_to_string(file_path)
        .expect("Err");

    //Разделяем и заносим в vec 
    let collection = contents.split("\n").collect::<Vec<&str>>();
    //let collection = collection1.split("\r").collect::<Vec<&str>>();
    // вектор вариантов подстановки
    let replacements = vec![
        ("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "9"), ("zero", "0")
    ];

    let mut combo_string = String::new();
    let mut couples_num = vec![];
    for mut item1 in &collection {
        let mut cal_str: String = item1.parse().expect("Err str in str");
        cal_str = cal_str.trim().to_string();// избавляемся от пробелов
        for (word, digit) in &replacements {

            if cal_str.contains(word) {
                &mut cal_str.replace(word, digit);
                //println!("{}", &item1);
            }
            //println!("{}", &combo_string.parse::<i64>().unwrap());
            //couples_num.push(combo_string).parse::<i64>().unwrap();
        }
        let mut f_char = String::new();
        for c in cal_str.chars() {
            let first_char:char = c;
            if first_char != ' '{
                &mut f_char.push(first_char);
            }else {
                println!("Первый символ пустой")
            };
            println!("{}", c);
        }

        let mut l_char;
        //for b in cal_str.len()
        loop{
            let mut char_calc = ' ';
            let mut b: usize = 0;
            let last_char = &cal_str.chars().rev().nth(b);
            if last_char != char_calc {
               &mut l_char.push(last_char);
                break;
            }else {
                println!("Последний символ пустой")
            };
            b+=1;
            println!("{}", b);
        }
        /*let f_char = &cal_str.chars().next().expect("Err f ch");
        dbg!("{}",);
        let l_char = &cal_str.chars().last().expect("Err l ch");*/


        combo_string = format!("{}{}", f_char, l_char);
    }
    if let Ok(parsed_number) = combo_string.parse::<i64>() {
        couples_num.push(parsed_number);
    } else {
        println!("Ошибка: Невозможно преобразовать строку в число");
    }

    println!("{:?}", &couples_num);
    let mut summ:i64 = 0;
    for item in &couples_num{
        summ += item;
    }
    println!("{}", summ);
}




