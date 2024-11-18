# Temperature Converter

## Overview
Simple rust program that accepts a temperature in celsius or fahrenheit and then converts it to the opposite format.

## Get started

Note: you need to have rust installed to build and run this program.

Build an run the dev version:
```bash
cargo run
```

Create production version:
```bash
cargo build --release
```

Run prod. binary version:
```bash
./target/debug/temp_converter
```

## Ideas

1. Support multiple temperature conversions
1. Improve error handling
1. Track how many times current user has run the program (including persisting count after program terminates)