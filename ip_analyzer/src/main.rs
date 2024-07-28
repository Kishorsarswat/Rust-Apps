use std::io::Write;
use std::net::TcpStream;
use std::{io, process, thread};
use std::sync::mpsc::{channel, Sender};
use std::{env, net::IpAddr};
use std::str::FromStr;

struct Arguments {
    ipaddr: IpAddr,
    threads: u16,
}

const MAX: u16 = 65535;

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("too less arguments!");
        }else if args.len() > 4{
            return Err("too many arguments!");
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f){
            return Ok(Arguments{
                ipaddr,
                threads: 4,
            });
        }else {
            let flag = args[1].clone();
            if (flag.contains("-h") || flag.contains("-help")) && args.len() > 2 {
                return Err("too many arguments!");
            }else if flag.contains("-h") || flag.contains("-help") {
                println!(
                    "Usage: \n flags: \n  1. '-j': to specify the number of threads.\n  2. '-h': to see this message and get help\n\n ipaddress: ip to sniff\n\n");
                return Err("help");
            }else if flag.contains("-j"){
                let ipaddr = match IpAddr::from_str(&args[3]){
                    Ok(s) => s,
                    Err(_) => return Err("Invalid Ip address")
                };
                let threads = match args[2].parse::<u16>(){
                    Ok(s) => s,
                    Err(_) => return Err("Error to parse the threads")
                };
                return Ok(Arguments{
                    ipaddr,
                    threads,
                });
            }else{
                return Err("Invalid syntax!");
            }
        }
    }
}

fn scan (tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            },
            Err(_) => {}
        }
        if MAX - port <= num_threads {
            break;
        }
        port+=num_threads;
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            }else{
                println!("{} error parsing argumnets: {}",program, err);
                process::exit(0);
            }
        }
    );

    let num_threads = arguments.threads;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, arguments.ipaddr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }
    println!("");
    out.sort();
    for v in out {
        println!("{v} is open");
    }
}
