# man-parse-py

Parser for Manchester Syntax in Python.
Dependencies: `tree-sitter`, `tree-sitter-manchester`, `maturin`, `man-parse-rust`.

1. Project Setup: run `install.sh` 

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


