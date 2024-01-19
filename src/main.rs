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


    let mut couples_num = vec![];
    for mut item1 in &collection {
        for (word, digit) in &replacements {
            let mut cal_str: String = item1.parse().expect("Err str in str");
            if cal_str.contains(word) {
                &mut cal_str.replace(word, digit);
                //println!("{}", &item1);
            }
            let mut combo_string = String::new();
            let f_char = &cal_str.chars().next().expect("Err f ch");
            let l_char = &cal_str.chars().last().expect("Err l ch");
            combo_string = format!("{} {}", f_char, l_char);


            //println!("{}", &combo_string.parse::<i64>().unwrap());
            couples_num.push(combo_string).parse::<i64>().unwrap();
        }
    }
    println!("{:?}", &couples_num);
}




