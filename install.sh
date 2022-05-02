
#check whether tree-sitter is installed
if ! command -v tree-sitter &> /dev/null
then 
    cargo install tree-sitter-cli 
fi

echo "Fetch Python Repository"
git clone git@github.com:ckindermann/man-parse-py.git
cd man-parse-py

echo "Fetch Tree-Sitter Repository"
git clone git@github.com:ckindermann/tree-sitter-manchester.git
cd tree-sitter-manchester

echo "Build Manchester Parser"
tree-sitter generate

cd ..

echo "Fetch Rust Repository"
git clone git@github.com:ckindermann/man-parse-rust.git

mv python_module.rs man-parse-rust/src/
mv Cargo.toml man-parse-rust/
echo "mod python_module;" >> man-parse-rust/src/lib.rs 

