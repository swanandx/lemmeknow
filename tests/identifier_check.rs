use lemmeknow::Identify;
#[test]
fn identify_works() {
    let identifier = Identify::default();
    let result = identifier.identify("UC11L3JDgDQMyH8iolKkVZ4w");
    assert_eq!(result[0].data.name, "YouTube Channel ID");
}

#[test]
fn boundaryless_and_min_rarity_works() {
    let identifier = Identify::default().boundaryless(true).min_rarity(0.6);
    let result = identifier.identify("abcthm{kgh}jk");
    assert_eq!(result[0].data.name, "TryHackMe Flag Format");
}
