use ai_bindgen::ai;

#[ai]
extern "C" {
    #[ai(prompt = "This function computes the max of the two values")]
    fn magic_max(a: i32, b: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ai]
    extern "C" {
        #[ai(prompt = "Generate some test cases for the magic_max(a, b) function please.")]
        #[test]
        fn test_ai_max();
    }
}

fn main() {
    println!("magic_max(4, 6) = {}", magic_max(4, 6)); // 6!
}
