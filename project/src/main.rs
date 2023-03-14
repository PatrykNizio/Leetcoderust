fn main() {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut results: Vec<i32> = Vec::new();
        for (index_i, i) in nums.iter().enumerate() {
            for (index_j, j) in nums.iter().enumerate() {
                if ((i + j) == target) && (index_i != index_j) {
                    results.push(index_i as i32);
                    results.push(index_j as i32);
                    return results;
                }
            }
        }
        return results;
    }
    let mut nums: Vec<i32> = Vec::new();
    nums.push(3);
    nums.push(2);
    nums.push(4);
    let target = 6;
    let res = two_sum(nums, target);
    println!("{:?}", res)
}
