use crate::periodic_table;

#[test]
fn test_length() {
    let elements = periodic_table();
    assert!(elements.len() > 0);
}

#[test]
fn test_hydrogen() {
    let elements = periodic_table();
    let hydrogen = elements.get(0).unwrap();
    assert_eq!("H", hydrogen.symbol);
    assert_eq!(1, hydrogen.atomic_number);
    assert_eq!("Hydrogen", hydrogen.name);
}
