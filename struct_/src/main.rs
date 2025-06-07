struct Person {
    first: String,
    last: String,
    age: u32,
    gender: String
}

const GRETTINGS: &str = "Hello, world!";

fn main(){

    let first_person = Person{
        first: String::from("Faith"),
        last: String::from("Adedokun"),
        age: 25,
        gender: String::from("Male"),
    };


    println!("{} am {} {} by name, and am {} years of age, {} by gender",GRETTINGS, first_person.first, first_person.last, first_person.age, first_person.gender);
   
    another_d();
}

fn another_d(){
    let first = String::from("Emmanuel");
    let last = String::from("Ajiboye");
    let age = 30;
    let gender = String::from("Female");

    let another_person = Person{first, last, age, gender};
    println!("{} am {} {} by name, and am {} years of age); {}", GRETTINGS, another_person.first, another_person.last, another_person.age, another_person.gender);
}