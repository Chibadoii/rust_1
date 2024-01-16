use std::fs;


fn main() {
    let file_path: String = String :: from ("text.txt");
    // Считываем файл
    let contents = fs::read_to_string(file_path)
        .expect("Err");

    //Разделяем и заносим в vec 
    let collection = contents.split("\n").collect::<Vec<&str>>();
    // вектор вариантов подстановки
    let replacements =vec![
        ("one","1"),("two","2"),("three","3"),("four","4"),("five","5"), ("six","6"),("seven","7"),("eight", "9"), ("zero","0")
    ];
    //println!("{:?}", collection);
    //let mut final_num = vec![];
    let mut words_in_string = vec![];// хранение слов из втроки
    //for item in &collection{
         for (word,digit) in &replacements  {
              if collection[0].contains(word) || collection[0].contains(digit) {

                        words_in_string.push(digit); // Добавляем стринг в вектор
                        println!("Число успешно добавлен в вектор");
                    }
                }
    for i in words_in_string{
        println!("{}", i);
    }

            /*final_num.push(words_in_string.get(words_in_string[0]));
            final_num.push(words_in_string.get(words_in_string.len() - 1));
            println!("{:?}", final_num)*/
            /*final_num.push(words_in_string[0]);
            final_num.push(words_in_string.len() - 1);*/

            }




