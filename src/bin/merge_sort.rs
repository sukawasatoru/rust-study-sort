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
    merge_sort(&mut data.data);
    info!("post={:?}", &data.data);

    info!("Bye");
}

fn merge_sort(src: &mut [i32]) {
    let merge = |left: &[i32], right: &[i32]| -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(left.len() + right.len());
        let mut left_index = 0;
        let mut right_index = 0;

        loop {
            if left[left_index] <= right[right_index] {
                debug!("merge_sort left: {}", left[left_index]);
                result.push(left[left_index]);
                left_index += 1;
                if left.len() <= left_index {
                    debug!("merge_sort append right: {:?}",
                           right[right_index..right.len()].to_vec());
                    result.extend_from_slice(&right[right_index..right.len()]);
                    break;
                }
            } else {
                debug!("merge_sort right: {}", right[right_index]);
                result.push(right[right_index]);
                right_index += 1;
                if right.len() <= right_index {
                    debug!("merge_sort append left: {:?}", left[left_index..left.len()].to_vec());
                    result.extend_from_slice(&left[left_index..left.len()]);
                    break;
                }
            }
        }

        result
    };

    struct PrepareAndMerge<'s> {
        f: &'s Fn(&PrepareAndMerge, &[i32]) -> Vec<i32>
    }

    let prepare_and_merge = PrepareAndMerge {
        f: &|step1, data| -> Vec<i32> {
            if data.len() <= 1 {
                return data.to_vec();
            }

            debug!("prepare_and_merge data: {:?}", data);

            let a = data.split_at(data.len() / 2);
            let left: Vec<i32> = (step1.f)(step1, a.0);
            let right: Vec<i32> = (step1.f)(step1, a.1);
            debug!("prepare_and_merge left: {:?}, right: {:?}", left, right);
            merge(&left, &right)
        }
    };

    let merged = (prepare_and_merge.f)(&prepare_and_merge, src);
    src.copy_from_slice(&merged);
}

#[test]
fn test_merge_sort() {
    use rust_study_sort::testdata::testdata::get_list2;
    use rust_study_sort::testdata::testdata::get_list3;

    let mut data = get_list1();
    merge_sort(&mut data.data);
    assert!(data.data == data.expect, format!("{:?}", data.data));

    let mut data2 = get_list2();
    merge_sort(&mut data2.data);
    assert!(data2.data == data2.expect, format!("{:?}", data2.data));

    let mut data3 = get_list3();
    merge_sort(&mut data3.data);
    assert!(data3.data == data3.expect, format!("{:?}", data3.data));
}
