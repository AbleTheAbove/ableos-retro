# ableOS
## Set up

Install [Qemu](https://www.qemu.org/)

> On Windows be sure to add `C:\Program Files\qemu` to your `PATH` variable

`rustup component add rust-src`

`rustup component add llvm-tools-preview`

`cargo install bootimage`

## Roadmap
- [ ] File System implementation
- [ ] SRI Based user land syscall system
- [ ] AES Based encryption
- [ ] Advanced, feature-full multiseat implementation
- [ ] Make promise to provide full Unicode support
- [ ] Able Graphics Library
  - [ ] Rectangle
  - [ ] Circle

## Research && Resources
### File Systems
- [File System Design](http://web.cs.ucla.edu/classes/fall10/cs111/scribe/11a/)
- [Hackaday Article on file systems](https://hackaday.com/2019/01/24/cool-tools-a-little-filesystem-that-keeps-your-bits-on-lock/)