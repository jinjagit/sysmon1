// pub const GREETING: &'static str = "Hallo, Rust library here!";

pub fn array_ave(arry: Vec<f32>) -> f32 {
    let count: usize = arry.iter().count();
    let mut total: f32 = 0.0;

    for i in 0..count {
        total += arry[i];
    }

    return total / count as f32;
}
