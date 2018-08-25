use testdata::protocols::TestData;
use testdata::protocols::TestDataU;

pub fn get_list1() -> TestData {
    TestData {
        data: vec![1, 5, 4, 7, 2, 10],
        expect: vec!(1, 2, 4, 5, 7, 10),
    }
}

pub fn get_list2() -> TestData {
    TestData {
        data: vec![2, 2, 2, 1, 2, 3],
        expect: vec![1, 2, 2, 2, 2, 3],

    }
}

pub fn get_list3() -> TestData {
    TestData {
        data: vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
        expect: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    }
}

pub fn get_list1u() -> TestDataU {
    TestDataU {
        data: vec![1, 5, 4, 7, 2, 10],
        expect: vec!(1, 2, 4, 5, 7, 10),
    }
}
