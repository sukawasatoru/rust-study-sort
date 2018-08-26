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
    insertion_sort(&mut data.data);
    info!("post={:?}", &data.data);

    info!("Bye");
}

fn insertion_sort(src: &mut [i32]) {
    for i in 1..src.len() {
        debug!("{:?}", src);

        if src[i - 1] < src[i] {
            continue;
        }

        let mut j = i;
        let mut tmp = src[j];
        loop {
            src[j] = src[j - 1];

            j -= 1;
            if j <= 0 || src[j - 1] < tmp {
                break;
            }
        }
        debug!("insert {}", tmp);
        src[j] = tmp;
    }
}

#[test]
fn test_insertion_sort() {
    use rust_study_sort::testdata::testdata::get_list2;
    use rust_study_sort::testdata::testdata::get_list3;

    let mut data = get_list1();
    insertion_sort(&mut data.data);
    assert!(data.data == data.expect, format!("{:?}", data.data));

    let mut data2 = get_list2();
    insertion_sort(&mut data2.data);
    assert!(data2.data == data2.expect, format!("{:?}", data2.data));

    let mut data3 = get_list3();
    insertion_sort(&mut data3.data);
    assert!(data3.data == data3.expect, format!("{:?}", data3.data));
}
