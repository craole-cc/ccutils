use super::ToNumeral;

pub fn test() {
  let integer = <num::BigInt as num::FromPrimitive>::from_u64(10).unwrap().pow(100);
  let fractional: usize = isize::MAX as usize;
  let sign = "-";
  let decimal_stringy = format!("{sign}{integer}.{fractional}");
  let decimal_numeric = 1;

  match decimal_stringy.to_numeral() {
    Ok(value) => {
      logline::info!("Decimal::from({}) => {:?} => {}", decimal_stringy, value, value);
    }
    Err(err) => {
      logline::error!("{}", err);
    }
  };

  match decimal_stringy.to_numeral() {
    Ok(value) => {
      logline::info!("to_numeral({}) {:?} => {}", decimal_stringy, value, value);
    }
    Err(err) => {
      logline::error!("{}", err);
    }
  };

  match decimal_numeric.to_numeral() {
    Ok(value) => {
      logline::info!("Decimal::from({}) => {:?} => {}", decimal_numeric, value, value);
    }
    Err(err) => {
      logline::error!("{}", err);
    }
  };

  match decimal_numeric.to_numeral() {
    Ok(value) => {
      logline::info!("to_numeral({}) {:?} => {}", decimal_numeric, value, value);
    }
    Err(err) => {
      logline::error!("{}", err);
    }
  };
}
