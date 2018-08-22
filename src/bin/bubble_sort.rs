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
    bubble_sort(&mut data.data);
    info!("post={:?}", &data.data);

    info!("Bye");
}

fn bubble_sort(src: &mut Vec<i32>) {
    let len = src.len();
    for i in 0..(len - 1) {
        debug!("i: {}, data: {:?}", i, src);
        for j in (i + 1)..(len - 1) {
            debug!("i: {}, j: {}, data: {:?}", i, j, src);

            if src[j] < src[i] {
                debug!("swap {}, {}", src[i], src[j]);

                src.swap(i, j);
            }
        }
    }
}

#[test]
fn test_bubble_sort() {
    let mut data = get_list1();
    bubble_sort(&mut data.data);
    assert!(data.data == data.expect, format!("{:?}", data.data));
}
