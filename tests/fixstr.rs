use fixstr::FixStr;

#[test]
fn test_basic_creation() {
    let s: FixStr<8> = FixStr::new("abc").unwrap();
    assert_eq!(s.as_str(), "abc");
    assert_eq!(s.len(), 3);
    assert_eq!(s.capacity(), 8);
}

#[test]
fn test_empty_string() {
    let s: FixStr<8> = FixStr::new("").unwrap();
    assert!(s.is_empty());
    assert_eq!(s.len(), 0);
}

#[test]
fn test_capacity_limits() {
    let ok: Option<FixStr<4>> = FixStr::new("abcd");
    assert!(ok.is_some());

    let too_long: Option<FixStr<4>> = FixStr::new("abcde");
    assert!(too_long.is_none());
}

#[test]
fn test_utf8_strings() {
    let s: FixStr<8> = FixStr::new("café").unwrap();
    assert_eq!(s.as_str(), "café");
    assert_eq!(s.len(), 5); // 'é' takes 2 octets
    assert_eq!(s.char_len(), 4);
}

#[test]
fn test_conversions() {
    let s: Result<FixStr<8>, _> = "hello".try_into();
    assert!(s.is_ok());

    let s: FixStr<8> = FixStr::new("world").unwrap();
    let string: String = s.into();
    assert_eq!(string, "world");
}

#[test]
#[should_panic(expected = "String 'too long' (len=8) exceeds capacity 2")]
fn test_unchecked_panics() {
    let _s: FixStr<2> = FixStr::new_unchecked("too long");
}

#[test]
fn test_default() {
    let s = FixStr::<8>::default();
    assert!(s.is_empty());
    assert!(s.to_string().is_empty());
}

#[test]
fn debug_string() {
    let s: FixStr<8> = FixStr::new("abc").unwrap();
    assert_eq!(format!("{:?}", s), "FixStr(\"abc\")");
}
