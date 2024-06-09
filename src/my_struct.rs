use std::cell::Cell;
#[derive(Debug)]
#[allow(dead_code)]
enum Name {
    aditya,
    arpan,
    raja,
    kishan,
}
// #[derive(Debug)]
#[derive(Debug)]
#[allow(dead_code)]
struct User {
    name: Name,
    email: String,
    username: String,
    password: u32,
}

fn struct_method() -> User {
    let mut user1: User = User {
        name: Name::kishan,
        email: "aditya@gmail.com".to_string(),
        username: "ak".to_string(),
        password: 123,
    };
    user1.email = "aditya1506k".to_string();
    println!("{:?}", user1);
    return user1;
}

// derive debug means if you print a result of printline macro
// the will we show formatter error
// use after enum or struct derive debug
#[derive(Debug)]
struct TuppleStruct(String, u32, String); // tupple struct only pass datatype
                                          // not pass the name with datatype

fn struct_tupple() -> TuppleStruct {
    let mut Tupple = TuppleStruct("hey".to_string(), 32, "bey".to_string());
    Tupple.0 = "hey1".to_string();
    println!("{:?}", Tupple);
    return Tupple;
}

fn direct_type_string() {
    let a: String = "ehy".to_string();
    println!("{}", a)
}

struct Rectangle {
    width: u32,
    height: u32,
}
fn new_fn() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    // 126 page
    println!("{:?}", area(&rect1));
    //rectangle ,whose type is an immutable borrow of a struct Rectangle instance
    // we want to borrow the struct rather than take ownership of it. This way, main
    // retains its ownership and can continue using rect1 , which is the reason we use the & in
    // the function signature and where we call the function.
}
fn area(rectangle: &Rectangle) -> u32 {
    let _a = rectangle.width * rectangle.height;
    println!("{}", _a);
    return _a;
}

// impl block create a method to allow atach a user defined type

#[allow(dead_code)]
#[derive(Debug)]
enum VehicleColor {
    red,
    green,
    yellow,
}
#[derive(Debug)]
struct Vehicle {
    manufacturer: String,
    model: String,
    year: u32,
    color: VehicleColor,
}

impl Vehicle {
    fn paint(&mut self, new_color: VehicleColor) {
        self.color = new_color;
    }
}

fn new_vehicle() {
    let mut _a = Vehicle {
        manufacturer: "aditya".to_string(),
        model: "ak".to_string(),
        year: 30,
        color: VehicleColor::yellow,
    };
    println!("{:?}", _a);
    _a.paint(VehicleColor::red);
    println!("{:?}", _a);
}

pub fn call_stack() {
    // struct_method();
    // struct_tupple();
    // direct_type_string();
    new_vehicle();
    // new_fn();
}
