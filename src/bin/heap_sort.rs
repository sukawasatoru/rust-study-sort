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
    heap_sort(&mut data.data);
    info!("post={:?}", &data.data);

    info!("Bye");
}

fn heap_sort(src: &mut [i32]) {
    for i in 1..src.len() {
        up_heap(src, i);
    }

    for i in (1..src.len()).rev() {
        src.swap(0, i);
        down_heap(src, i);
    }
}

fn up_heap(src: &mut [i32], index: usize) {
    let parent = |i: usize| {
        (i + 1) / 2 - 1
    };

    let mut i = index;
    loop {
        if i <= 0 {
            break;
        }

        let m = parent(i);

        if src[m] < src[i] {
            src.swap(m, i);
        } else {
            break;
        }

        i = m;
    }
}

fn down_heap(src: &mut [i32], index: usize) {
    let left_child = |i: usize| {
        (i + 1) * 2 - 1
    };

    let right_child = |i: usize| {
        (i + 1) * 2
    };

    let mut m = 0;
    let mut tmp = 0;
    loop {
        let left_index = left_child(m);
        let right_index = right_child(m);

        if index <= left_index {
            break;
        }

        if src[tmp] < src[left_index] {
            tmp = left_index;
        }

        if right_index < index && src[tmp] < src[right_index] {
            tmp = right_index;
        }

        if tmp == m {
            break;
        }

        src.swap(tmp, m);

        m = tmp;
    }
}

#[test]
fn test_heap_sort() {
    let mut data = get_list1();
    heap_sort(&mut data.data);
    assert!(data.data == data.expect, format!("{:?}", data.data));
}
