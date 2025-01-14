fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 10; //Modify the vector in safe way
    println!("The first element is: {:?}", v[0]);
} 