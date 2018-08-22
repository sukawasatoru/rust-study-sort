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
    if right <= left {
        return;
    }

    // left          right
    //   |               |
    // - - - - - - - - - -
    //         |
    //   (right - left) / 2
    let pivot = med3(src[left], src[left + (right - left) / 2], src[right]);

    let mut left_index = left;
    let mut right_index = right;
    loop {
        while src[left_index] < pivot {
            left_index += 1;
        }

        while pivot < src[right_index] {
            right_index -= 1;
        }

        if right_index <= left_index {
            break;
        }

        src.swap(left_index, right_index);
        left_index += 1;
        right_index -= 1;

        sort(src, left, left_index - 1);
        sort(src, right_index, right);
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
    let mut data = get_list1();
    quick_sort(&mut data.data);
    assert!(data.data == data.expect, format!("{:?}", data.data));
}
