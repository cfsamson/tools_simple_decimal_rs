use std::fmt::Display;
use std::str::FromStr;

struct Decimal(f64,char);

impl Decimal {
  fn set_decimal_sign(&mut self, sign: char) {
    self.1 = sign;
  }
}

trait FromText {
  type Number;
  fn from_text(text: &str) -> Option<Self::Number>;
}

impl FromText for Decimal {
  type Number = Decimal;
  fn from_text(text: &str) -> Option<Self> {
    
    // trim empty space arount the number
    let text: &str = text.trim();

    // if the number contains whitespace then remove it
    let text: Vec<&str> = text.split_whitespace().collect();
    let text = text.concat();


    match f64::from_str(&text) {
      Ok(n) => Some(Decimal(n,'.')),
      Err(e) => {
        println!("{}", e);
        None
      }
    }
  }
}

impl Display for Decimal {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    // format f64 to a decimal string rounded to 2 decimal places
    let s: String = format!("{:.2}", self.0);

    // split the string in two parts, an integer (or exponent) part
    // and a fractional (or mantissa) part
    let s: Vec<&str> = s.split('.').collect();
    let integer = &s[0];
    let fractional = s.get(1).unwrap_or(&"00");

    // if there is no fractional or it's 0 we just set it to nothing
    let fractional = if self.0.fract() == 0.00 {
      "00"
    } else {
      fractional
    };

    // we focus on formatting the integer part
    // here we create a char-array from the string, reverse it and maps it to an array
    // of strings since we will be adding spaces to some char representations
    // 1234 => 4321 => 4 321
    let integer: Vec<String> = integer.chars().rev().enumerate().map(|(i, n)|{
      if i % 3 == 0 {
        format!(" {}",n)
      } else {
       n.to_string()
      }
    }).collect();

    // Remember, the string is now in reversed order, so we concatenate the 
    // strings to one string, then we split them to an array of
    // chars, reverse the order and collect them back to a string again.
    let integer: String = integer.concat().chars().rev().collect();

    
    write!(f, "{}{}{}", integer.trim_end(), &self.1, fractional)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn parses_and_displays_poorly_formatted_string() {
    let to_parse = " 1789 1   23.2245     \n";

    let decimal = Decimal::from_text(to_parse).unwrap();

    let formatted = format!("{}", decimal);
    assert_eq!("1 789 123.22".to_owned(), formatted);
  }

  #[test]
  fn parses_and_displays_with_custom_seperator() {
    let to_parse = " 1789 1   23.2245     \n";

    let mut decimal = Decimal::from_text(to_parse).unwrap();
    decimal.set_decimal_sign(',');

    let formatted = format!("{}", decimal);
    assert_eq!("1 789 123,22".to_owned(), formatted);
  }
}
