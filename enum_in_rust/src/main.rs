





enum IpAddrKind{
    V4,
    V6,
}

enum IpAddrKindNew{
    V4(String),
    V6(String),
}

fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr{
        kind:IpAddrKind::V4,
        address:String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
        kind:IpAddrKind::V6,
        address:String::from("::1"),
    };

    let homeNew = IpAddrKindNew::V4(String::from("127.0.1.0"));
    let loopbackNew = IpAddrKindNew::V6(String::from("::1"));

    let m = Message::Wirte(String::from("fuck you"));
    m.call();

    let some_num = Some(5);
    let some_str = Some("fuckyou");
    // 必须制定空值类型 因为编译器无法自动推导
    let blank_num:Option<i32> = None;    

    let fuck = plus_one(Some(5));
    println!("value fuck is {:#?}",fuck);
    let fuck1 = plus_one(None);
    println!("value fuck1 is {:#?}",fuck1);

    let some_u8_value = 0u8;

    match some_u8_value {
        1 =>{
            println!("1");
        },
        2 =>{
            println!("2");
        },
        // _ 通配符 相当于其他语言中的default
        _ =>{
            println!("whaterver fuck you!!!!");
        },
    }
    let some_u8_value_new = Some(0u8);

    match some_u8_value_new {
        Some(3) => println!("33333"),
        _ =>(),
    }
    // 简单控制流

    if let Some(3) = some_u8_value_new {
        println!("3 fuckyou!!!!");
    }

    let coin = CoinNew::Penny;
    let mut count = 0;
    // 1
    match coin {
        CoinNew::Quarter(state) =>{
            println!("fuck the quarter from {:?}!",state);
        },
        _ => count += 1,
    }

    println!("count is {:#?}",count);

    count = 0;
    // 2
    let coin_1 = CoinNew::Quarter(UsState::Henan);
    if let CoinNew::Quarter(state) = coin_1 {
        println!("fuck the quarter form {:?}!",state);
    }else{
        count += 1;
    }
    println!("count is {:#?}",count);
}

struct IpAddr {
    kind:IpAddrKind,
    address:String,
}


struct Ipv4Addr {
}

struct Ipv6Addr{

}


enum IpAddrNew2{
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message{
    Quit,
    Move {x:i32,y:i32},
    Wirte(String),
    ChangeColor(i32,i32,i32),
}

impl Message{
    fn call(&self){

    }
}
fn route(ip_type:IpAddrKind){

}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[derive(Debug)]
enum UsState{
    Shandong,
    Henan,
}

enum CoinNew {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin:Coin) ->u32{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_cents_new(coin:CoinNew) ->u32{
    match coin{
        CoinNew::Penny => 1,
        CoinNew::Nickel => 5,
        CoinNew::Dime => 10,
        CoinNew::Quarter(state) =>{
            println!("State quarter form {:?}!",state);
            25
        },
    }
}

fn plus_one(x:Option<i32>) ->Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

