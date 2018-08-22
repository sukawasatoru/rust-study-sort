use testdata::protocols::TestData;
use testdata::protocols::TestDataU;

pub fn get_list1() -> TestData {
    TestData {
        data: vec![1, 5, 4, 7, 2, 10],
        expect: vec!(1, 2, 4, 5, 7, 10),
    }
}

pub fn get_list1u() -> TestDataU {
    TestDataU {
        data: vec![1, 5, 4, 7, 2, 10],
        expect: vec!(1, 2, 4, 5, 7, 10),
    }
}
