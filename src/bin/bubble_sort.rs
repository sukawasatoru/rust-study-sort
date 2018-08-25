extern crate env_logger;
#[macro_use]
extern crate log;
extern crate rust_study_sort;

use rust_study_sort::testdata::testdata::get_list1;
use rust_study_sort::testdata::testdata::get_list2;
use rust_study_sort::testdata::testdata::get_list3;

fn main() {
    env_logger::init();

    info!("Hello");

    let mut data = get_list1();
    info!("pre={:?}", &data.data);
    bubble_sort(&mut data.data);
    info!("post={:?}", &data.data);

    info!("Bye");
}

fn bubble_sort(src: &mut [i32]) {
    let right = src.len() - 1;
    for i in 0..=right {
        debug!("{:?}", src);
        for j in 1..=(right - i) {
            if src[j] < src[j - 1] {
                debug!("swap {}, {}", src[j - 1], src[j]);

                src.swap(j - 1, j);
            }
        }
    }
}

#[test]
fn test_bubble_sort() {
    let mut data = get_list1();
    bubble_sort(&mut data.data);
    assert!(data.data == data.expect, format!("{:?}", data.data));

    let mut data2 = get_list2();
    bubble_sort(&mut data2.data);
    assert!(data2.data == data2.expect, format!("{:?}", data2.data));

    let mut data3 = get_list3();
    bubble_sort(&mut data3.data);
    assert!(data3.data == data3.expect, format!("{:?}", data3.data));
}
