pub struct A {
    pub name: String,
    pub age: u32,
}

pub fn make_a(f: impl Fn() -> u32) -> A {
    A {
        name: "frank".to_string(),
        age: f(),
    }
}

#[test]
fn test_a() {
    fn get_age() -> u32 {
        42
    }

    let a = make_a(get_age);
    assert!(a.age == 42);
    assert_eq!(a.name, "frank");
}

#[test]
#[ignore]
#[should_panic]
fn test_a_panic() {
    fn explode() -> u32 {
        panic!()
    }

    let _a = make_a(explode);
}
