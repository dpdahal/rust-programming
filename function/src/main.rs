// fn user(){
//     println!("user function");
// }

fn main() {
    // user("ram", 23);
    let name = user("ram");
    println!("name = {}", name);
    
}

// function access parameter
// fn user(name:&str,age:i32){
//     println!("name = {}", name.to_string());
//     println!("age = {}", age);
// }


// function ruturn value
fn user(name:&str)->String{
    return name.to_string();
}