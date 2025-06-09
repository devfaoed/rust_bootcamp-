// write a rust program to create a user model with three diferrent role user,seller,admin

#[derive(Debug)]
enum Role {
    User,
    Seller,
    Admin,
}


// Define the User struct
struct User {
    name: String,
    email: String,
    number: u64, // phone numbers are usually larger than u32 allows
    role: Role,
}

fn main() {
    let name = String::from("Adedokun Faith");
    let email = String::from("devfaith@gmail.com");
    let number = 9063111875; // use u64 (no leading zero needed)
    let role = Role::User;

    match role{
        Role::User => println!("a user"),
        Role::Seller => println!("a seller"),
        Role::Admin => println!("an admin"),
    }

    let user1 = User { name, email, number, role };

    println!(
        "user {} with {} and contact number {} has {:?} role",
        user1.name, user1.email, user1.number, user1.role
    );
}