fn main() {
    let nums = vec![1, 3, 2, 99, 77, 88];
    let largest = largest_num(&nums);
    println!("num is {}", largest);
}

fn largest_num(nums: &Vec<i32>) -> i32 {
    let mut i = nums[0];
    for item in nums.iter() {
        if i < *item {
            i = *item;
        }
    }
    i
}
