// testsディレクトリでは結合テストができる

use test_sample;

#[test]
fn it_adds_two() {
    assert_eq!(4, test_sample::add(2, 2));
}
