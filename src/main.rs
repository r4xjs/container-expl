use interfaces::{Interface, Kind};
use procfs::process;
use seahorse::{App, Command};
use std::env;

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
        println!("{}", &iface.name);
        for addr in &iface.addresses {
            match addr.kind {
                Kind::Ipv4 | Kind::Ipv6 => {
                    println!("{:?}", addr)
                }
                _ => continue,
            };
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage(format!("{} [args]", args[0]))
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
        );
    app.run(args);

    Ok(())
}
