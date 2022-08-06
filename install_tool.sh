echo "Which distro are u running me on?"
echo "1.Debian base distros [Ubunut, pop, ...]"
echo "2.Arch base distro [EOS, Manjaro, ..]"
echo "3.Other (Make sure you have openssl and libssl dev installed, this options skips process)"
echo -n "Enter option :"
read option

if [ $option -eq "1" ];then
    sudo apt update
    sudo apt install -y openssl libssl-dev pkg-config curl wget
elif [ $option -eq "2" ]; then
    sudo pacman -Syy openssl  curl wget
fi

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

