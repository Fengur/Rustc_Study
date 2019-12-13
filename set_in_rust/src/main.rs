
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

//Vector===================================
    let v:Vec<i32> = Vec::new();

    let v1 = vec![1,2,3];
    
    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);


    let v3 = vec![1,2,3,4,5];
    let third:&i32 = &v3[2];
    println!("第三个是 {}",third);


    match v3.get(3){
        Some(fourth) => println!("第四个是 {}",fourth),
        None => println!("没有第四个元素"),
    }

    let third = v3.get(2);
    let third_value = &v3[2];

    // panic 越界
    // let no_value = &v3[100];

    let first = &v3[0];

    // 这时push会崩溃 （ cannot borrow as mutable）
    // 因为在 vector 的结尾增加新元素时，在没有足够空间将所有所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。
    // v3.push(6);

    let runlist = vec!["草","你","妈"];

    for i in &runlist {
        println!("{}",i);
    }

    let mut testv = vec![44,55,66];

    for i in &mut testv {
        *i += 1;
    }

    println!("{:?}",testv);

    let heros = vec![
        Hero::Age(3),
        Hero::Weight(150.2),
        Hero::Name(String::from("亚索")),
    ];

    println!("{:#?}",heros);

    let mut heros_mut = heros;
    heros_mut.pop();
    println!("{:#?}",heros_mut);

//String===================================
//Todo 重新看一遍 太难懂

// UTF8编码

    let mut s_new = String::new();
    // let data_str = data_tmp.to_string();
    let data_str1 = "fuck you shabi".to_string();
    let data_str2 = String::from("fuck you shabi");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");


    let mut s1 = String::from("fuck");
    s1.push_str("you");
    println!("s1 value {}",s1);


//HashMap===================================
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);


    let teams = vec![String::from("A"),String::from("B")];
    let points = vec![100,50];

    let scores_new:HashMap<_,_> = teams.iter().zip(points.iter()).collect();

    println!("{:#?}",scores_new);


    // let tmp_name = String::from("fuck");
    let tmp_value = String::from("value");
    let mut dict_tmp = HashMap::new();
    dict_tmp.insert(hello, tmp_value);

    // tmp_value 所有权已经变动  所以不能再访问 执行下面代码会报错
    // println!("{}",tmp_value);

    // 访问
    let mut meinvs = HashMap::new();
    meinvs.insert(String::from("linzhiling"), 40);
    meinvs.insert(String::from("yangmi"), 30);

    let zhilingAge = String::from("linzhiling");
    let ageValue = meinvs.get(&zhilingAge);

    println!("林志玲多大啦 {:?}",ageValue);

    descofdict(&meinvs);

    meinvs.insert(String::from("linzhiling"), 99);
    descofdict(&meinvs);

    // 只在键没有值时插入
    meinvs.entry(String::from("linzhiling")).or_insert(18);
    meinvs.entry(String::from("gulinazha")).or_insert(28);
    descofdict(&meinvs);

    //根据旧值更新新值
    let testStr = "wo cao ni ma,wo cao";
    let mut testMap = HashMap::new();
    
    for word in testStr.split_whitespace(){
        let count  = testMap.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}",testMap);

}

fn descofdict(dict:&HashMap<String,i32>){
    println!("\nReady to log all keyvalues");
    for(key,value) in dict{
        println!("{}:{}",key,value);
    }   
}


#[derive(Debug)]
enum Hero{
    Age(i32),
    Weight(f64),
    Name(String),
}