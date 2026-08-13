# Core 1.0 compatibility authority

`c-abi-v1.baseline` is the reviewed 64-bit C ABI and PKSS identity baseline for
Core 1.0. `tools/check-abi-compatibility.sh` checks the public header digest,
every listed public type layout, the append-only executable-extension callback
table prefix, exported symbols, and an exact PKSS 1.0 wire vector.

`tools/check-api-compatibility.sh` runs pinned `cargo-semver-checks` against the
hash-pinned packaged predecessor from accepted final-candidate
requalification. It checks the normal/default Rust API only; the
`internal-testing` feature is intentionally not a product compatibility
contract.

Changing this baseline requires an explicit compatibility review. Removing a
symbol, changing an existing layout or wire value, or breaking an accepted
consumer is not repaired by editing the baseline. Compatible additions require
the appropriate ABI/protocol minor-version change and a newly accepted report.
