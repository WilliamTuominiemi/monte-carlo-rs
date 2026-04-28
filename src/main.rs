fn main() {
    let x = generate_list_of_random_values(10);
    let y = generate_list_of_random_values(10);

    println!("{:?}", x);
    println!("{:?}", y);
}

fn generate_random_float() -> f32 {
    rand::random_range(-1.0..=1.0)
}

fn generate_list_of_random_values(size: usize) -> Vec<f32> {
    (0..size).map(|_| generate_random_float()).collect()
}
