# rustcalc

rustcalc is a simple command-line calculator implemented in Rust. It performs basic arithmetic operations such as addition, subtraction, multiplication, and division on two numbers provided as command-line arguments.

## Usage

To use the calculator, open the ./target/debug folder and run the program with the following command:

```
./rustcalc <operation> <x> <y>
```

Replace `<operation>` with one of the following arithmetic operations:
- `a` for addition
- `s` for subtraction
- `m` for multiplication
- `d` for division

Replace `<x>` and `<y>` with the two numbers you want to operate on.

### Example

```
./rustcalc a 10 5
```

This will output:
```
Result: 15
```

## Error Handling

- If the program is run without providing the required arguments, it will display an error message indicating the correct usage.
- If parsing of `<x>` or `<y>` fails (e.g., if they are not valid numbers), an error message will be displayed, and the program will exit.
- Division by zero is not allowed. If either `<x>` or `<y>` is zero when performing division, an error message will be displayed, and the program will exit.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
