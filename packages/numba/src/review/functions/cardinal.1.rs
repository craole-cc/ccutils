use crate::{Currency, Language, num2words::Num2Err};
use num_bigfloat::BigFloat;

pub struct English {
  prefer_oh: bool,
  prefer_nil: bool
}

const UNITS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

const TENS: [&str; 9] = [
  "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"
];

const TEENS: [&str; 10] = [
  "ten",
  "eleven",
  "twelve",
  "thirteen",
  "fourteen",
  "fifteen",
  "sixteen",
  "seventeen",
  "eighteen",
  "nineteen"
];

// As defined by the AHD4, CED, RHD2, W3 and UM authorities
// For more information, see
// https://en.wikipedia.org/wiki/Names_of_large_numbers
const HUGE: [&str; 21] = [
  "thousand",
  "million",
  "billion",
  "trillion",
  "quadrillion",
  "quintillion",
  "sextillion",
  "septillion",
  "octillion",
  "nonillion",
  "decillion",
  "undecillion",
  "duodecillion",
  "tredecillion",
  "quattuordecillion",
  "quindecillion",
  "sexdecillion",
  "septendecillion",
  "octodecillion",
  "novemdecillion",
  "vigintillion"
];

impl English {
  pub fn new(prefer_oh: bool, prefer_nil: bool) -> Self {
    Self { prefer_oh, prefer_nil }
  }

  fn currencies(&self, currency: Currency, plural_form: bool) -> String {
    currency.default_string(plural_form)
  }

  fn cents(&self, currency: Currency, plural_form: bool) -> String {
    currency.default_subunit_string("cent{}", plural_form)
  }

  fn split_thousands(&self, mut num: BigFloat) -> Vec<u64> {
    let mut thousands = Vec::new();
    let bf_1000 = BigFloat::from(1000);

    while !num.is_zero() {
      thousands.push((num % bf_1000).to_u64().unwrap());
      num /= bf_1000;
    }

    thousands
  }

  fn int_to_cardinal(&self, mut num: BigFloat) -> Result<String, Num2Err> {
    // special case zero
    if num.is_zero() {
      return Ok(String::from(if self.prefer_oh {
        "oh"
      } else if self.prefer_nil {
        "nil"
      } else {
        "zero"
      }));
    }

    // handling negative values
    let mut words = vec![];
    if num.is_negative() {
      words.push(String::from("minus"));
      num = -num;
    }

    // iterate over thousands
    let mut first_elem = true;
    for (i, triplet) in self.split_thousands(num).iter().enumerate().rev() {
      let hundreds = (triplet / 100 % 10) as usize;
      let tens = (triplet / 10 % 10) as usize;
      let units = (triplet % 10) as usize;

      if hundreds > 0 {
        words.push(String::from(UNITS[hundreds - 1]));
        words.push(String::from("hundred"));
      }

      if tens != 0 || units != 0 {
        if i == 0 && !first_elem {
          words.push(String::from("and"));
        } else {
          first_elem = false;
        }

        match tens {
          0 => {
            // case 102 => [one hundred] two
            words.push(String::from(UNITS[units - 1]));
          }
          1 => {
            // case 112 => [one hundred] twelve
            words.push(String::from(TEENS[units]));
          }
          _ => {
            // case 142 => [one hundred] forty-two
            let ten: String = String::from(TENS[tens - 1]);
            words.push(match units {
              0 => ten,
              _ => format!("{}-{}", ten, UNITS[units - 1])
            });
          }
        }
      }

      if i != 0 && triplet != &0 {
        if i > HUGE.len() {
          return Err(Num2Err::CannotConvert);
        }
        words.push(String::from(HUGE[i - 1]));
      }
    }

    Ok(words.join(" "))
  }

  fn float_to_cardinal(&self, num: BigFloat) -> Result<String, Num2Err> {
    let integral_part = num.int();
    let mut words: Vec<String> = vec![];

    if !integral_part.is_zero() {
      let integral_word = self.int_to_cardinal(integral_part)?;
      words.push(integral_word);
    }

    let mut ordinal_part = num.frac();
    if !ordinal_part.is_zero() {
      words.push(String::from("point"));
    }
    while !ordinal_part.is_zero() {
      let digit = (ordinal_part * BigFloat::from(10)).int();
      ordinal_part = (ordinal_part * BigFloat::from(10)).frac();
      words.push(match digit.to_u64().unwrap() {
        0 => String::from(if self.prefer_oh { "oh" } else { "zero" }),
        i => String::from(UNITS[i as usize - 1])
      });
    }
    Ok(words.join(" "))
  }
}

impl Language for English {
  fn to_cardinal(&self, num: BigFloat) -> Result<String, Num2Err> {
    if num.is_inf_pos() {
      Ok(String::from("infinity"))
    } else if num.is_inf_neg() {
      Ok(String::from("minus infinity"))
    } else if num.frac().is_zero() {
      self.int_to_cardinal(num)
    } else {
      self.float_to_cardinal(num)
    }
  }

  fn to_ordinal(&self, num: BigFloat) -> Result<String, Num2Err> {
    let cardinal_word = self.to_cardinal(num)?;

    let mut words: Vec<String> = vec![];
    let mut split = cardinal_word.split_whitespace().peekable();

    while let Some(w) = split.next() {
      if split.peek().is_some() {
        // not last word, no modification needed
        words.push(String::from(w));
      } else {
        // last word, needs to be processed
        let mut prefix = String::from("");
        let mut suffix = String::from(w);

        if w.contains('-') {
          // e.g. forty-two => forty-second
          let mut w_split = w.split('-');

          if let Some(pre) = w_split.next() {
            prefix = format!("{}-", pre);
          }

          if let Some(suf) = w_split.next() {
            suffix = String::from(suf);
          }
        }

        suffix = match suffix.as_str() {
          "one" => String::from("first"),
          "two" => String::from("second"),
          "three" => String::from("third"),
          "four" => String::from("fourth"),
          "five" => String::from("fifth"),
          "six" => String::from("sixth"),
          "seven" => String::from("seventh"),
          "eight" => String::from("eighth"),
          "nine" => String::from("ninth"),
          "ten" => String::from("tenth"),
          "eleven" => String::from("eleventh"),
          "twelve" => String::from("twelfth"),
          _ =>
            if suffix.ends_with('y') {
              format!("{}ieth", &suffix[..suffix.len() - 1])
            } else {
              format!("{}th", suffix)
            },
        };

        words.push(format!("{}{}", prefix, suffix))
      }
    }

    Ok(words.join(" "))
  }

  fn to_ordinal_num(&self, num: BigFloat) -> Result<String, Num2Err> {
    let tail = (num % BigFloat::from(100)).to_u64().unwrap();
    let last = tail % 10;
    Ok(format!(
      "{}{}",
      num.to_u128().unwrap(),
      match (tail / 10 != 1, last) {
        (true, 1) => "st",
        (true, 2) => "nd",
        (true, 3) => "rd",
        _ => "th"
      }
    ))
  }

  fn to_year(&self, num: BigFloat) -> Result<String, Num2Err> {
    if !num.frac().is_zero() {
      return Err(Num2Err::FloatingYear);
    }

    let mut num = num;

    let mut suffix = "";
    if num.is_negative() {
      num = num.inv_sign();
      suffix = " BC";
    }

    let bf_100 = BigFloat::from(100);

    let (high, low) = ((num / bf_100).to_i64().unwrap(), (num % bf_100).to_i64().unwrap());
    let year_word = if high == 0 || (high % 10 == 0 && low < 10) || high >= 100 {
      // if year is 00XX, X00X, or beyond 9999, go cardinal
      self.int_to_cardinal(num)?
    } else {
      let high_word = self.int_to_cardinal(BigFloat::from(high))?;
      let low_word = if low == 0 {
        String::from("hundred")
      } else if low < 10 {
        format!("oh-{}", self.int_to_cardinal(BigFloat::from(low))?)
      } else {
        self.int_to_cardinal(BigFloat::from(low))?
      };

      format!("{} {}", high_word, low_word)
    };

    Ok(format!("{}{}", year_word, suffix))
  }

  fn to_currency(&self, num: BigFloat, currency: Currency) -> Result<String, Num2Err> {
    if num.is_inf() {
      Ok(format!(
        "{}an infinity of {}",
        if num.is_negative() { "minus " } else { "" },
        self.currencies(currency, true)
      ))
    } else if num.frac().is_zero() {
      let words = self.int_to_cardinal(num)?;
      Ok(format!(
        "{} {}",
        words,
        self.currencies(currency, num != BigFloat::from(1))
      ))
    } else {
      let integral_part = num.int();
      let cents_nb = (num * BigFloat::from(100)).int() % BigFloat::from(100);
      let cents_words = self.int_to_cardinal(cents_nb)?;
      let cents_suffix = self.cents(currency, cents_nb != BigFloat::from(1));
      let integral_word = self.to_currency(integral_part, currency)?;

      if cents_nb.is_zero() {
        Ok(integral_word)
      } else if integral_part.is_zero() {
        Ok(format!("{} {}", cents_words, cents_suffix))
      } else {
        Ok(format!("{} and {} {}", integral_word, cents_words, cents_suffix))
      }
    }
  }
}
