fn main(){
    println!("hello world");
    let name="John";
    println!("Hello, this is {}", name);
    // name="tom"; This will cause error
    let mut mutable_name = "John";
    println!("Hello, my initial name is {}", mutable_name);
    mutable_name = "Tom";
    println!("Hello, this is {}", mutable_name);
    for i in 1..11{//11 is not inclusive
        print!("{}\t", i)//This will print without changing a line
    }
    println!()
}
