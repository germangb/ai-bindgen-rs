use ai_bindgen::ai;

#[ai]
extern "C" {
    #[ai]
    fn max(a: i32, b: i32) -> i32;
}

fn main() {
    println!("max(23, 67) = {}", max(23, 67)); // 67
}
