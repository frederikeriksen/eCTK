# Exploratory Container ToolKit (For Teaching)
## Setup Guide
OPS: You must have vagrant and virtualbox installed!
1. Clone the repository
2. Run vagrant up
3. Once provisioned, run vagrant ssh
4. Inside the container, become root (sudo -i)
5. Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
6. Run: source $HOME/.cargo/env
7. Run: cd /container/ectk && cargo build --release
8. Run: cp ./target/release/ectk /usr/bin
9. Run: cp ./target/release/ectk /container/usr/bin && cd /

## Commands