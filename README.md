# Lexical analyzer

Automated docs: https://ivanromero03.github.io/lexical-analyzer/lexical_analyzer/index.html

# Usage

```bash
cargo run
```

Sample:
```bash
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target\debug\lexical-analyzer.exe`
Enter a string to tokenize:
x 78 8.4 -55 size55 54RR if <= while x += *= /= -= !=
Error: No token found for input '54RR'
Tokens found:
<TOKEN: String("x") value: "x" (String): x>
<TOKEN: Int(78) value: "78" (Int): 78>
<TOKEN: Float(8.4) value: "8.4" (Float): 8.4>
<TOKEN: Int(-55) value: "-55" (Int): -55>
<TOKEN: String("size55") value: "size55" (String): size55>
<TOKEN: If value: "if">
<TOKEN: LessThanOrEqual value: "<=">
<TOKEN: While value: "while">
<TOKEN: String("x") value: "x" (String): x>
<TOKEN: PlusEquals value: "+=">
<TOKEN: MultiplyEquals value: "*=">
<TOKEN: DivideEquals value: "/=">
<TOKEN: MinusEquals value: "-=">
<TOKEN: NotEquals value: "!=">
```

# Features
- Tokenizes strings into various types of tokens.
- Handles integers, floats, strings, and keywords.
- Supports operators like `+=`, `-=`, `*=`, `/=`, and `!=`.
- Provides simple error handling for unrecognized tokens.
```
Enter a string to tokenize:
123a   
Error: No token found for input '123a'
Tokens found
```

# Lexical Analyzer
This is a simple lexical analyzer written in Rust that tokenizes input strings into various types of tokens, including integers, floats, strings, and keywords. It also supports operators like `+=`, `-=`, `*=`, `/=`, and `!=`.

# How to Run without Cargo/Rust
I prepared the build for Windows x64, linux x64, macOS aarch64/x86_64 in the `build` folder. You can run the binary directly without needing to install Rust or Cargo. 
```bash shell
# For Windows
cd build/x86_64-pc-windows-msvc/release
./lexical-analyzer.exe
# For Linux
cd build/x86_64-unknown-linux-gnu/release
./lexical-analyzer
# For macOS
cd build/aarch64-apple-darwin/release
./lexical-analyzer
# For macOS x86_64
cd build/x86_64-apple-darwin/release
./lexical-analyzer
```

## Cross Compilation
To cross-compile  for macOS from Linux check out this conversation:
https://github.com/rust-lang/rust/issues/112501#issuecomment-1682426620

