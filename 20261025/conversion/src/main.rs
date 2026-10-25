use std::convert::From;
use std::convert::Into;
use std::convert::TryFrom;

fn main() {
    /*
    |==========================================================================
    | CONVERSION: From
    |==========================================================================
    | The `From` trait allows for a type to define how to create itself from
    | another type.
    | This can be made with primitive and common type of the language, even
    | our own types. Example, if we wnat a type like `Money` in our application
    | with a simple mechanism of conversion, we can do as:
    |
    |```rust
    |   #[derive(Debug)]
    |   struct Money {
    |       value: i64,
    |   }
    |```
    |
    | and implement the `From` trait in our struct `Money` returning itself:
    |
    |```rust
    |   impl From<i64> for Money {
    |       fn from(item: &i64) -> Self {
    |           return Money { value: item };
    |       }
    |   }
    |```
    |
    | Note use of Generics `From<i64>`, where we pass the type that field
    | `value` of `Money` struct needs to receive in our implementation.
    |
    | If we try pass a type unexpected the compiler let notice us:
    | - unsatisfied trait bound
    | Follow by help message on `From<i64>` implementation:
    | - for that trait implementation, expected `i64`, found `{float}`
    */

    /*
    |------------------------------------------------------------------------
    | Easy conversion from `str` to `String`
    | ------------------------------------------------------------------------
    */
    let message: &str = "Hello, world!";
    let string_message: String = String::from(message);
    println!("[conversion.from] str ({message}) to String ({string_message})");

    /*
    |------------------------------------------------------------------------
    | Defining a conversion for our own type
    | ------------------------------------------------------------------------
    */
    #[derive(Debug)]
    struct Money {
        value: i64,
    }

    impl From<i64> for Money {
        fn from(item: i64) -> Self {
            return Money { value: item };
        }
    }

    let price: Money = Money::from(25);
    println!(
        "[conversion.from] our own type, the price is {} | debug info: {:?}",
        price.value, price
    );

    /*
    |==========================================================================
    | CONVERSION: Into
    |==========================================================================
    | The `Into` trait is reciprocal of the `From` trait. It defines how to
    | convert a type into another type.
    | It's prefereable use `From` instead `Into`, once for ouwr type we gain
    | `Into` for free, avoiding orphan rule conflicts when converting foreign
    | types.
    |
    | Notes:
    | - Prefer use `Into` when we need to maximixing flexibility of callers
    | in a generic function.
    | - Both traits are for infallible conversions, if a conversion can fail,
    | use `TryFrom` and `TryInto`.
    |
    */

    /*
    |--------------------------------------------------------------------------
    | Defining Into implementation expliciting
    |--------------------------------------------------------------------------
    */
    #[derive(Debug)]
    struct Email {
        value: &'static str,
    }

    impl Into<Email> for &'static str {
        fn into(self) -> Email {
            return Email { value: self };
        }
    }

    let email_raw: &'static str = "johndoe@email.com";
    let email: Email = email_raw.into();
    println!(
        "[conversion.into] email raw: {} | email value: {} | debug info: {:?}",
        email_raw, email.value, email
    );

    /*
    |--------------------------------------------------------------------------
    | Using Into without implement it (interchangeable with From)
    |--------------------------------------------------------------------------
    */
    let balance_raw: i64 = 100;
    let balance_money: Money = balance_raw.into();
    println!(
        "[conversion.into] using into impliciting, balance raw: {} | balance value: {} | debug info: {:?}",
        balance_raw, balance_money.value, balance_money
    );

    #[derive(Debug)]
    struct Balance {
        value: i64,
    }

    /*
    |--------------------------------------------------------------------------
    | Using TryFrom for failable conversions
    |--------------------------------------------------------------------------
    */
    impl TryFrom<i64> for Balance {
        type Error = ();

        fn try_from(value: i64) -> Result<Self, Self::Error> {
            if value > 0 {
                return Ok(Balance { value });
            } else {
                return Err(());
            }
        }
    }
    let balance_input_negative: i64 = -1;
    let balance_negative: Result<Balance, ()> = balance_input_negative.try_into();
    println!("[conversion.tryfrom] balance negative: {:?}", balance_negative);

    let balance_input_positive: i64 = 25;
    let balance_positive: Result<Balance, ()> = balance_input_positive.try_into();
    println!(
        "[conversion.tryfrom] balance positive: {}",
        balance_positive.unwrap().value
    );
}
