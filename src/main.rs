use interfaces::{Interface, Kind};
use procfs::process;
use anyhow::{Result, Context};

use std::fs::OpenOptions;
use std::net::{TcpStream, TcpListener};
use std::io::{Write, Read};
use std::str;
use std::thread;

fn print_process_list() -> Result<()> {
    println!("{:<10}{:<10}{:<10}\tCMDLINE", "OWNER", "PID", "PPID");
    for proc in process::all_processes()? {
        let stat = &proc.stat;
        let cmdline = proc.cmdline()?;
        if cmdline.is_empty() {
            continue;
        }
        println!(
            "{:<10}{:<10}{:<10}\t{}",
            &proc.owner,
            &stat.pid,
            &stat.ppid,
            &proc.cmdline()?.join(" ")
        );
    }

    Ok(())
}

fn print_network_interfaces() -> Result<()> {
    for iface in Interface::get_all()? {
        println!("{}:", &iface.name);
        for addr in &iface.addresses {
            match addr.kind {
                Kind::Ipv4 | Kind::Ipv6 => {
                    if let (Some(ip_addr), Some(mask)) = (addr.addr, addr.mask) {
                        println!("\tIP: {:<40} Mask: {:<40}", ip_addr, mask)
                    }
                }
                _ => continue,
            };
        }
    }
    Ok(())
}

fn print_port_scan(ip: &str, start_port: usize, end_port: usize) -> Result<()> {
    println!("--------------------\n{}:\n--------------------", ip);
    for port in port_scan(ip, start_port, end_port)? {
        println!("{:<15} Open", port);
    }
    Ok(())
}
fn port_scan(ip: &str, start_port: usize, end_port: usize) -> Result<Vec<usize>> {
    let mut res = vec![];
    for port in start_port..(end_port + 1) {
        match TcpStream::connect(format!("{}:{}", ip, port)) {
            Ok(_) => res.push(port),
            Err(_) => continue,
        };
    }
    Ok(res)
}

fn download_file(url: &str, dst: &str) -> Result<()> {
    let resp = ureq::get(url).call()
	.with_context(|| format!("can't open = {}", &url))?;
    let mut reader = resp.into_reader();
    let mut writer = OpenOptions::new()
        .write(true)
        .create(true)
        .open(dst)
	.with_context(|| format!("Can't open file {}", &dst))?;
    std::io::copy(&mut reader, &mut writer).
	context("download_file() io error")?;
    Ok(())
}

fn handle_socket(mut socket: TcpStream) -> Result<()> {
    // start thread to read stdin
    let mut socket2 = socket.try_clone()?;
    thread::spawn(move || {
	loop {
	    let mut buf = [0;1024];
	    match std::io::stdin().read(&mut buf) {
		Ok(0) | Err(_) => break,
		Ok(num) => socket2.write(&buf[..num]).unwrap(),
	    };
	}
    });
    // read socket and print to stdout
    loop {
	let mut buf = [0;1024];
	match socket.read(&mut buf) {
	    Ok(0) | Err(_) => break,
	    Ok(num) => std::io::stdout().write(&buf[..num]).unwrap(),
	};
    }
    Ok(())
}

fn tcp_listener(port: usize) -> Result<()> {
    let socket = TcpListener::bind(format!("0.0.0.0:{}", &port))
	.with_context(||format!("can't bind port {}", &port))?;
    println!("Listen on 0.0.0.0:{}", &port);
    let (stream, addr) = socket.accept()?;
    println!("Connection from {}", &addr);
    handle_socket(stream)?;
    Ok(())
}

fn tcp_connect(ip: &str, port: usize) -> Result<()> {
    let socket = TcpStream::connect(format!("{}:{}", ip, port))
	.with_context(|| format!("can't connect to {}:{}", &ip, &port))?;
    handle_socket(socket)?;
    Ok(())
}


const HELP: &str = "\
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

";


fn main() -> Result<()> {
    let mut args = pico_args::Arguments::from_env();
    let cmd = args.subcommand()?;

    if args.contains(["-h", "--help"]) || cmd.is_none() {
	print!("{}", HELP);
	return Ok(());
    }
    let cmd = cmd.unwrap();

    if cmd == "ip" {
	print_network_interfaces()?;
    } else if cmd == "ps" {
	print_process_list()?;
    } else if cmd == "wget" {
	let url: String = args.value_from_str("--url")?;
	let dst: String  = args.value_from_str("--out")?;
	download_file(&url, &dst)?;
    } else if cmd == "scan" {
	let ip: String = args.value_from_str("--ip")?;
	let start: usize = args.value_from_str("--start")?;
	let end: usize = args.value_from_str("--end")?;
	print_port_scan(&ip, start, end)?;
    } else if cmd == "bind" {
	let port: usize = args.value_from_str("--port")?;
	tcp_listener(port)?;
    } else if cmd == "connect" {
	let ip: String = args.value_from_str("--ip")?;
	let port: usize = args.value_from_str("--port")?;
	tcp_connect(&ip, port)?;
    }

    Ok(())
}
