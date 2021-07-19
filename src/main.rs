use interfaces::{Interface, Kind};
use procfs::process;
use seahorse::{App, Command};
use std::env;
use std::str;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn print_process_list() -> Result<()> {
    println!("{:<10}{:<10}{:<10}\t{}", "OWNER", "PID", "PPID", "CMDLINE");
    for proc in process::all_processes()? {
        let stat = &proc.stat;
        let cmdline = proc.cmdline()?;
        if cmdline.len() == 0 {
            continue;
        }
        println!(
            "{:<10}{:<10}{:<10}\t{}",
            &proc.owner,
            &stat.pid,
            &stat.ppid,
            &proc.cmdline()?.get(0).unwrap_or(&"".into())
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

fn print_port_scan(ip: &str, start_port: usize, end_port: usize) {
    println!("--------------------\n{}:\n--------------------", ip);
    for port in port_scan(ip, start_port, end_port).unwrap() {
	println!("{:<15} Open", port);
    }
}
fn port_scan(_ip: &str, start_port: usize, end_port: usize) -> Result<Vec<usize>> {

    Ok([start_port, end_port].into())
}

fn download_file(url: &str, dst: &str) {
    println!("{} -> {}", url, dst);
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage(format!("{} [Commands]", args[0]))
        .command(
            Command::new("ip")
                .description("Display network information about the interfaces")
                .action(|_| {
                    print_network_interfaces().unwrap();
                }),
        )
        .command(
            Command::new("ps")
                .description("Display process information")
                .action(|_| {
                    print_process_list().unwrap();
                }),
        )
	.command(
	    Command::new("scan")
		.description("Scan target IP")
		.usage("[IP] [StartPort] [EndPort]")
		.action(|ctx| {
		    let mut arg_iter = ctx.args.iter().take(3);
		    let ip = arg_iter.next().unwrap();
		    let start_port: usize = arg_iter.next().unwrap().parse().unwrap();
		    let end_port: usize = arg_iter.next().unwrap().parse().unwrap();
		    print_port_scan(ip, start_port, end_port);
		})
	)
	.command(
	    Command::new("wget")
		.description("Download file")
		.usage("[URL] [Destination File]")
		.action(|ctx| {
		    let mut arg_iter = ctx.args.iter().take(2);
		    let url = arg_iter.next().unwrap();
		    let dst = arg_iter.next().unwrap();
		    download_file(url, dst);
		})
	);
    app.run(args);

    Ok(())
}
