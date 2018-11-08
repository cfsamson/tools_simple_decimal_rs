# Parsing and displaying decimals

## Parsing and displaying
This crate parses strings to decimals, and is flexible regarding \
the input string. 

>Primary use should be quick parsing of text for display/presentation where absolute correctness of the rounding operation is not needed, i.e. scraping information from the web, or presenting larger numbers. 

Right now this is a binary crate so it serves more than a code example than a library. I will convert it to a library in time.

## Usage
The crate exposes a Decimal type that implements a trait called #from_text for. The Decimal type is a tuple type that only holds a f64 number.

Decimal also has an implementation for Display.

Example:
```rust
  let to_parse = "   789 1   23.2245     \n";

  let decimal = Decimal::from_text(to_parse).unwrap();

  println!("{}", decimal); // 789 123,22
```

It uses "," as decimal seperator as we usually do in Norway.

NB! the rounding for display is not accurate. As you see from the example above the "matissa" (or fractional) is .2245 - the expected way to round this would be .23, but we only round the last digit "4" down as displayed here.

## Todo
- [x] Make decimal seperator and 1000 seperator configurable
- [ ] Round with better precision
- [ ] Change from f64 to BigDecimal for better calculations