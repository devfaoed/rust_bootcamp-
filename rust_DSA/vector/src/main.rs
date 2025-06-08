
fn main() {
    let number = vec![1, 2, 3, 4, 5];
    match &number[..]{
        [first, rest @ ..] => {
            println!("The first elementt is {} and the rest of the elements are: {:?}", first, rest);
        }  
        _=> println!("The vector is empty"), 
    }
}
