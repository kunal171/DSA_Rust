use std::{env, io::Write, net::{IpAddr, TcpStream}};
use std::io::{self};
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::{process, thread};

const MAX: u16 = 65535; //Max port that we can sniff

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments{
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f)  {
            return Ok(Arguments {flag: String::from(""), ipaddr , threads: 4});
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!(" Usage: -j to select how many threads you want 
                \r\n -h or -help to show this help message");
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("Too many argumnets");
            } else if flag.contains("-j") { 
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IPADDR; must be IPV4 or IPV6")
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread number")
                };
                return Ok(Arguments{flag, ipaddr, threads});
            } else {
                return Err("Invalid syntax");
            }
        }
    }
}

fn scan( tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                println!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            } else {
                println!("{} problem parsing arguments: {}", program, err);
                process::exit(0);
            }
        }
    );
    let num_thread = arguments.threads;
    let _addr =arguments.ipaddr;
    let (tx, rx) = channel();

    for i in 0..num_thread {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, _addr, num_thread);
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
        println!("{} is open", v );
    }
}
