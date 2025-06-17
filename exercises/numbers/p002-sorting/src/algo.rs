pub fn bubble_sort(nums: &mut Vec<i32>) {
    // Early return for arrays that don't need sorting
    if nums.len() < 2 {
        return;
    }

    // Outer loop: number of passes through the array
    for i in 1..nums.len() {
        // Inner loop: compare adjacent elements, largest bubbles to the end
        for j in 0..nums.len() - i {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }
        // Show progress after each pass
        println!("Pass {}: {:?}", i, nums);
    }
}
