use std::collections::*;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
            let mut ss = HashSet::new();
            for n in nums{
                if ss.contains(&n){
                    return true;
                }
                ss.insert(n);
            }
        return false;
    }
}
