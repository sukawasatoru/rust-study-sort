extern crate env_logger;
#[macro_use]
extern crate log;
extern crate rust_study_sort;

use rust_study_sort::testdata::testdata::get_list1u;

fn main() {
    env_logger::init();

    info!("Hello");

    let mut data = get_list1u();
    info!("pre={:?}", &data.data);
    bucket_sort(&mut data.data);
    info!("post={:?}", &data.data);

    info!("Bye");
}

fn bucket_sort(src: &mut Vec<u32>) {
    let max = max(&src) as usize;
    let mut bucket = vec![0u32; max + 1];
    for val in src.iter() {
        bucket[*val as usize] += 1;
    }

    let mut src_index = 0;
    for (bucket_index, count) in bucket.iter().enumerate() {
        for _ in 0..*count {
            debug!("bucket_sort val: {}", bucket_index);
            src[src_index] = bucket_index as u32;
            src_index += 1;
        }
    }
}

fn max(src: &[u32]) -> u32 {
    let mut max = src[0];
    for val in src {
        if max < *val {
            max = *val;
        }
    }

    max
}

#[test]
fn test_bucket_sort() {
    let mut data = get_list1u();
    bucket_sort(&mut data.data);
    assert!(data.data == data.expect, format!("{:?}", data.data));
}
