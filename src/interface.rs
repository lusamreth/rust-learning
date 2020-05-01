use crate::structure::shop;
use std::collections::HashMap;
use std::fmt;
use std::io;
pub fn runner() {
    let mut running = 0;
    let breaker = "quit".to_string();
    let mut text_holder = "/".to_string();
    while &running <= &0 {
        let imp = input(&text_holder);

        if imp == breaker {
            running += 1;
            println!("application closed!")
        }

        let appshop = ["Inventory", "Accounting", "Shoppingcart"];

        for (i, app) in appshop.iter().enumerate() {
            if imp == String::from(appshop[i]) {
                match appshop[i] {
                    "Inventory" => {
                        text_holder.push_str("/Inventory");
                        let new_inventory = shop::Inventory {
                            items_list: HashMap::new(),
                        };
                    }
                    _ => (),
                }
            }
        }
    }
}
fn input(message: &'_ impl fmt::Display) -> String {
    println!("{}", message);
    let mut ret = String::new();
    io::stdin()
        .read_line(&mut ret)
        .expect("Failed to read from stdin");
    ret.trim().to_string()
}

// let acc_opt = vec!["insert","delete","find","update"];
// println!("Here are avialable operations:");
// fn dot_line(len:i32){
//     let mut count = 1;
//     while count < len {
//         print!("-");
//         count = count + 1;
//     }
//     println!("\n");
// }
// dot_line(50);
// let mut count = 1;
// for opt in acc_opt{
//     print!("{}.{}\n",count,opt);
//     count = count + 1;
// }
// dot_line(50);

// let new_inventory = shop::Inventory {
//     items_list: HashMap::new(),
// };
// let mut input_holder = String::new();
// let user_input = io::stdin().read_line(&mut input_holder);

// let item = new_inventory.list(input_holder);
// println!("this {:#?}", item);
// match user_input {
//     Ok(byte) => {
//         println!("byte read:{}", byte);
//         // match input_holder{
//         //      => {
//         //         println!("you have chosen insert op!");
//         //         let inserted_itm = new_inventory.list(input_holder);
//         //     }
//         // }
//         match item {
//             Ok(t) => print!("return item : {:#?}",t),
//             Err(e) => print!("Ops something went wrong during retrieval : \nmessage:'{}' ",e)
//         }
//     }
//     Err(e) => panic!("Opps something went wrong during input : {:#?}", e),
// }
