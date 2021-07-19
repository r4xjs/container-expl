use procfs::process;
use interfaces::{Interface, Kind};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn print_process_list() -> Result<()> {
    println!("{:<10}{:<10}{:<10}\t{}", "OWNER", "PID", "PPID", "CMDLINE");
    for proc in process::all_processes()? {
	let stat = &proc.stat;
	let cmdline = proc.cmdline()?;
	if cmdline.len() == 0 {
	    continue;
	}
	println!("{:<10}{:<10}{:<10}\t{}",
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
		}, 
		_ => continue,
	    };
	}
    }
    Ok(())
}




fn main() -> Result<()> {

    print_network_interfaces()?;
    if false {
	print_process_list()?;
    }
    Ok(())
}
