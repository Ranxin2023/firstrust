fn factorial(n:i32)->i32{
    if n==0{
        return 1
    }
    return n*factorial(n-1)
}
struct Student{
    name:String, age:i32
}
impl Student{
    fn show(&self){
        println!("My name is {} and I am {} years old", self.name, self.age)
    }
}
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
    println!();
    println!("The factoral of 8 is: {}", factorial(8));
    let arr=[3, 2, 5, 1, 6, 4, 5, 9];
    println!("All the elements in arr are:");
    for idx in 0..arr.len(){
        print!("{}\t", arr[idx]);
    }
    println!();
    let mut count_arr=[0;10];
    for idx in 0..arr.len(){
        count_arr[arr[idx]]+=1;
    } 
    for num in 0..count_arr.len(){
        println!("num: {} {}", num, count_arr[num]);
    }
    // String part
    let mut student_name=String::from("John");
    student_name.push_str(" Smith");
    let s=Student{name:student_name, age:25};
    s.show();
    let last_name=&s.name[5..];
    println!("My last name is: {}", last_name);
}
