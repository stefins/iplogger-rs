# iplogger-rs

A lightweight daemon application written in Rust that monitors and logs your public IP address changes.

## Features

- 🔄 Automatically checks your public IP address every 20 minutes
- 📝 Logs IP address changes with timestamps
- 🚀 Runs as a background daemon
- 💾 Minimal resource usage
- 🔒 Only logs when your IP address actually changes

## How It Works

iplogger-rs runs as a daemon process that:
1. Checks your public IP address by querying `https://ifconfig.me/ip`
2. Compares it with the last logged IP address
3. If the IP has changed, logs the new IP with a timestamp
4. Waits 20 minutes before checking again

## Installation

### Homebrew

```bash
brew tap stefins/iplogger
brew install stefins/iplogger/iplogger-rs
```

### Build from Source

#### Prerequisites

- Rust 1.56.0 or later
- OpenSSL development headers

On Ubuntu/Debian:
```bash
sudo apt-get install libssl-dev
```

On Arch Linux:
```bash
sudo pacman -S openssl
```

On Fedora:
```bash
sudo dnf install openssl-devel
```

#### Build and Install

```bash
git clone https://github.com/stefins/iplogger-rs.git
cd iplogger-rs
cargo build --release
sudo cp target/release/iplogger /usr/local/bin/
```

## Usage

### Starting the Logger

Simply run the binary:
```bash
iplogger
```

The daemon will:
- Print "Starting Iplogger" to the console
- Run in the background
- Create necessary directories and files automatically

### Log File Location

IP address changes are logged to:
```
~/.iplogger/log.txt
```

Each entry includes the IP address and timestamp:
```
203.0.113.42 - 2021-12-25 14:30:45.123456 +00:00
198.51.100.23 - 2021-12-26 09:15:22.987654 +00:00
```

### Managing the Daemon

#### Check if Running

```bash
ps aux | grep iplogger
# or
cat /tmp/iplogger.pid
```

#### Stop the Daemon

```bash
kill $(cat /tmp/iplogger.pid)
```

#### View Logs

```bash
cat ~/.iplogger/log.txt
# or for continuous monitoring
tail -f ~/.iplogger/log.txt
```

#### Error Logs

Error logs are written to:
```
/tmp/iplogger.err
```

## Configuration

The daemon uses the following locations:
- **PID file**: `/tmp/iplogger.pid`
- **Log file**: `~/.iplogger/log.txt`
- **Error log**: `/tmp/iplogger.err`
- **Check interval**: 20 minutes (hardcoded)

## Dependencies

- [daemonize](https://crates.io/crates/daemonize) - Daemon functionality
- [reqwest](https://crates.io/crates/reqwest) - HTTP client
- [chrono](https://crates.io/crates/chrono) - Date and time handling
- [dirs](https://crates.io/crates/dirs) - Standard directory paths

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Feel free to submit issues or pull requests.

## Author

Stef (sstefin@bk.ru)