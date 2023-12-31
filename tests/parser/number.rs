use crate::invalid_assert;
use crate::json_assert;

#[test]
fn simple_number() {
    json_assert!("0", Number(0.0));
    json_assert!("-0", Number(0.0));
    json_assert!("-0.0", Number(0.0));
    json_assert!("1", Number(1.0));
    json_assert!("-1", Number(-1.0));
    json_assert!("1.5", Number(1.5));
    json_assert!("-1.5", Number(-1.5));
    json_assert!("3.1416", Number(3.1416));
}

#[test]
fn parse_num_with_exp() {
    json_assert!("1E10", Number(1E10));
    json_assert!("1e10", Number(1e10));
    json_assert!("1E+10", Number(1E+10));
    json_assert!("1E-10", Number(1E-10));
    json_assert!("-1E10", Number(-1E10));
    json_assert!("-1e10", Number(-1e10));
    json_assert!("-1E+10", Number(-1E+10));
    json_assert!("-1E-10", Number(-1E-10));
    json_assert!("1.234E+10", Number(1.234E+10));
    json_assert!("1.234E-10", Number(1.234E-10));
}

#[test]
fn parse_num_edge_case() {
    json_assert!("1e-10000", Number(0.0)); /* must underflow */
    json_assert!("1.0000000000000002", Number(1.0000000000000002)); /* the smallest number > 1 */
    json_assert!("4.9406564584124654e-324", Number(4.9406564584124654e-324)); /* minimum denormal */
    json_assert!("-4.9406564584124654e-324", Number(-4.9406564584124654e-324));
    json_assert!("2.2250738585072009e-308", Number(2.2250738585072009e-308)); /* Max subnormal double */
    json_assert!("-2.2250738585072009e-308", Number(-2.2250738585072009e-308));
    json_assert!("2.2250738585072014e-308", Number(2.2250738585072014e-308)); /* Min normal positive double */
    json_assert!("-2.2250738585072014e-308", Number(-2.2250738585072014e-308));
    json_assert!("1.7976931348623157e+308", Number(1.7976931348623157e+308)); /* Max double */
    json_assert!("-1.7976931348623157e+308", Number(-1.7976931348623157e+308));
}

#[test]
fn parse_invalid_num() {
    /* invalid number */
    invalid_assert!("+0", InvalidValue);
    invalid_assert!("+1", InvalidValue);
    invalid_assert!(".123", InvalidValue); /* at least one digit before '.' */
    invalid_assert!("1.", InvalidValue); /* at least one digit after '.' */
    invalid_assert!("0.", InvalidValue); /* at least one digit after '.' */
    invalid_assert!("INF", InvalidValue);
    invalid_assert!("inf", InvalidValue);
    invalid_assert!("NAN", InvalidValue);
    invalid_assert!("nan", InvalidValue);
    invalid_assert!("-", InvalidValue);
    invalid_assert!("1ee", InvalidValue); /* bad exp field */
    invalid_assert!("1e", InvalidValue); /* bad exp field */
}

#[test]
fn parse_large_num() {
    invalid_assert!("1e309", NumberTooBig);
    // invalid_assert!("1e-618", NumberTooBig); // this should underflow
}
