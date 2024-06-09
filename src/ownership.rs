// ownership can work in heap variable not stack variable
pub fn call_stack() {
    let mut _a: String = String::from("hey, this is first");
    let mut1 = &mut _a;
    println!("defore unused{}", mut1);
    mut_refrence(&mut _a);
    mut2_ref(&mut _a);
    println!("{}", _a); // there have not tranfer ownership only refrence this memory address
}

// #[allow(dead_code)]
// fn main() {
//     let _a = String::from("hey");
//     print!("{}", _a);
//     let _b: String = _a;
//     //println!("{}",_a)// borrow of moved value: `_a`
//     // mean rihana(data) that have a new boyfriend(variable) that is _b
//     // rihana(data) says i set only one boyfriend
//     // first is _a then rihana change the boyfriend the is _b
//     // owner of rihana is _b
//     // cannot access the data _a because in heap there is no data to point _a
// }

//                  Borrow
// if rihana have multiple borrower that rihan says i dont hanky panky these guy
// rihana says if i have only one owner or borrower than should we hanky panky
// rihana says if borrower die i will not die
// but if owner die i will definitly die

// only one borrower can hanky panky with rihana(data) in current scope
fn mut_refrence(ref1: &mut String) -> &mut String {
    let _c = ref1;
    _c.push_str("this is add something");
    return _c;
}

fn mut2_ref(ref2: &mut String) {
    ref2.push_str("this is world");
    // println!("{}",ref2)
}
