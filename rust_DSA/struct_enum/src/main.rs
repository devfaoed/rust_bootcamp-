// write a rust program to create a user model with three diferrent role user,seller,admin

// Define the Role enum separately
enum Role {
    User,
    Seller,
    Admin,
}

// Implement Display trait to allow printing the role as text
impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Role::User => write!(f, "a user"),
            Role::Seller => write!(f, "a seller"),
            Role::Admin => write!(f, "an admin"),
        }
    }
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


    let user1 = User { name, email, number, role };

    println!(
        "user {} with {} and contact number {} has a {} role",
        user1.name, user1.email, user1.number, user1.role
    );
}