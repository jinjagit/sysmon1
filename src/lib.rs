use sysinfo::{ProcessorExt, System, SystemExt};

// pub const GREETING: &'static str = "Hallo, Rust library here!";

pub fn array_ave(arry: Vec<f32>) -> f32 {
    let count: usize = arry.iter().count();
    let mut total: f32 = 0.0;

    for i in 0..count {
        total += arry[i];
    }

    return total / count as f32;
}

pub fn add_to_queue(mut arry: Vec<f32>, new_elem: f32) -> Vec<f32> {
    arry.push(new_elem); // add element to end of vector
    arry.remove(0); // remove element at start of vector

    arry
}

pub fn get_num_cores() -> u8 {
    let system = sysinfo::System::new_all();
    let mut num: u8 = 0;

    // Count virtual cores.
    for _processor in system.get_processors() {
        num += 1;
    }

    num
}