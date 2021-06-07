# ableOS
![Discord](https://img.shields.io/discord/849794982566559754) ![Code Size](https://img.shields.io/github/languages/code-size/abletheabove/ableos)
## Set up
Install [Qemu](https://www.qemu.org/)

> On Windows be sure to add `C:\Program Files\qemu` to your `PATH` variable

`rustup component add rust-src`

`rustup component add llvm-tools-preview`

`cargo install bootimage`


## Testing on real hardware
I recommend using an old x86_64 computer
* `cargo run --release` to generate a binary image that is bootable
* flash it to a USB device via `dd` or belenaEtcher
* Remove said USB device and plug into test machine
* assure test machine boots from USB devices
