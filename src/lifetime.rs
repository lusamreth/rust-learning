pub fn runner(){
    println!("{}",ptr());
    let sub_scope = 199999;
    let mut bum :&i32;
    {
        let inner_scope = 89119999;
        bum = compare(&sub_scope, &inner_scope);
        println!("{:#?}",bum);

    }
    println!("{:#?}",compare(&32,&20));
    let mut abab = Vec::new();
    abab.push(String::from("Besstonn"));
    abab.push(String::from("0123"));
    let result = ahhhh(&abab);
    println!("{:#?}",result);

}
/*
fn  lifetime_test<'a>() -> &'a i32{
    let mut borrower:&'a i32;
    // this while block won't work
    while 1 > 2{
        do the cal logic blah blah
        let result:i32 = 92831238; (this var won't live pass the scope)
        borrower = &result;
    }// result is dead,get cleaned
    return borrower;// read garbage var 
}
*/

fn ptr<'a>() -> &'a str{
    let some = "ayaya"; // dis will get kill once finish end;
    some
}// some get cleaned here // so we cannot return it;
// but since we have lifetime a, we insure that it will long enough to use;
fn compare<'a,'b, T:PartialOrd>(x:&'a T,y:&'b T) -> &'a T where T: std::ops::Add {
    if x > y {
        x
    }else{
        x// dis work because returning x is in a lifetime so it live longer to work//
        //y dis cause error cause y is in b which is shorter lifetime
    }
}
struct Testing<'a,'b>{
    name : &'a String,
    num : &'b String
}
use std::fmt;
impl fmt::Debug for Testing<'_, '_>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"testing result: {:#?}",(self.name,self.num))
    }
}
trait Trait1{
    fn print_name(&self) -> String;
    fn parse_num(&self) -> i32;
}
impl Trait1 for Testing<'_, '_>{
    fn print_name(&self) -> String {
        self.name.clone()
    }
    fn parse_num(&self) -> i32 {
        self.num.parse::<i32>().unwrap().clone()
    }
}
fn ahhhh(param:&Vec<String>) ->Testing{
    return Testing{
        name:&param[0],
        num:&param[1]
    }
}
