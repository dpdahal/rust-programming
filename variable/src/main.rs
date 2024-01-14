fn main() {
    let num = 200;
    println!("num = {}", num);
    // here is an example of variable mutable 
    let mut num = 100;
    println!("num = {}", num);
    num = 300;
    println!("num = {}", num);
    // constant variable
    const MAX_NUM: u32 = 100_000;
    println!("MAX_NUM = {}", MAX_NUM);

}

/*
Rule for variable naming:
1. Variable names must start with a letter or underscore.
2. Subsequent characters can be letters, numbers, or underscores.
3. Variable names are case sensitive.
4. Variable names cannot be the same as Rust keywords.



*/