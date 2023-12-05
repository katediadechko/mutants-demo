# mutants-demo

1. [Install Rust](https://www.rust-lang.org/tools/install).
2. Install testing tools.
   ```
   cargo install --locked cargo-tarpaulin cargo-mutants
   ```
3. Calculate coverage -> 100%.
   ```
   cargo tarpaulin
   ```
4. List available mutations.
   ```
   cargo mutants --list
   ```
5. Run mutation tests -> not all mutations are caught. 
   ```
   cargo mutants 
   ```
6. Uncomment line #12 in `src/math.rs`.
7. Run mutation tests again -> all mutations are caught.
   ```
   cargo mutants 
   ```
