# Description

Just a project to learn some rust.

This tool can be used to explore a stripped down container images where you don't have tools like `ps` or `ip`. 
Further, the tool provides some network related commands, see the help output below for a list of supported commands.

# Usage

```
container-expl

USAGE:

  container-expl <CMD> [OPTIONS]

  CMD:
    ip          Display network interface infos
    ps          Display process infos
    wget        Download file from --url <URL> and
                save it to --out <FILE>
    scan        Port scan --ip <IP>. Start at port
                --start <PORT> and end at port --end <PORT>
    bind        Bind --port <PORT> and start listening.
                Works similar as netcat
    connect     Connect to --ip <IP> and --port <PORT>
                Works similar as netcat

  EXAMPLE:
    container-expl ip
    container-expl ps
    container-expl scan --ip 1.1.1.1 --start 80 --end 1000
    container-expl bind --port 1234
    container-expl connect --ip 1.1.1.1 --port 1234
    container-expl wget --url http://localhost:1234/xyz.txt --out f00.txt
```

# Build & Run

```
rustup target add x86_64-unknown-linux-musl
make
target/x86_64-unknown-linux-musl/release/container-expl
```

# Current Build

https://www.lowsec.net/release/container-expl

