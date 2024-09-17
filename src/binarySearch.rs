fn  main() {
    let array: [i32; 10] = [3, 7, 8, 11, 16, 17, 19, 23, 27, 30];
    let target: i32 = 27;
    let mut left: usize = 0;
    let mut right: usize = array.len() - 1;
    
    while left <= right {
        let middle_index: usize = left + (right - left) / 2;
        let current_value: i32 = array[middle_index];

        if current_value == target {
            println!("Target found at index: {}", middle_index);
            return;
        } else if current_value > target {
            right = middle_index - 1;
        } else {
            left = middle_index + 1;
        }
    }
}