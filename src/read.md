/*
    Num:{
        p -> size of bits;
        signed:i[p],
        unsigned:u[p]
        float:f[p]
        *size of bit: [8,16,32,64,128,arch]
    }
    statement dont' return value;expression do; statements are instruction that perfom some action;
    example : statement -> let x = 10;

    expression is used to evaluate a result / valuel;It has combination of different operator,var,const,etc;
    it does not have semicolon; example : exp -> 10 + 10 (return 20),

    *expression

    if expression: use to return / execute the expression inside its blocks when particular condition is meet;
    Condition itself is a witbise expression that return boolen;It must be boolean to work;"if" could also be
    used inside variable like 'let', however the retun value must be the same type otherwise compiler will
    throw error;We can chain 'if' with else if or match;

    Loop:
    There are 3 loops that we can use: loops,for,while and have each one have distinct characteristic and will
    be suitable for different tasks;
    +loop: execute its code block over and over again until we manaully tell it to stop at some point;The bummer
    of this loop is that it need to nest different condition before this execution breaks which add more overhead;
    +While: this loop address the problem that loop has;It look at the condition everytime the code block finish
    and if that condition hold true it'll keep running until false state;
    +for:This loop is optimal to go through collections and lists since it's less error prone than while and also faster;

    *break is a keyword that tell rust to break out the loop and get the final result;
    [Exercises]:
    Convert temperatures between Fahrenheit and Celsius.
    Generate the nth Fibonacci number.
    Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”;

*/

// custom scope to test memory ownership
/**
 * Ownership summary:
 * Rust implement a garbage collector through sth call ownership;It's a set of rule that the compiler has
 * to check at compile time and ensure that the memory stay safe as much as possible;Basically rust will
 * invoke special function called "drop" when the variable is out of its scope;
 * *out of scoped : mean the var is used outside the curly bracket or the fx already used it;
 * Some notes:
 * -Each Value in Rust has its own variables call owner;
 * -The value only belong to one owner;
 * -The variable only avialable in the scope otherwise it will be dropped;
 * 
 * Primative types could be copy from the original var and bind through other var really quickly;The reason is :
 * -those types have a fixed size and length that can be known at compile time therefore it could be pushed into a 
 * stack when in used and pop off when we finished;
 * -While, this hardcoded var could be quicker to run and bind easily,it's not adjustable and require to know 
 * the value prior;
 * 
 * How the heap works:
 * -Compiler init the value and call for memory;
 * -OS need to jump around to find enough spaces and book-keep them;
 * -During retrieval OS need to manaully find the data through pointers store in stack;
 * *because the size could be grow or shrink, the data location could be messy and require pointer to guide;
 * => More work for OS ,slow runtime / but in return more flexibility;
 *
 * by default because referencing has no ownership,we cannot modify the borrowed value;unless:
 * -the main owner is mutable
 * -and only 1 mutable reference is allowed at a time;
 * -if the dat borrowed var is out of scope then u can init another one;
 * 
 * function could either borow the var from parameter or completely own it or return it back too;
*/