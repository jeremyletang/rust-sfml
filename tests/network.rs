extern crate sfml;
use self::sfml::network::IpAddress;

#[test]
fn ip_to_string() {
    let ip = IpAddress::from_integer(101010);
    assert_eq!(ip.to_string(), "0.1.138.146");
}
