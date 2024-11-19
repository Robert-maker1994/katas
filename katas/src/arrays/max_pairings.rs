use super::Arrays;

/// TODO documation and organising 
impl Arrays {
    fn lower_bound(nums: &Vec<i32>, mut low: usize, mut high: usize, element: i32) -> usize {
        while low <= high {
            let mid = low + (high - low) / 2;
            if nums[mid] >= element {
                if mid == 0 { break; }
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        low
    }
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut nums = nums.clone();
        nums.sort();
        let mut ans = 0i64;

        for i in 0..nums.len() {
            let low = Self::lower_bound(&nums, i + 1, nums.len() - 1, lower - nums[i]);

            let high = Self::lower_bound(&nums, i + 1, nums.len() - 1, upper - nums[i] + 1);

            ans += (high - low) as i64;
        }

        ans
        
    }
}