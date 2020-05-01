pub fn run() -> i32 {
    let teststring = "hey guy this is austin the autism minecraft player";
    let smallnum: i32 = 91332320;
    let bignum: i64 = 01083912720173439;
    let _bool: bool = true;
    let _tuple: (i32, i32, &str) = (1, 2, "hey");
    println!("hello hahaha!{}", teststring);
    println!("small number {} => i32max:{}", smallnum, i32::max_value());
    println!("big number {} => i64max:{}", bignum, i64::max_value());
    println!("This is tuple {:?}", (1, 2, 3, "hello", false));
    return fibo(20);
    
}
pub fn fibo(mut num: i32) -> i32 {
    let mut total: i32 = 0;
    while num >= 0 {
        println!("aye {}", num);
        total += num;
        num = num - 1;
    }
    println!("fib total {}", total);
    return total;
}
