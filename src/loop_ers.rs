pub fn str_ing(first_name:&str)->String{
    println!("hey");
    let a:String = format!("{0}",first_name);
    return  a;
}

pub fn for_loop()->[u8;5]{
    let a:[u8;5] = [3,4,6,75,5];
    for i in a{
        println!("{}",i)
    }
    return a;
}

pub fn loop_cli(value:u32)->u32{
    while value == 5 {
        if value as u32 == 5 {
            println!("hey loop work on loop keyword");
            break;
        }
    }
    // loop is special type keyword that 
    // not set start condition only set 
    // break condition when loop is break
    loop {
        println!("hey");
        if value == 5 {
            println!("hey loop is loop {}",value);
            break;
        }
    }
    return value;
}

// every variable is immutable in rust 
// by default 
// if you change the value of variable 
// before name of variable after let keyword using mut keyword
// while create a function if accept a data in parameter 
// mendetory to set a name with datatype
// while calling a fn pass there a same datatype value
