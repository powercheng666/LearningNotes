package main

import "fmt"

func merge(nums1 []int, m int, nums2 []int, n int) {
	index1 := m - 1
	index2 := n - 1
	indexMerged := m + n - 1

	// 从后向前遍历数组
	for index1 >= 0 && index2 >= 0 {
		if nums1[index1] > nums2[index2] {
			nums1[indexMerged] = nums1[index1]
			index1--
		} else {
			nums1[indexMerged] = nums2[index2]
			index2--
		}
		indexMerged--
	}

	// 如果 nums2 还有剩余元素，复制到 nums1
	for index2 >= 0 {
		nums1[indexMerged] = nums2[index2]
		index2--
		indexMerged--
	}
}

func main() {
	fmt.Println("Hello, World!")
}
