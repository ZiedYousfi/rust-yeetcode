fn main() {
    println!("Hello, world!");
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut resultVec: Vec<i32> = Vec::new();

    for (i, &nums1) in nums.iter().enumerate() {
        for (j, &nums2) in nums.iter().enumerate() {
            if (nums1 + nums2) == target && i != j {
                resultVec.push(i as i32);
                resultVec.push(j as i32);
                return resultVec;
            }
        }
    }

    return resultVec;
}

// 20. Valid Parentheses

pub fn is_valid(s: String) -> bool {
    todo!()
}

// 14. Longest Common Prefix
// Write a function to find the longest common prefix string amongst an array of strings.

// If there is no common prefix, return an empty string "".
// Example 1:

// Input: strs = ["flower","flow","flight"]
// Output: "fl"
// Example 2:

// Input: strs = ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.

// Constraints:

// 1 <= strs.length <= 200
// 0 <= strs[i].length <= 200
// strs[i] consists of only lowercase English letters if it is non-empty.

// Example usage
#[test]
fn test_longest_common_prefix() {
    let strs1 = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    assert_eq!(longest_common_prefix(strs1), "fl");

    let strs2 = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
    assert_eq!(longest_common_prefix(strs2), "");

    let strs3 = vec!["apple".to_string()];
    assert_eq!(longest_common_prefix(strs3), "apple");
}

//let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut result: String = String::new();

    if strs.is_empty() {
        return result;
    }

    if let Some(first_str) = strs.first() {
        for (i, c) in first_str.chars().enumerate() {
            if strs.iter().all(|s| s.chars().nth(i) == Some(c)) {
                result.push(c);
            } else {
                break;
            }
        }
    }

    result
}

// 26. Remove Duplicates from Sorted Array

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    println!("{:?}", nums);

    let mut resultvec: Vec<i32> = Vec::new();

    if !nums.is_empty() {
        resultvec.push(nums[0]);
        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                resultvec.push(nums[i]);
            }
        }
    }

    if resultvec.is_empty() {
        return 0;
    }

    nums[..(resultvec.len())].copy_from_slice(&resultvec[..resultvec.len()]);

    println!("{:?}", resultvec);
    println!("{:?}", resultvec.len());

    resultvec.len() as i32
}

#[test]
fn test_remove_duplicates() {
    let mut nums1 = vec![1, 1, 2];
    assert_eq!(remove_duplicates(&mut nums1), 2);
    assert_eq!(nums1[0..2], [1, 2]);

    let mut nums2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(remove_duplicates(&mut nums2), 5);
    assert_eq!(nums2[0..5], [0, 1, 2, 3, 4]);

    let mut nums3: Vec<i32> = vec![];
    assert_eq!(remove_duplicates(&mut nums3), 0);

    let mut nums4 = vec![1];
    assert_eq!(remove_duplicates(&mut nums4), 1);
    assert_eq!(nums4[0..1], [1]);

    let mut nums5 = vec![1, 2, 3, 4, 5];
    assert_eq!(remove_duplicates(&mut nums5), 5);
    assert_eq!(nums5[0..5], [1, 2, 3, 4, 5]);
}

// 27. Remove Element

// Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.

// Consider the number of elements in nums which are not equal to val be k, to get accepted, you need to do the following things:

// Change the array nums such that the first k elements of nums contain the elements which are not equal to val. The remaining elements of nums are not important as well as the size of nums.
// Return k.

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {

    nums.retain_mut(|x| *x != val);

    nums.len() as i32
}

#[test]
fn test_remove_element() {
    let mut nums1 = vec![3, 2, 2, 3];
    assert_eq!(remove_element(&mut nums1, 3), 2);
    let mut actual = nums1[0..2].iter().collect::<Vec<&i32>>();
    actual.sort();
    let mut expected = [&2, &2];
    expected.sort();
    assert_eq!(actual, expected);

    let mut nums2 = vec![0, 1, 2, 2, 3, 0, 4, 2];
    assert_eq!(remove_element(&mut nums2, 2), 5);
    let mut actual = nums2[0..5].iter().collect::<Vec<&i32>>();
    actual.sort();
    let mut expected = [&0, &0, &1, &3, &4];
    expected.sort();
    assert_eq!(actual, expected);

    let mut nums3: Vec<i32> = vec![];
    assert_eq!(remove_element(&mut nums3, 0), 0);

    let mut nums4 = vec![5];
    assert_eq!(remove_element(&mut nums4, 5), 0);
    assert_eq!(nums4.len(), 0);

    let mut nums5 = vec![5];
    assert_eq!(remove_element(&mut nums5, 3), 1);
    assert_eq!(nums5[0..1], [5]);

    let mut nums6 = vec![5, 5, 5, 5];
    assert_eq!(remove_element(&mut nums6, 5), 0);
    assert_eq!(nums6.len(), 0);
}
