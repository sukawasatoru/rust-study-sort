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

fn selection_sort(src: &mut [i32]) {
    let right = src.len() - 1;
    for i in 0..right {
        debug!("{:?}", src);

        let mut min_index = i;
        for j in i..=right {
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
    use rust_study_sort::testdata::testdata::get_list2;
    use rust_study_sort::testdata::testdata::get_list3;

    let mut data = get_list1();
    selection_sort(&mut data.data);
    assert!(data.data == data.expect, format!("{:?}", data.data));

    let mut data2 = get_list2();
    selection_sort(&mut data2.data);
    assert!(data2.data == data2.expect, format!("{:?}", data2.data));

    let mut data3 = get_list3();
    selection_sort(&mut data3.data);
    assert!(data3.data == data3.expect, format!("{:?}", data3.data));
}
