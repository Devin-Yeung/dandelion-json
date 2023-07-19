use crate::invalid_assert;

#[test]
fn not_singular_root() {
    invalid_assert!("null x", RootNotSingular);
    invalid_assert!("1u10", RootNotSingular); /* bad exp field */
    invalid_assert!("0123", RootNotSingular); /* after zero should be '.' or nothing */
    invalid_assert!("0x0", RootNotSingular); /* after zero should be '.' or nothing */
    invalid_assert!("0x123", RootNotSingular); /* after zero should be '.' or nothing */
    invalid_assert!("001", RootNotSingular); /* after zero should be '.' or nothing */
    invalid_assert!("00.1", RootNotSingular); /* after zero should be '.' or nothing */
}
