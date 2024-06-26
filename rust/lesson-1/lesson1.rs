
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut index1 = m - 1;
        let mut index2 = n - 1;
        let mut index_merged = m + n - 1;
    
        // 从后向前遍历数组
        while index1 >= 0 && index2 >= 0 {
            if nums1[index1 as usize] > nums2[index2 as usize] {
                nums1[index_merged as usize] = nums1[index1 as usize];
                index1 -= 1;
            } else {
                nums1[index_merged as usize] = nums2[index2 as usize];
                index2 -= 1;
            }
            index_merged -= 1;
        }
    
        // 如果 nums2 还有剩余元素，复制到 nums1
        while index2 >= 0 {
            nums1[index_merged as usize] = nums2[index2 as usize];
            index2 -= 1;
            index_merged -= 1;
        }
    }
}