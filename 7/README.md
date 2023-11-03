## Packages and Crates

### Crate Types in Rust

- **Binary Crates**
  - Compile to an executable.
  - Can be command-line programs or servers.
  - Must have a `main` function.
  - Defines what happens when the executable runs.

- **Library Crates**
  - Do not have a `main` function.
  - Do not compile to an executable.
  - Provide functionality for use in multiple projects.
  - Often referred to as a "library".
  - Example: `rand` crate for generating random numbers.

### Packages in Rust

- **Definition of a Package**
  - A bundle of one or more crates.
  - Provides a set of functionality.

- **Contents of a Package**
  - Includes a `Cargo.toml` file.
  - The `Cargo.toml` file describes how to build the crates within the package.

- **Example: Cargo**
  - Cargo itself is a package.
  - Contains the binary crate for the command-line tool (used to build code).
  - Also has a library crate that the binary crate depends on.
  
- **Usage of the Library Crate**
  - Other projects can depend on this library crate.
  - Allows projects to use the same logic as the Cargo command-line tool.
