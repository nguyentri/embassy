
# rust_ra4m1

This repository provides an example project for developing embedded software for the **Arduino UNO R4 WiFi** (based on the **RA4M1** microcontroller) using **Rust**.

## Prerequisites

### 1. Install Rust
Ensure you have **Rust** installed on your system. Follow the official installation guide:
- [Rust Installation](https://www.rust-lang.org/tools/install)

### 2. Install ARM Toolchain
Install the **ARM GCC toolchain** which includes the necessary tools like `objcopy` for converting ELF files to `.bin` and `.hex`:
- **Windows**: [GNU Toolchain for Windows](https://developer.arm.com/downloads/-/gnu-toolchain-downloads)
- **Linux**:
  ```bash
  sudo apt-get install gcc-arm-none-eabi
  ```

### 3. Install Required Rust Components
Install the **LLVM tools** and **cargo-binutils** for further operations:
```bash
rustup component add llvm-tools
cargo install cargo-binutils
```

### 4. Install Probe-RS for Flashing and Debugging
`cargo-embed` is now part of **probe-rs**. To install the required tools, run:
```bash
irm https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.ps1 | iex
```

---

## Setting Up the Project

### 1. Create a New Rust Project
```bash
cargo new rust_ra4m1
cd rust_ra4m1
```

### 2. Add Dependencies
Add necessary dependencies to `Cargo.toml` for Cortex-M development:

```toml
[dependencies]
cortex-m = { version = "0.7.7", features = ["critical-section-single-core"] }
cortex-m-rt = "0.7.5"
panic-halt = "1.0.0"
rtt-target = "0.6.1"  # For RTT output
```

### 3. Set Up `.cargo/config.toml`
Add the following to configure the target architecture for RA4M1:
```toml
[build]
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
rustflags = [
  "-C", "link-arg=-Tlink.x",
]
```

---

## Building and Flashing the Application

### 1. Build the Project
To compile the project for the RA4M1, run:
```bash
cargo build --release
```

### 2. Convert ELF to .bin and .hex
After building the project, use `objcopy` to convert the ELF file into a `.bin` or `.hex` file:

**Convert to `.hex`:**
```bash
cargo objcopy --release -- -O ihex target/thumbv7em-none-eabihf/release/rust_ra4m1 rust_ra4m1.hex
```

**Convert to `.bin`:**
```bash
cargo objcopy --release -- -O binary target/thumbv7em-none-eabihf/release/rust_ra4m1 rust_ra4m1.bin
```

### 3. Flash the Microcontroller
Once the files are generated, you can flash the RA4M1 using **cargo embed** (make sure the target is available in the `probe-rs` database):

```bash
cargo embed --chip R7FA4M1AB
```

This will erase and program the microcontroller with the ELF file.

---

## Troubleshooting

### Chip Not Found Error
If you encounter the error:
```text
The chip 'RA4M1' was not found in the database.
```
It means **RA4M1** is not supported in the `probe-rs` toolset. You can:
1. Check for the device in the [probe-rs chip registry](https://probe.rs/docs/tools/debugger/).
2. Add a custom chip definition in the `probe-rs` toolset or use another debugger like **OpenOCD** or **J-Link**.

### Unresolved Import Errors (e.g., `rtt_target`)
If you get errors like `unresolved import 'rtt_target'`, ensure that `rtt-target` is added to `Cargo.toml` and that you’re using the correct syntax in `main.rs`:
```rust
use rtt_target::{rprintln, rtt_init_print};

fn main() {
    rtt_init_print!();
    rprintln!("Hello, world!");
    loop {}
}
```

---

## Additional Resources

- **Probe-rs Documentation**: [https://probe.rs/docs/](https://probe.rs/docs/)
- **Embedded Rust Book**: [https://docs.rust-embedded.org/book/](https://docs.rust-embedded.org/book/)
- **Arduino UNO R4 WiFi** Documentation: Check the [official Arduino website](https://www.arduino.cc/) for detailed hardware specifications.

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
