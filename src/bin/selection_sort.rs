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
    selection_sort(&mut data.data);
    info!("post={:?}", &data.data);

    info!("Bye");
}

fn selection_sort(src: &mut Vec<i32>) {
    let len = src.len();
    for i in 0..(len - 1) {
        debug!("i: {}, data: {:?}", i, src);

        let mut min_index = i;
        for j in (i + 1)..(len - 1) {
            if src[j] < src[min_index] {
                min_index = j;
            }
        }

        debug!("swap {}, {}", src[i], src[min_index]);
        src.swap(i, min_index);
    }
}

#[test]
fn test_selection_sort() {
    let mut data = get_list1();
    selection_sort(&mut data.data);
    assert!(data.data == data.expect, format!("{:?}", data.data));
}
