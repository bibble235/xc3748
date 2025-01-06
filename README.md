# xc3748 driver

![xc3748 with speaker and FT232 Serial UART](xc3748.jpg?raw=true)

Serial Port Driver for xc3748

## Example

```rust
use std::time::Duration;

use xc3748::{Feedback, Xc3748, Xc3748Device};

fn main() {
    let port = serialport::new("/dev/ttyUSB0", 9600)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    let device = Xc3748Device::new(port);
    let mut xc3748 = Xc3748::new(Box::new(device), Feedback::OFF);
    xc3748.play();
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.