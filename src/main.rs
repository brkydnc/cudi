mod attribute;

use attribute::ATTRIBUTE_LIST;
use clap::Parser;
use colored::Colorize;
use cust::{device::*, prelude::*};

/// Print CUDA device properties
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Do not print the descriptions of attributes.
    #[arg(short, long)]
    no_descriptions: bool,
}

fn main() {
    let args = Args::parse();

    let Ok(_) = cust::init(CudaFlags::empty()) else {
        eprintln!("Couldn't initialize CUDA Driver API.");
        return;
    };

    let Ok(devices) = Device::devices() else {
        eprintln!("No CUDA devices found.");
        return;
    };

    for (idx, dev) in devices.enumerate() {
        if let Ok(device) = dev {
            match device.name() {
                Ok(name) => println!("Device #{}: {}", idx, name.green()),
                _ => println!("Device #{}:", idx),
            }

            if let Ok(memory) = device.total_memory() {
                println!(
                    "{:indent$}{}: {}",
                    "",
                    "TotalMemory".yellow(),
                    memory.to_string().bold(),
                    indent = 4
                );

                if !args.no_descriptions {
                    println!(
                        "{:indent$}{}",
                        "",
                        "The total amount of memory available on the device in bytes"
                            .bright_black(),
                        indent = 8
                    );
                }
            }

            for (attribute, description) in ATTRIBUTE_LIST {
                if let Ok(value) = device.get_attribute(attribute) {
                    let name = format!("{:?}", attribute);
                    println!("{:indent$}{}: {}", "", name.yellow(), value, indent = 4);

                    if !args.no_descriptions {
                        println!("{:indent$}{}", "", description.bright_black(), indent = 8);
                    }
                }
            }
        } else {
            println!(
                "Device #{}: {}",
                idx,
                "couldn't access device information.".red()
            )
        }
    }
}
