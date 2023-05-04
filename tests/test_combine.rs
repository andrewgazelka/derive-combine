use derive_combine::Combine;

#[derive(Combine, Eq, PartialEq, Debug)]
struct Abc {
    a: Option<u8>,
    b: Vec<u8>,
    d: u16,
}

#[test]
fn test_combine() {
    let mut abc = Abc {
        a: Some(1),
        b: vec![2],
        d: 3,
    };

    let other = Abc {
        a: None,
        b: vec![4],
        d: 5,
    };

    abc.combine(other);

    let expected = Abc {
        a: Some(1),
        b: vec![2, 4],
        d: 3,
    };

    assert_eq!(abc, expected);
}
