fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let total_nums  : usize = nums1.len() + nums2.len();
    let is_odd      : bool  = total_nums % 2 == 1;
    let median_index: usize = if is_odd { total_nums/2 } else { total_nums/2 - 1};

    let mut n1i: usize = 0;
    let mut n2i: usize = 0;
    for i in 0..median_index {
             if n1i == nums1.len()      { n2i += 1; }
        else if n2i == nums2.len()      { n1i += 1; }
        else if nums1[n1i] < nums2[n2i] { n1i += 1; }
        else                            { n2i += 1; }
    }
    
    if is_odd {
             if n1i == nums1.len() { return nums2[n2i].into(); }
        else if n2i == nums2.len() { return nums1[n1i].into(); }
        else                       { return if nums1[n1i] < nums2[n2i] { nums1[n1i].into() } else { nums2[n2i].into() }; }
    }

         if n1i == nums1.len() { return (nums2[n2i] + nums2[n2i+1]) as f64 / 2.0; }
    else if n2i == nums2.len() { return (nums1[n1i] + nums1[n1i+1]) as f64 / 2.0; }
    else {
        let first = if nums1[n1i] < nums2[n2i] { nums1[n1i] } else { nums2[n2i] };
        let second = if nums1[n1i] < nums2[n2i] {
            if n1i + 1 < nums1.len() && nums1[n1i + 1] < nums2[n2i] {
                nums1[n1i + 1]
            } else {
                nums2[n2i]
            }
        } else {
            if n2i + 1 < nums2.len() && nums2[n2i + 1] < nums1[n1i] {
                nums2[n2i + 1]
            } else {
                nums1[n1i]
            }
        };
        
        return (first + second) as f64 / 2.0;
    }
}

fn main() {
    let v1: Vec<i32> = vec![2, 6];
    let v2: Vec<i32> = vec![1, 4];

    let median: f64 = find_median_sorted_arrays(v1, v2);
    println!("Median: {}", median)
}
