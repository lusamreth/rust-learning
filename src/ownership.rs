#![allow(dead_code)]
pub fn testing() {
    let literal_string = "DIS is literal string; allocated in stack memory";
    let _copy_x = literal_string; // <- the var get copied and bind to copy_x

    let heap_string = String::from("Dis is the string dat store in da heappp");
    let ref_str = &heap_string; // borrow var
    let borrow_from_heap_string = borrow_sth(ref_str);
    // hey look heap_string still have its ownership because all da fx just borow it
    println!("Ownership {}", heap_string);
    /*"&" syntax mean reference to the pointer from the original heap_type variable */
    println!("Testing memory  {}", literal_string);
    println!("Testing memory  {}", borrow_from_heap_string);

    let mut newstx = String::from("Testing stx");
    let a = &newstx;
    let b = &newstx;
    println!("{} {}", a, b);
    // a & b is out of scoped;
    let muta = &mut newstx;
    muta.push('A');
    muta.push_str(",Additional_string");
    println!("{:?}", (&muta, firstword(&muta)));
    anotherscope();
    let testing_2 = String::from("hey guys this is austin");
    let _spaceout = split_space(&testing_2);
    /*
        0 -> 2;s:3
        4 -> 7;s:8
        9 -> 12;s:13
        14 -> 19;s:null
    */
} // here is out of scope

pub fn borrow_sth(strx: &str) -> &str {
    println!(
        "We borrow dis variables with the value :: \n ++ //{} ++",
        strx
    );
    return strx;
}

pub fn anotherscope() {
    let a = String::from("ayayya");
    let b = a;
    println!(
        "{:?}",
        (b, "a is moved to b, so it cannot access data anymore")
    );
    //b is not longer usable
}

pub fn firstword(word: &str) -> &str {
    let Wordsliced = word.as_bytes(); // convert to ASCII code;
    for (i, &byte) in Wordsliced.iter().enumerate() {
        println!("value : {:?}", (i, byte));
        if byte == b' ' {
            return &word[..i];
        }
    }
    println!("word params {:?}", (word, Wordsliced));
    return &word[..];
}

pub fn split_space(word: &str) -> Vec<&str>  {
    let ascii_array = word.as_bytes();
    let mut v: Vec<usize> = Vec::new();
    for (idx, &byte) in ascii_array.iter().enumerate() {
        if byte == b' ' {
            v.push(idx); // spaces index
        }
    }
    let mut spacingword: Vec<&str> = Vec::new();
    let mut tempstack = Vec::new();
    let mut count = 0;
    let mut scount = 0;

    while count < word.len(){
        if count == v[scount] && scount < v.len()- 1{

            if scount == 0 {
                spacingword.push(&word[0..v[scount]]);
            }
            tempstack.push(v[scount] + 1);
            spacingword.push(&word[tempstack[0]..v[scount + 1]]);
                
            tempstack.pop();
            scount = scount + 1;
        } 

        if count == v[v.len() - 1]{
            spacingword.push(&word[count + 1..]);
        }
        
        count = count + 1;
    }
    println!("sp {:#?}",&spacingword);
    return spacingword;
}
