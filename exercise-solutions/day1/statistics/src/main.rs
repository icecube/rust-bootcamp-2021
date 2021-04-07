fn main() {
    let array = [1,5,3,21,3,45,6,23,4,5,64,2,23,5,5,32,23,45];
    let mut min = 1000000;
    let mut max: i32 = -1;
    let mut mean: f32 = 0.;
    for i in array.iter() {
        let i = *i;
        if i < min {
            min = i;
        }
        if i > max {
            max = i;
        }
        mean += i as f32;
    }
    mean /= array.len() as f32;
    println!("The min is {}", min);
    println!("The max is {}", max);
    println!("The mean is {}", mean);
}
