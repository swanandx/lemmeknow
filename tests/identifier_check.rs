#[test]
fn identify_works() {
    let identifier = lemmeknow::Identifier::default();
    let result = identifier.identify("UC11L3JDgDQMyH8iolKkVZ4w");
    assert_eq!(result[0].data.name, "YouTube Channel ID");
}

#[test]
fn boundaryless_and_min_rarity_works() {
    let identifier = lemmeknow::Identifier::default()
        .boundaryless(true)
        .min_rarity(0.6);
    let result = identifier.identify("abcthm{kgh}jk");
    assert_eq!(result[0].data.name, "TryHackMe Flag Format");
}

#[test]
fn bytes_identify() {
    let identifier = lemmeknow::bytes::Identifier::default();
    let data = [
        104, 116, 116, 112, 115, 58, 47, 47, 115, 119, 97, 110, 97, 110, 100, 120, 46, 103, 105,
        116, 104, 117, 98, 46, 105, 111,
    ];
    let result = identifier.identify(&data);
    assert_eq!(result[0].data.name, "Uniform Resource Locator (URL)")
}

#[test]
fn bytes_identify_boundaryless() {
    let identifier = lemmeknow::bytes::Identifier::default()
        .boundaryless(true)
        .min_rarity(0.6);
    let data = [
        115, 119, 97, 110, 104, 116, 98, 123, 97, 110, 100, 125, 100, 120,
    ];
    let result = identifier.identify(&data);
    assert_eq!(result[0].data.name, "HackTheBox Flag Format")
}
