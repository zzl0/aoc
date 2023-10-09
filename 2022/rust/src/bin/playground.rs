fn main() {
    use regex::Regex;

    let re = Regex::new(r"\$ cd (.*)|(\d+) .*|\$ ls|dir .*").unwrap();
    let mut locs = re.capture_locations();
    let m = re.captures_read(&mut locs, "123 abc").unwrap();
    assert_eq!(0..7, m.range());
    assert_eq!(Some((0, 7)), locs.get(0));
    assert_eq!(None, locs.get(1));
    assert_eq!(Some((0, 3)), locs.get(2));

    let mut locs = re.capture_locations();
    let m = re.captures_read(&mut locs, "$ cd abc").unwrap();
    assert_eq!(0..8, m.range());
    assert_eq!(Some((0, 8)), locs.get(0));
    assert_eq!(Some((5, 8)), locs.get(1));
    assert_eq!(None, locs.get(2));
}
