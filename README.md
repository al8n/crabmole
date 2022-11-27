<div align="center">
<h1>Crabmole</h1>
</div>
<div align="center">

<p align="center">
  <img src="https://raw.githubusercontent.com/al8n/crabmole/main/art/crabmole.jpg" width="40%" height="auto" />
</p>

Porting some Go standard libraries in Rust

**Note:** This crate will not port all go standard libraries in Rust, but some missing libraries in Rust environment.

[<img alt="github" src="https://img.shields.io/badge/GITHUB-crabmole-8da0cb?style=for-the-badge&logo=Github" height="22">][Github-url]
[<img alt="Build" src="https://img.shields.io/github/workflow/status/al8n/crabmole/CI/main?logo=Github-Actions&style=for-the-badge" height="22">][CI-url]
[<img alt="codecov" src="https://img.shields.io/codecov/c/gh/al8n/crabmole?style=for-the-badge&token=P175Q03Q1L&logo=codecov" height="22">][codecov-url]

[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-crabmole-66c2a5?style=for-the-badge&labelColor=555555&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">][doc-url]
[<img alt="crates.io" src="https://img.shields.io/crates/v/crabmole?style=for-the-badge&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iaXNvLTg4NTktMSI/Pg0KPCEtLSBHZW5lcmF0b3I6IEFkb2JlIElsbHVzdHJhdG9yIDE5LjAuMCwgU1ZHIEV4cG9ydCBQbHVnLUluIC4gU1ZHIFZlcnNpb246IDYuMDAgQnVpbGQgMCkgIC0tPg0KPHN2ZyB2ZXJzaW9uPSIxLjEiIGlkPSJMYXllcl8xIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHhtbG5zOnhsaW5rPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5L3hsaW5rIiB4PSIwcHgiIHk9IjBweCINCgkgdmlld0JveD0iMCAwIDUxMiA1MTIiIHhtbDpzcGFjZT0icHJlc2VydmUiPg0KPGc+DQoJPGc+DQoJCTxwYXRoIGQ9Ik0yNTYsMEwzMS41MjgsMTEyLjIzNnYyODcuNTI4TDI1Niw1MTJsMjI0LjQ3Mi0xMTIuMjM2VjExMi4yMzZMMjU2LDB6IE0yMzQuMjc3LDQ1Mi41NjRMNzQuOTc0LDM3Mi45MTNWMTYwLjgxDQoJCQlsMTU5LjMwMyw3OS42NTFWNDUyLjU2NHogTTEwMS44MjYsMTI1LjY2MkwyNTYsNDguNTc2bDE1NC4xNzQsNzcuMDg3TDI1NiwyMDIuNzQ5TDEwMS44MjYsMTI1LjY2MnogTTQzNy4wMjYsMzcyLjkxMw0KCQkJbC0xNTkuMzAzLDc5LjY1MVYyNDAuNDYxbDE1OS4zMDMtNzkuNjUxVjM3Mi45MTN6IiBmaWxsPSIjRkZGIi8+DQoJPC9nPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPC9zdmc+DQo=" height="22">][crates-url]
[<img alt="crates.io" src="https://img.shields.io/crates/d/crabmole?color=critical&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBzdGFuZGFsb25lPSJubyI/PjwhRE9DVFlQRSBzdmcgUFVCTElDICItLy9XM0MvL0RURCBTVkcgMS4xLy9FTiIgImh0dHA6Ly93d3cudzMub3JnL0dyYXBoaWNzL1NWRy8xLjEvRFREL3N2ZzExLmR0ZCI+PHN2ZyB0PSIxNjQ1MTE3MzMyOTU5IiBjbGFzcz0iaWNvbiIgdmlld0JveD0iMCAwIDEwMjQgMTAyNCIgdmVyc2lvbj0iMS4xIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHAtaWQ9IjM0MjEiIGRhdGEtc3BtLWFuY2hvci1pZD0iYTMxM3guNzc4MTA2OS4wLmkzIiB3aWR0aD0iNDgiIGhlaWdodD0iNDgiIHhtbG5zOnhsaW5rPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5L3hsaW5rIj48ZGVmcz48c3R5bGUgdHlwZT0idGV4dC9jc3MiPjwvc3R5bGU+PC9kZWZzPjxwYXRoIGQ9Ik00NjkuMzEyIDU3MC4yNHYtMjU2aDg1LjM3NnYyNTZoMTI4TDUxMiA3NTYuMjg4IDM0MS4zMTIgNTcwLjI0aDEyOHpNMTAyNCA2NDAuMTI4QzEwMjQgNzgyLjkxMiA5MTkuODcyIDg5NiA3ODcuNjQ4IDg5NmgtNTEyQzEyMy45MDQgODk2IDAgNzYxLjYgMCA1OTcuNTA0IDAgNDUxLjk2OCA5NC42NTYgMzMxLjUyIDIyNi40MzIgMzAyLjk3NiAyODQuMTYgMTk1LjQ1NiAzOTEuODA4IDEyOCA1MTIgMTI4YzE1Mi4zMiAwIDI4Mi4xMTIgMTA4LjQxNiAzMjMuMzkyIDI2MS4xMkM5NDEuODg4IDQxMy40NCAxMDI0IDUxOS4wNCAxMDI0IDY0MC4xOTJ6IG0tMjU5LjItMjA1LjMxMmMtMjQuNDQ4LTEyOS4wMjQtMTI4Ljg5Ni0yMjIuNzItMjUyLjgtMjIyLjcyLTk3LjI4IDAtMTgzLjA0IDU3LjM0NC0yMjQuNjQgMTQ3LjQ1NmwtOS4yOCAyMC4yMjQtMjAuOTI4IDIuOTQ0Yy0xMDMuMzYgMTQuNC0xNzguMzY4IDEwNC4zMi0xNzguMzY4IDIxNC43MiAwIDExNy45NTIgODguODMyIDIxNC40IDE5Ni45MjggMjE0LjRoNTEyYzg4LjMyIDAgMTU3LjUwNC03NS4xMzYgMTU3LjUwNC0xNzEuNzEyIDAtODguMDY0LTY1LjkyLTE2NC45MjgtMTQ0Ljk2LTE3MS43NzZsLTI5LjUwNC0yLjU2LTUuODg4LTMwLjk3NnoiIGZpbGw9IiNmZmZmZmYiIHAtaWQ9IjM0MjIiIGRhdGEtc3BtLWFuY2hvci1pZD0iYTMxM3guNzc4MTA2OS4wLmkwIiBjbGFzcz0iIj48L3BhdGg+PC9zdmc+&style=for-the-badge" height="22">][crates-url]

https://img.shields.io/badge/dynamic/json?label=coverage&style=for-the-badge&color=success&logo=codecov&query=%24.totals.coverage&url=https%3A%2F%2Fcodecov.io%2Fapi%2Fv2%2Fgithub%2Fal8n%2Frepos%2Fcrabmole%2Freport%2F%3Fbranch%3Dmain%26path%3Dsrc%252Fencoding%252Fbase32.rs

</div>

## Installation
```toml
crabmole = "0.0.1"
```

## Status
| name | status | no_std  |  100% safe  | code coverage |
|:----:|:------:|:-------:|:-----------:|:-------------:|
| `container/ring` | 🚧 | ✅ | ❌ | |
| `sort` |   ✅   |    ✅    | ✅ | [<img alt="github" src="https://img.shields.io/badge/dynamic/json?label=&style=for-the-badge&color=success&query=%24.totals.coverage&url=https%3A%2F%2Fcodecov.io%2Fapi%2Fv2%2Fgithub%2Fal8n%2Frepos%2Fcrabmole%2Freport%2F%3Fbranch%3Dmain%26path%3Dsrc%252Fsort.rs" height="22">](https://app.codecov.io/gh/al8n/crabmole/blob/main/src/sort.rs) |
| `encoding/ascii85` | ✅ | ✅ | ✅ | |
| `encoding/base32` | ✅ | ✅ | ✅ | [<img alt="github" src="https://img.shields.io/badge/dynamic/json?label=&style=for-the-badge&color=success&query=%24.totals.coverage&url=https%3A%2F%2Fcodecov.io%2Fapi%2Fv2%2Fgithub%2Fal8n%2Frepos%2Fcrabmole%2Freport%2F%3Fbranch%3Dmain%26path%3Dsrc%252Fencoding%252Fbase32.rs" height="22">](https://app.codecov.io/gh/al8n/crabmole/blob/main/src/encoding/base32.rs) |
| `encoding/base64` | 🚧 | ✅ | ✅ | [<img alt="github" src="https://img.shields.io/badge/dynamic/json?label=&style=for-the-badge&color=success&query=%24.totals.coverage&url=https%3A%2F%2Fcodecov.io%2Fapi%2Fv2%2Fgithub%2Fal8n%2Frepos%2Fcrabmole%2Freport%2F%3Fbranch%3Dmain%26path%3Dsrc%252Fencoding%252Fbase64.rs" height="22">](https://app.codecov.io/gh/al8n/crabmole/blob/main/src/encoding/base64.rs) |
| `encoding/binary` | 🚧 | ✅ | ✅ | |
| `encoding/hex` | ✅ | ✅ | ✅ | |
| `encoding/pem` | ✅ | ✅ | ✅ | |
| `io/pipe` | ✅ | ❌ | ✅ | [<img alt="github" src="https://img.shields.io/badge/dynamic/json?label=&style=for-the-badge&color=success&query=%24.totals.coverage&url=https%3A%2F%2Fcodecov.io%2Fapi%2Fv2%2Fgithub%2Fal8n%2Frepos%2Fcrabmole%2Freport%2F%3Fbranch%3Dmain%26path%3Dsrc%252Fio%252Fpipe.rs" height="22">](https://app.codecov.io/gh/al8n/crabmole/blob/main/src/io/pipe.rs) |
| `async-io/pipe` | ✅ | ❌ | ✅ | [<img alt="github" src="https://img.shields.io/badge/dynamic/json?label=&style=for-the-badge&color=success&query=%24.totals.coverage&url=https%3A%2F%2Fcodecov.io%2Fapi%2Fv2%2Fgithub%2Fal8n%2Frepos%2Fcrabmole%2Freport%2F%3Fbranch%3Dmain%26path%3Dsrc%252Fio%252Fasync_pipe.rs" height="22">](https://app.codecov.io/gh/al8n/crabmole/blob/main/src/io/async_pipe.rs) |
| `time` | - | ❌ | ✅ | |

- 🚧: WIP
- ✅: Support
- ❌: Not Support
- `-`: Not start



## Acknowledgements
- Thanks Go std libraries developers.

#### License

<sup>
Licensed under either of <a href="https://opensource.org/licenses/Apache-2.0">Apache License, Version
2.0</a> or <a href="https://opensource.org/licenses/MIT">MIT license</a> or <a href="https://opensource.org/licenses/BSD-3-Clause">BSD-3-Clause license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0, MIT, BSD-3-Clause license,
shall be triple licensed as above, without any additional terms or conditions.
</sub>

[Github-url]: https://github.com/al8n/crabmole/
[CI-url]: https://github.com/al8n/crabmole/actions/workflows/ci.yml
[doc-url]: https://docs.rs/crabmole
[crates-url]: https://crates.io/crates/crabmole
[codecov-url]: https://app.codecov.io/gh/al8n/crabmole/
[license-url]: https://opensource.org/licenses/Apache-2.0
[rustc-url]: https://github.com/rust-lang/rust/blob/master/RELEASES.md
[license-apache-url]: https://opensource.org/licenses/Apache-2.0
[license-mit-url]: https://opensource.org/licenses/MIT
[rustc-image]: https://img.shields.io/badge/rustc-1.52.0--nightly%2B-orange.svg?style=for-the-badge&logo=Rust