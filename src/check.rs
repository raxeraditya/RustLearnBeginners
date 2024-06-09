pub fn call_stack(){
    let a = 3;
    println!("{}",a);
    hey(a);
    hey1(a);
    let b = a;
    println!("{}",b);
    println!("{}",a);
}

fn hey(data:u32){
    let data = data;
    println!("{}",data);
}

fn hey1(data:u32){
    let data = data;
    println!("{}",data);
}