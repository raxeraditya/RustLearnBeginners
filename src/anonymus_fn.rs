pub fn anoy(){
    let add = |a:u32,b:u32| println!("hey this anonymus fnc :{}",a+b);
    add(4,3);
}
#[allow(dead_code)]
pub fn anoy_bin()->u32{
    let add_sub = |a:u32,b:u32| {
        let s = a+b;
        let n = b-a;
        println!("add this ,{0}, sub this {1}",b,a);
        return b;
    };
    add_sub(3,4)
}