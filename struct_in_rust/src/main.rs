

#[derive(Debug)]

struct Rectangle {
    width:u32,
    height:u32,
}

impl Rectangle{
    fn area(&self) ->u32{
        self.width * self.height
    }

    fn can_hold(&self,other:&Rectangle) ->bool{
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    println!("Hello, world!");
    let width = 30;
    let height = 50;
    println!(
        "The area of the retangle is {} square pixels.",
        area(width, height)
    );
    

    let rect = (30,50);
    println!(
        "area is {}!!!",
        area1(rect)
    );

    let rect2 = Rectangle{width:30,height:50};

    println!(
        "rect2 is {:?}",rect2
    );

    println!(
        "rect2 is {:#?}",rect2
    );

    println!(
        "area2 is {}!!!",
        area2(&rect2)
    );

    println!(
        "area3 is {}突突突！！！",
        rect2.area()
    );
}


fn area(width:u32,height:u32) ->u32{
    width * height
}

fn area1(dimensions:(u32,u32)) ->u32{
    dimensions.0 * dimensions.1
}

fn area2(rect:&Rectangle) ->u32{
    rect.width * rect.height
}