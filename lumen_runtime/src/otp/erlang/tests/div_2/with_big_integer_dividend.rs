use super::*;

#[test]
fn with_atom_divisor_errors_badarith() {
    with_divisor_errors_badarith(|_| Term::str_to_atom("dividend", DoNotCare).unwrap());
}

#[test]
fn with_local_reference_divisor_errors_badarith() {
    with_divisor_errors_badarith(|mut process| Term::local_reference(&mut process));
}

#[test]
fn with_empty_list_divisor_errors_badarith() {
    with_divisor_errors_badarith(|_| Term::EMPTY_LIST);
}

#[test]
fn with_list_divisor_errors_badarith() {
    with_divisor_errors_badarith(|mut process| {
        Term::cons(
            0.into_process(&mut process),
            1.into_process(&mut process),
            &mut process,
        )
    });
}

#[test]
fn with_small_integer_zero_errors_badarith() {
    with(|dividend, mut process| {
        let divisor = 0.into_process(&mut process);

        assert_badarith!(erlang::div_2(dividend, divisor, &mut process))
    })
}

#[test]
fn with_small_integer_divisor_with_underflow_returns_small_integer() {
    with(|dividend, mut process| {
        let divisor: Term = 2.into_process(&mut process);

        assert_eq!(divisor.tag(), SmallInteger);

        let result = erlang::div_2(dividend, divisor, &mut process);

        assert!(result.is_ok());

        let quotient = result.unwrap();

        assert_eq!(quotient.tag(), SmallInteger);
    })
}

#[test]
fn with_big_integer_divisor_with_underflow_returns_small_integer() {
    with(|dividend, mut process| {
        let divisor = dividend;

        assert_eq!(divisor.tag(), Boxed);

        let unboxed_divisor: &Term = divisor.unbox_reference();

        assert_eq!(unboxed_divisor.tag(), BigInteger);

        let result = erlang::div_2(dividend, divisor, &mut process);

        assert!(result.is_ok());

        let quotient = result.unwrap();

        assert_eq!(quotient.tag(), SmallInteger);
        assert_eq!(quotient, 1.into_process(&mut process))
    })
}

#[test]
fn with_big_integer_divisor_without_underflow_returns_big_integer() {
    with_process(|mut process| {
        let divisor: Term = (crate::integer::small::MAX + 1).into_process(&mut process);

        assert_eq!(divisor.tag(), Boxed);

        let unboxed_divisor: &Term = divisor.unbox_reference();

        assert_eq!(unboxed_divisor.tag(), BigInteger);

        let dividend = erlang::multiply_2(divisor, divisor, &mut process).unwrap();

        let result = erlang::div_2(dividend, divisor, &mut process);

        assert!(result.is_ok());

        let quotient = result.unwrap();

        assert_eq!(quotient.tag(), Boxed);

        let unboxed_quotient: &Term = quotient.unbox_reference();

        assert_eq!(unboxed_quotient.tag(), BigInteger);
        assert_eq!(quotient, divisor);
    })
}

#[test]
fn with_float_zero_errors_badarith() {
    with(|dividend, mut process| {
        let divisor = 0.0.into_process(&mut process);

        assert_badarith!(erlang::div_2(dividend, divisor, &mut process))
    })
}

#[test]
fn with_float_divisor_errors_badarith() {
    with_divisor_errors_badarith(|mut process| 3.0.into_process(&mut process));
}

#[test]
fn with_local_pid_divisor_errors_badarith() {
    with_divisor_errors_badarith(|_| Term::local_pid(0, 1).unwrap());
}

#[test]
fn with_external_pid_divisor_errors_badarith() {
    with_divisor_errors_badarith(|mut process| Term::external_pid(1, 2, 3, &mut process).unwrap());
}

#[test]
fn with_tuple_divisor_errors_badarith() {
    with_divisor_errors_badarith(|mut process| Term::slice_to_tuple(&[], &mut process));
}

#[test]
fn with_map_is_divisor_errors_badarith() {
    with_divisor_errors_badarith(|mut process| Term::slice_to_map(&[], &mut process));
}

#[test]
fn with_heap_binary_divisor_errors_badarith() {
    with_divisor_errors_badarith(|mut process| Term::slice_to_binary(&[], &mut process));
}

#[test]
fn with_subbinary_divisor_errors_badarith() {
    with_divisor_errors_badarith(|mut process| {
        let original =
            Term::slice_to_binary(&[0b0000_00001, 0b1111_1110, 0b1010_1011], &mut process);
        Term::subbinary(original, 0, 7, 2, 1, &mut process)
    });
}

fn with<F>(f: F)
where
    F: FnOnce(Term, &mut Process) -> (),
{
    with_process(|mut process| {
        let dividend: Term = (crate::integer::small::MAX + 1).into_process(&mut process);

        assert_eq!(dividend.tag(), Boxed);

        let unboxed_dividend: &Term = dividend.unbox_reference();

        assert_eq!(unboxed_dividend.tag(), BigInteger);

        f(dividend, &mut process)
    })
}

fn with_divisor_errors_badarith<M>(divisor: M)
where
    M: FnOnce(&mut Process) -> Term,
{
    super::errors_badarith(|mut process| {
        let dividend: Term = (crate::integer::small::MAX + 1).into_process(&mut process);

        assert_eq!(dividend.tag(), Boxed);

        let unboxed_dividend: &Term = dividend.unbox_reference();

        assert_eq!(unboxed_dividend.tag(), BigInteger);

        let divisor = divisor(&mut process);

        erlang::div_2(dividend, divisor, &mut process)
    });
}
