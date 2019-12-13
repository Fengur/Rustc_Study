

// 输入输出
use std::io;
// 比较
use std::cmp::Ordering;

// 你不可能凭空就知道应该 use 哪个 trait 以及该从 crate 中调用哪个方法。crate 的使用说明位于其文档中。
// Cargo 有一个很棒的功能是：运行 cargo doc --open 命令来构建所有本地依赖提供的文档，并在浏览器中打开。
// 例如，假设你对 rand crate 中的其他功能感兴趣，你可以运行 cargo doc --open 并点击左侧导航栏中的 rand

use rand::Rng;


// fn main() {
//     println!("Guess the number!");

//     let secret_number = rand::thread_rng().gen_range(1, 101);

//     loop{
//         println!("Please input your guess.");

//         let mut guess = String::new();

//         io::stdin().read_line(&mut guess)
//             .expect("Failed to read line");

//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         println!("You guessed: {}", guess);

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }
// }


fn main() {







    // Part 1
    // println!("Guess the number!");

    // let secret_number = rand::thread_rng().gen_range(1,101);

    // println!("The secret number is:{}",secret_number);


    // loop{
    //     println!("Please input your guess.");
    //     // mut 可变
    //     let mut guess = String::new();
    //     io::stdin().read_line(&mut guess)
    //         .expect("Faid to read line");

    //     // trim() 去除空格 parse 匹配
    //     let guess:u32 = match guess.trim().parse(){
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     println!("Your guessed:{}",guess);

    //     match guess.cmp(&secret_number){
    //         Ordering::Less => println!("太小了"),
    //         Ordering::Greater => println!("太大了"),
    //         Ordering::Equal => {
    //             println!("你赢了");
    //             break;
    //         }
    //     };
    // }
    
    // Part 2
    
    let s = String::from("Fuck");
    
    take_owership(s);

    let x = 5;
    make_copy(x);


    let mut s = String::from("fuck you");
    
    let word = find_first_word(&s);

    println!("First word index 0-{}",word);

    s.clear();
    
    let mut fuck  = String::from("fuck your mother");
    let first_word = slice_find_first_word(&fuck);
    println!("第一个字是什么 {}",first_word);


}


fn take_owership(some_string:String){
    println!("{}",some_string);
}

fn make_copy(some_integer:i32){
    println!("{}",some_integer);
}

// 使用并返回s 这样保证s在调用函数后还可以使用
fn caculate_len(s:String) ->(String,usize){
    let length = s.len();
    (s,length)
}

fn refer_caculete_len(s:&String)->usize{
    let length = s.len();
    length
}

fn find_first_word(s:&String) ->usize{
    let bytes = s.as_bytes();

    for(i,&item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn slice_find_first_word(s:&String)->&str{
    let bytes = s.as_bytes();
    for(i,&item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

