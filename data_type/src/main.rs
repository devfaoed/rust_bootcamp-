// learning integar
fn main(){
    let age = 24;
    println!("i am {}, years old", age);
    price()

}

// learning floating point
fn price(){
    let cost = 6.8;
    println!("the total cost of the item is ${}", cost);
    char()
}

// learning character
fn char(){
    let my_char:char = 'A';
    println!("the first letter of the alphabet is {}", my_char);
    boolean()
}

// learning boolean
fn boolean(){
    let is_active:bool = true;
    println!("is the user active? {}", is_active);
    name()
}

// learning String
fn name(){
    let name:&str = "faith Adedokun";
    println!("my name is {}", name);
    tupple()
}

// learning tupple
fn tupple(){
    let person: (&'static str, &'static str, i32, &'static str, bool) = ("Adedokun", "Faith", 25, "M", true);
    println!("introducing user First Name: {}, Last Name: {}, Age:{}, Gender: {}, Active:{}", person.0, person.1, person.2, person.3, person.4);
}

