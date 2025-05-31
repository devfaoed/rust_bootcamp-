fn main(){
    let number = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let slice = &number[3..8];
    println!("we are slicing the provided array of numbers and the result gotten are {:?}", slice);
}