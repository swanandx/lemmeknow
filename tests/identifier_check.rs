use lemmeknow::Identifier;

#[test]
fn identify_works() {
    let identifier = Identifier::default();
    let result = identifier.identify("UC11L3JDgDQMyH8iolKkVZ4w");
    assert_eq!(result[0].data.name, "YouTube Channel ID");
}

#[test]
fn boundaryless_and_min_rarity_works() {
    let identifier = Identifier::default().boundaryless(true).min_rarity(0.6);
    let result = identifier.identify("abcthm{kgh}jk");
    assert_eq!(result[0].data.name, "TryHackMe Flag Format");
}
