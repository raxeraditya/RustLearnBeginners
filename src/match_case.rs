pub fn matchcase(_value:u32)->u32{
    let a:u32 = _value;
    match a {
        3 | 4 =>{println!("this is 3")},
        5..=30 if a == 7=>{println!("this is rage of 5 to 30 or 7")},
        // there is ..= match 5 to 30 add or match 30
        5..=30 if a == 8=>{println!("this is rage of 5 to 30 or 8")},
        5.. => {println!("this is 5")},
        _ =>{println!("hey")}
    };
    return a;
}

pub fn test_match_array()->[u32;4]{
    let a:[u32;4] = [444,3434,565,4545];
    match a[1..=3]{
        [3434,565]=>{println!("this is 3434,565")},
        [444,3434,565]=>{println!("this is 444,3434,565")}
        [3434,..]=>{println!("this remain")}
        _=>{println!("this is default")},
    };
    return a;
}