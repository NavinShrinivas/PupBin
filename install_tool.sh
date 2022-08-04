echo "Installing pupbin tool..."
echo "Checking if rust toolchains are present..."
cargo > /dev/null
if [ $? -ne "0" ];then
    echo "Rust toolchains not found, installing rust"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
else
    echo "Rust toolchains found!"
fi
if [ $? -ne "0" ];then
    echo 'build failed'
    exit
fi
echo "building tool..."
cd Frontend
cargo build --release
if [ $? -eq "0" ];then
    sudo cp ./target/release/pupbin /bin
fi

