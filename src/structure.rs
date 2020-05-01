#![allow(dead_code)]
use rand::Rng;
// hashmap
pub fn run() -> f64 {
    struct Color(i32, i32, i32);
    let _white = Color(255, 255, 255);
    // println!("{:?}",_white);
    coordinate_plane();
    let point_array = rand_point(20, 1000, Lopt::Max);
    let half = point_array.len() / 2;
    let mut line_array: Vec<Line> = Vec::new();
    println!("{:?}", Lopt::Avg);
    return Exchange_Usa(1.5, Currency::Dollar(DollarCountries::Australia));
}
pub fn coordinate_plane() {
    let a = Point(5, 10);
    let b = Point(10, 12);
    let newline = Line { a: a, b: b };
    println!("{:?}", newline);
    println!("x dis: {}", newline.distance());
}

#[derive(Debug)]
struct Point(i64, i64);
#[derive(Debug)]
struct Line {
    a: Point,
    b: Point,
}
// struct Triangle(Line,Line,Line);
impl Line {
    fn distance(&self) -> f32 {
        let xvalue = ((self.b.0 - &self.a.0) as f32).powf(2.0);
        let yvalue = ((self.b.1 - &self.a.1) as f32).powf(2.0);
        return (xvalue as f32 + yvalue as f32).sqrt();
    }
    #[allow(dead_code)]
    fn compare(&self, line2: &Line) -> bool {
        return self.distance() > line2.distance();
    }
}
#[allow(dead_code)]
fn calc_p(triangle: [Line; 3]) -> i32 {
    let mut _result = 0;
    for tri in 0..triangle.len() {
        println!("{}", tri)
        // result += tri;
    }
    return 1;
}
#[derive(Debug)]
#[allow(dead_code)]
enum Lopt {
    Min,
    Avg,
    Max,
    Tree,
}
fn rand_point(amount: i64, seed: i64, options: Lopt) -> Vec<Point> {
    let mut count: i64 = 0;
    let mut result: Vec<Point> = Vec::new();
    while count < amount {
        let random_seed = rand::thread_rng().gen_range(seed, 10000);
        let x = seed * count / random_seed;
        let calc = x;

        let random_x: i64 = rand::thread_rng().gen_range(-9999, random_seed);
        let random_y: i64 = rand::thread_rng().gen_range(-9999, calc) ^ 10000;

        let a;
        match options {
            Lopt::Max => {
                a = Point(random_x, random_y);
                result.push(a)
            }
            Lopt::Avg => {
                a = Point(random_x / seed, random_y / x);
                result.push(a)
            }
            Lopt::Min => {
                a = Point(random_x / random_seed, random_y / random_seed);
                result.push(a)
            }
            Lopt::Tree => {}
        }
        count = count + 1;
    }
    println!("{:?}", result);

    println!("{:?}", options);
    return result;
}
//Match binding value:
pub enum DollarCountries {
    Usa,
    Singapore,
    Hongkong,
    Canada,
    Australia,
}
pub enum Currency {
    Reil,
    Euro,
    Dollar(DollarCountries),
}
pub fn Exchange_Usa(money: f64, curr: Currency) -> f64 {
    fn calc(money: f64, rate: f64) -> f64 {
        let result = money * rate;
        println!("{}$ exchange to {}", money, result);
        return result;
    }
    match curr {
        Currency::Reil => calc(money, 4000.0),
        Currency::Euro => calc(money, 0.92),
        Currency::Dollar(DollarCountries) => {
            use DollarCountries::{Australia, Canada, Hongkong, Singapore, Usa};
            match DollarCountries {
                Hongkong => calc(money, 7.75),
                Australia => calc(money, 1.59),
                Canada => calc(money, 1.42),
                Singapore => calc(money, 1.43),
                Usa => money,
            }
        }
    }
}

/*nested modules*/
pub mod shop {
    mod shopping_cart {
        //utilities
    }
    use std::collections::HashMap;
    #[derive(Debug)]
    pub struct Date(i32, String, i32);
    #[derive(Debug)]
    pub struct Item {
        name: String,
        desc: String,
        price: f32,
        expired_data: Date,
    }
    pub struct Inventory {
        pub items_list: HashMap<String, Item>,
    }
    impl Inventory {
        pub fn insert(&mut self, name: String, item: Item) -> Item {
            let inserted_itm = self.items_list.insert(name, item).unwrap();
            println!("New item with the name of {:#?} was inserted", inserted_itm);
            return inserted_itm;
        }

        pub fn list(&self, name: String) -> Result<&Item, String> {
            if name.to_lowercase() == "all".to_string() {
                for itm in self.items_list.iter() {
                    println!("Available items in shop : {:#?}", itm);
                }
            }

            let found = self.items_list.get(&name);
            match found {
                Some(t) => Ok(t),
                None => Err("Item is not found in the list.".to_string()),
            }
        }
    }
    mod accounting {
        //functions
    }
}
