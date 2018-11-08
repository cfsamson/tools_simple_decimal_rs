# Parsing and displaying decimals

## Parsing and displaying
This crate parses strings litterals to decimals, and is flexible regarding \
the input string. 

>Primary use should be quick parsing of text for display/presentation where absolute correctness of the rounding operation is not needed, i.e. scraping information from the web, or presenting larger numbers. 


## Usage
The crate exposes a Decimal type that implements a trait called #from_text for. The Decimal type is a tuple type that only holds a f64 number.

Decimal also has an implementation for Display.

Example:
```rust
  let to_parse = "   789 1   23.2245     \n";

  let decimal = Decimal::from_text(to_parse).unwrap();

  println!("{}", decimal); // 789 123,22
```

It uses "." as a default seperator. To set a custom one, do this:

```rust
let mut decimal = Decimal::from_text("18292.50").unwrap();
decimal.set_decimal_sign(',');
```

## Todo
- [x] Make decimal seperator and 1000 seperator configurable
- [ ] Round with better precision
- [ ] Change from f64 to BigDecimal for better calculations