# Why learn Rust?

+ Rust memory management is handled by Rust without the need for a garbage collector
+ If your code compiles, it will run without error
+ Native cross-platform executables
+ Helps enforce consistency which supports governance and makes onboarding easier`

> rustup default nightly
> cargo expand
> cargo expand --bin rust_fundamentals --color=always --tests

## Stack

> last in first out

# data types

> isize and usize

> f32 and f64

Scalar vs Compound

| Scala Data Types                      | Compound Data Types                            |
|---------------------------------------|------------------------------------------------|
| Hold a single value                   | HOlds multiple value                           |
| Numbers                               | Array                                          |
| Characters                            | Tuple                                          |
| Booleans                              | String                                         |
| Array                                 | Tuple                                          |
| Multiple values of a single data type | Multiple values but can be different data type |

## Primitive Data types

> Data types that are build into the language and are stored on the stack

### Two's Compliment

invert all digits in the binary number and add one


| Character Byte Size | desc                                      |
|---------------------|-------------------------------------------|
| 1 Byte              | 255 characters in ASCII table             |
| 2 Bytes             | 65535 characters in Unicode-16 table      |
| 4 Bytes             | 4294967296 characters in Unicode-32 table |


Arrays and Tuples are very fast at runtime but are fixed size





