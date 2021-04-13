# Microchip/Atmel ATSAM4E, ATSAM4N, ATSAM4S, and ATSAME5x PACs for Rust

This repository holds the Peripheral Access Crates (PAC) for various Microchip/Atmel `ATSAM` microcontrollers.

[![Build Status](https://github.com/atsam-rs/atsam-pac/workflows/Rust/badge.svg)](https://github.com/atsam-rs/atsam-pac/actions)

## ATSAM based MCUs currently supported by this repository

| Family   | Variant      | Version                                                                            |
|----------|--------------|------------------------------------------------------------------------------------|
| ATSAM4E  | ATSAM4E8C    | [1.6.1 (2018-05-07)](https://keilpack.azureedge.net/pack/Keil.SAM4_DFP.1.6.1.pack) |
|          | ATSAM4E8E    |                                                                                    |
|          | ATSAM4E16C   |                                                                                    |
|          | ATSAM4E16E   |                                                                                    |
| ATSAM4N  | ATSAM4N8A    | [1.6.1 (2018-05-07)](https://keilpack.azureedge.net/pack/Keil.SAM4_DFP.1.6.1.pack) |
|          | ATSAM4N8B    |                                                                                    |
|          | ATSAM4N8C    |                                                                                    |
|          | ATSAM4N16B   |                                                                                    |
|          | ATSAM4N16C   |                                                                                    |
| ATSAM4S  | ATSAM4S2A    | [1.6.1 (2018-05-07)](https://keilpack.azureedge.net/pack/Keil.SAM4_DFP.1.6.1.pack) |
|          | ATSAM4S2B    |                                                                                    |
|          | ATSAM4S2C    |                                                                                    |
|          | ATSAM4S4A    |                                                                                    |
|          | ATSAM4S4B    |                                                                                    |
|          | ATSAM4S4C    |                                                                                    |
|          | ATSAM4S8B    |                                                                                    |
|          | ATSAM4S8C    |                                                                                    |
|          | ATSAM4S16B   |                                                                                    |
|          | ATSAM4S16C   |                                                                                    |
|          | ATSAM4SA16B  |                                                                                    |
|          | ATSAM4SA16C  |                                                                                    |
|          | ATSAM4SD16B  |                                                                                    |
|          | ATSAM4SD16C  |                                                                                    |
|          | ATSAM4SD32B  |                                                                                    |
|          | ATSAM4SD32C  |                                                                                    |
|          | ATSAM4SP32A  |                                                                                    |
| ATSAME51 | ATSAME51G18A | [3.4.98 (2021-02-08)](https://packs.download.microchip.com)                        |
|          | ATSAME51G19A |                                                                                    |
|          | ATSAME51J18A |                                                                                    |
|          | ATSAME51J19A |                                                                                    |
|          | ATSAME51J20A |                                                                                    |
|          | ATSAME51N19A |                                                                                    |
|          | ATSAME51N20A |                                                                                    |
| ATSAME53 | ATSAME53J18A | [3.4.79 (2021-02-08)](https://packs.download.microchip.com)                        |
|          | ATSAME53J19A |                                                                                    |
|          | ATSAME53J20A |                                                                                    |
|          | ATSAME53N19A |                                                                                    |
|          | ATSAME53N20A |                                                                                    |
| ATSAME54 | ATSAME54N19A | [3.5.87 (2021-02-03)](https://packs.download.microchip.com)                        |
|          | ATSAME54N20A |                                                                                    |
|          | ATSAME54P19A |                                                                                    |
|          | ATSAME54P20A |                                                                                    |

## License

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
