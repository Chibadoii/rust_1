use std::fs;


fn main() {
    let file_path: String = String::from("text.txt");
    // Считываем файл
    let contents = fs::read_to_string(file_path)
        .expect("Err");

    //Разделяем и заносим в vec 
    let collection = contents.split("\n").collect::<Vec<&str>>();
    // вектор вариантов подстановки
    let replacements = vec![
        ("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9"), ("zero", "0")
    ];

    let mut combo_string = String::new();
    let mut couples_num = vec![];
    for item1 in &collection {
        let mut cal_str: String = item1.parse().expect("Err str in str");
        let mut rev_call_str = cal_str.chars().rev().collect::<Vec<_>>();
        let mut rev_call_str_str = cal_str.chars().rev().collect::<String>();
        let mut str_for_push = String::new();
        for i in &rev_call_str{
            str_for_push.push(*i);
            for (word, digit) in &replacements {
                let rev_word = word.chars().rev().collect::<String>();

                if str_for_push.contains(&rev_word) {
                    rev_call_str_str = rev_call_str_str.replace(&rev_word, digit).to_string().clone();//Переводим слова в числа
                    //println!("{:?}", &rev_call_str);
                }
            }
        }
        rev_call_str_str = rev_call_str_str.chars().rev().collect::<String>();


        let num_char:String = rev_call_str_str.chars().filter(|c| c.is_digit(10)).collect();//Выбираем числа

        let f_char = &num_char.chars().next().expect("Err f ch");
        //dbg!("{}", f_char);
        let l_char = &num_char.chars().last().expect("Err l ch");

        &mut combo_string.push(**&f_char);
        &mut combo_string.push(**&l_char);
        //combo_string = format!("{}{}", f_char, l_char);
        //dbg!("{}", &combo_string);


        if let Ok(parsed_number) = combo_string.parse::<i64>() {
           &mut couples_num.push(parsed_number);
           &mut combo_string.clear();
        } else {
            println!("Err to str");
        }
    }

    for (index, value) in collection.iter().enumerate() {
        print!("  --{}--{}",&index, value);
        println!("{}", &couples_num[index])
    }
    //  for (index, value) in collection.iter().enumerate() {// такой вариант выглядит лучше, но в выводе одни числа накладываются на другие
    //     println!("Str {}  —  {}  — {}", index, value, &couples_num[index]);
    // }
    let mut summ:i64 = 0;
    for itm in &couples_num{
        summ += itm;
    }
    //println!("{}", &summ);
    }





