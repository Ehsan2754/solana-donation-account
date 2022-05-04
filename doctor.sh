if ! command -v rustup &> /dev/null
then
    echo "Installing Rust"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
    rustup component add rustfmt
else
    echo "Rust is installed!"
fi

if ! command -v solana &> /dev/null
then
    echo "Installing solana"
    sh -c "$(curl -sSfL https://release.solana.com/v1.9.1/install)"
else
    echo "Solana is installed!"
fi

if ! command -v node &> /dev/null
then
    echo "Installing nodejs"
    sudo apt -y install curl dirmngr apt-transport-https lsb-release ca-certificates
    curl -sL https://deb.nodesource.com/setup_12.x | sudo -E bash -
    sudo apt -y install nodejs
else
    echo "Nodejs is installed!"
fi

if ! command -v yarn &> /dev/null
then
    echo "Installing yarn"
    sh -c "npm install -g yarn"
else
    echo "Yarn is installed!"

fi

if ! command -v anchor &> /dev/null
then
    echo "Installing anchor\n"
    sudo apt-get update && sudo apt-get upgrade && sudo apt-get install -y pkg-config build-essential libudev-dev
    cargo install --git https://github.com/project-serum/anchor --tag v0.24.2 anchor-cli --locked
else
    echo "Anchor is installed!"
    exit  
fi

