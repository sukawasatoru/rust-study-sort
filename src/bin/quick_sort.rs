extern crate env_logger;
#[macro_use]
extern crate log;
extern crate rust_study_sort;

use rust_study_sort::testdata::testdata::get_list1;

fn main() {
    env_logger::init();

    info!("Hello");

    let mut data = get_list1();
    info!("pre={:?}", &data.data);
    quick_sort(&mut data.data);
    info!("post={:?}", &data.data);

    info!("Bye");
}

fn quick_sort(src: &mut Vec<i32>) {
    let slice = src.as_mut_slice();
    let len = slice.len();
    sort(slice, 0, len - 1);
}

fn sort(src: &mut [i32], left: usize, right: usize) {
    // left          right
    //   |               |
    // - - - - - - - - - -
    //         |
    //   (right - left) / 2
    let pivot = med3(src[left], src[left + (right - left) / 2], src[right]);
    let mut left_index = left;
    let mut right_index = right;
    loop {
        debug!("data: {:?}, left_index: {}, right_index: {}", src, left_index, right_index);

        while src[left_index] < pivot {
            left_index += 1;
        }

        while pivot < src[right_index] {
            right_index -= 1;
        }

        if left_index <= right_index {
            debug!("swap {}, {}", src[left_index], src[right_index]);
            src.swap(left_index, right_index);
            if left_index < right {
                left_index += 1;
            }

            if left < right_index {
                right_index -= 1;
            }
        } else {
            break;
        }
    }

    if left < right_index && 0 < right_index - left {
        debug!("recursive left: {}, {}", left, right_index);
        sort(src, left, right_index);
    }

    if left_index < right && 0 < right - left_index {
        debug!("recursive right: {}, {}", left_index, right);
        sort(src, left_index, right);
    }
}

fn med3(left: i32, mid: i32, right: i32) -> i32 {
    if left < right {
        if right < mid {
            // left, right, mid
            right
        } else if left < mid {
            // left, mid, right
            mid
        } else {
            // mid, left, right
            left
        }
    } else {
        if left < mid {
            // right, left, mid
            left
        } else if mid < right {
            // mid, right, left
            right
        } else {
            // right, mid, left
            mid
        }
    }
}

#[test]
fn test_mid3() {
    assert_eq!(med3(1, 2, 3), 2, "1, 2, 3");
    assert_eq!(med3(2, 3, 1), 2, "2, 3, 1");
    assert_eq!(med3(3, 1, 2), 2, "3, 1, 2");
}

#[test]
fn test_quick_sort() {
    use rust_study_sort::testdata::testdata::get_list2;
    use rust_study_sort::testdata::testdata::get_list3;

    let mut data = get_list1();
    quick_sort(&mut data.data);
    assert!(data.data == data.expect, format!("{:?}", data.data));

    let mut data2 = get_list2();
    quick_sort(&mut data2.data);
    assert!(data2.data == data2.expect, format!("{:?}", data2.data));

    let mut data3 = get_list3();
    quick_sort(&mut data3.data);
    assert!(data3.data == data3.expect, format!("{:?}", data3.data));
}
