fn main() {
  let to_parse = "   789 1   23.2245     \n";

  let decimal = Decimal::from_text(to_parse).unwrap();

  println!("{}", decimal);

}

use std::fmt::Display;
use std::str::FromStr;

struct Decimal(f64);

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
      Ok(n) => Some(Decimal(n)),
      Err(e) => {
        println!("{}", e);
        None
      }
    }
  }
}

impl Display for Decimal {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let s: String = format!("{:.2}", self.0);
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
    let integer: Vec<String> = integer.chars().rev().enumerate().map(|(i, n)|{
      if i % 3 == 0 {
        format!(" {}",n)
      } else {
       n.to_string()
      }
    }).collect();

    // we concatenate the strings to one string, then we split them to an array of
    // chars, reverse the order and collect them back to a string again.
    let integer: String = integer.concat().chars().rev().collect();

    
    write!(f, "{},{}", integer.trim_end(), fractional)
  }
}
