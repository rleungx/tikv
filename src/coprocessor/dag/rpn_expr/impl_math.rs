// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use cop_codegen::rpn_fn;
use crate::coprocessor::codec::data_type::*;

use crate::coprocessor::Result;

#[rpn_fn]
#[inline]
pub fn ceil_real(arg: &Option<Real>) -> Result<Option<Real>> {
    match arg {
        Some(arg) => {
            let f = arg.ceil();
            Ok(Some(Real::new(f).unwrap()))
        }
        None => Ok(None),
    }
}

// #[rpn_fn]
// #[inline]
// pub fn ceil_dec_to_int(arg: &Option<Decimal>) -> Result<Option<i64>> {
//      match arg {
//          Some(arg) = {
//             let d: Result<Decimal> = arg.ceil().into();
//          }
//     d.and_then(|dec| dec.as_i64_with_ctx(ctx)).map(Some)
// }

// #[rpn_fn]
// #[inline]
// pub fn ceil_dec_to_dec(arg: &Option<Decimal>) -> Result<Option<Decimal>> {
//     Ok(Some(arg.ceil()))
// }

#[rpn_fn]
#[inline]
pub fn ceil_int_to_int(arg: &Option<i64>) -> Result<Option<i64>> {
    Ok(*arg)
}


#[cfg(test)]
mod tests {
    use tipb::expression::ScalarFuncSig;
    use crate::coprocessor::codec::data_type::*;
    use crate::coprocessor::dag::rpn_expr::types::test_util::RpnFnScalarEvaluator;

    #[test]
    fn test_ceil_int_to_int() {
        let test_cases = vec![
            (ScalarValue::Int(Some(std::i64::MAX)), Some(std::i64::MAX)),
            (ScalarValue::Int(Some(std::i64::MIN)),  Some(std::i64::MIN)),
            (ScalarValue::Int(Some(std::u64::MAX as i64)),  Some(-1)),
            (ScalarValue::Int(Some(std::u64::MIN as i64)),  Some(0)),
        ];
        for (arg, expect_output) in test_cases {
            let output = RpnFnScalarEvaluator::new()
                .push_param(arg.clone())
                .evaluate(ScalarFuncSig::CeilIntToInt)
                .unwrap();
            assert_eq!(output, expect_output, "{:?}, {:?}", arg, ScalarFuncSig::CeilIntToInt);
        }
    }

    #[test]
    fn test_ceil_real() {
        let test_cases = vec![
            (ScalarValue::Real(Real::new(-3.45).ok()),ScalarFuncSig::CeilReal, Some(Real::new(-3f64).ok())),
            (ScalarValue::Real(Real::new(3.45).ok()),ScalarFuncSig::CeilReal, Some(Real::new(4f64).ok())),
        ];
        for (arg, sig, expect_output) in test_cases {
            let output = RpnFnScalarEvaluator::new()
                .push_param(arg.clone())
                .evaluate(sig)
                .unwrap();
            assert_eq!(output, expect_output, "{:?}, {:?}", arg, sig);
        }
    }
}
