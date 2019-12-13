
/* 
包（Packages）是 Cargo 的一个功能，它允许你构建、测试和分享 crate。
Crates 是一个模块的树形结构，它形成了库或二进制项目。
模块（Modules）和 use 关键字允许你控制作用域和路径的私有性。
路径（path）是一个命名例如结构体、函数或模块等项的方式
*/



mod sound {
    pub mod instrument {
        pub fn clarient(){
            println!("fuck you!!!");
        }
    }
}

use crate::sound::instrument;

// 不推荐这种直接引用到根对象的用法
// use std::collections::hash_map;
use std::collections;

// 不同父模块的一种类型 可能会有同个方法或变量 会导致贬义无法粪便
use std::fmt::Result;
// use std::io;

use std::io::Result as IoResult;

// 嵌套使用
// use std::{cmp::Ordering,io}

// use std::io;
// use std::io::Read;
// use std::io::Write;

// 合并
use std::io::{self,Read,Write};

// fn fuc1() -> fmt::Result{
//     Ok(());
// }

// // 会报错
// fn fuc2() -> io::Result{
//     Ok(());
// }

// Globe 运算符  *

// 指定路径下全部公有项目
use std::collections::*;
fn fuc1() -> Result{
    Ok(())
}

fn fuc2() -> IoResult<()>{
    Ok(())
}

// 导入外部mod文件
mod gailun;
fn main() {
    println!("Hello, world!");
    // 绝对路径
    crate::sound::instrument::clarient();

    // 相对路径
    sound::instrument::clarient();
    maren::fuckyou();
    maren::ganniniang::fuck();
    
    let mut xiazi = lol::Mangseng::init("liqing");
    xiazi.name = String::from("xiazi");
    println!("{} 是瞎子！！！",xiazi.name);

    // 下面这句会报错 因为shanghai属性是私有的 field `shanghai` of struct `lol::Mangseng` is private
    // println!("{} 是瞎子！！！",xiazi.shanghai);

    // 枚举只要公开则全部枚举都可用
    let adc1 = lol::Adc::ez;
    let adc2 = lol::Adc::xiaopao;

    // 引用路径 推荐用法保留上一级父级目录调用
    instrument::clarient();
    guoma::nidaye();

    let mut map = collections::HashMap::new();
    map.insert("gan", 45);

    // pub use示例
    maren::ganniniang::fuck();
    gailun::jineng::dazhao();

}



mod maren{
    pub fn fuckyou(){
        super::shit();
    }

    pub mod ganniniang{
        pub fn fuck(){
            super::caonima();
        }
    }

    fn caonima(){
        println!("什么几把瓦木偶");
    }
}

mod guoma{
    pub use crate::maren::ganniniang;
    pub fn nidaye(){
        ganniniang::fuck();
    }
}


fn shit(){
    println!("草拟吗");
}


mod lol{
    pub struct Mangseng{
        pub name:String,
        shanghai:i32,
    }

    impl Mangseng{
        pub fn init(name:&str) -> Mangseng{
            Mangseng{
                name:String::from(name),
                shanghai:50,
            }
        }
    }

    pub enum Adc{
        ez,
        xiaopao,
    }
}

