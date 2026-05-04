fn create_string() -> String{
    let s = String::from("hello");
    s
}

fn main(){
    let result = create_string();
    println!("{}", result);
}