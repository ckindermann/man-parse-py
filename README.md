# man-parse-py

Parser for Manchester Syntax in Python.
Dependencies: `tree-sitter`, `tree-sitter-manchester`, `maturin`, `man-parse-rust`.

1. Project Setup
    1. `cargo install tree-sitter-cli`
    2. `git clone git@github.com:ckindermann/man-parse-py.git`
    3. `cd man-parse-py` 
    4. `git clone git@github.com:ckindermann/tree-sitter-manchester.git`
    5. `cd tree-sitter-manchester`
    6. `tree-sitter generate` (this builds the parser)
    7. `cd ..`
    8. `git clone git@github.com:ckindermann/man-parse-rust.git`
    9. `mv python_module.rs man-parse-rust/src/`
    10. `cd man-parse-rust`
    11. add the line `mod python_module;` to the end of file `src/lib.rs`
    12. modify `Cargo.toml` so that it contains the following lines:
    ```
    [lib]
    name="man_parse_rust"
    crate-type = ["cdylib", "lib"]

    [[bin]]
    name = "mybin"
    path = "src/main.rs"

    [dependencies.pyo3]
    version = "0.14.5"
    # "extension-module" tells pyo3 we want to build an extension module (skips linking against libpython.so)
    # "abi3-py36" tells pyo3 (and maturin) to build using the stable ABI with minimum Python version 3.6
    features = ["extension-module", "abi3-py36"]
    ```

2. Installing Maturin

    1. `python3 -m venv .venv`
    2. `source .venv/bin/activate`
    3. `pip install -U pip maturin`

3. Build
    1. `maturin develop` for local installation
    2. `maturin build` for creating a wheel

4. Test
    1. `cd ..`
    2. `python demo.py`


