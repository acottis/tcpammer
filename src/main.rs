use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::thread;

const HACKERS: usize = 6;

fn main() {
    // Parse command line or default 127.0.0.1:3000
    let target = pick_target();

    // My lambda that both use
    let schedule = |id: usize| -> thread::JoinHandle<()> {
        println!("Hacker {id} ONLINE, target -> {target}");
        thread::spawn(move || hack(id, target))
    };

    (0..HACKERS)
        .map(schedule)
        .collect::<Vec<_>>()
        .into_iter()
        .try_for_each(|job| job.join())
        .unwrap();
}

fn pick_target() -> SocketAddr {
    let mut args = std::env::args();

    let mut target: (Option<IpAddr>, Option<u16>) = (None, None);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--host" => target.0 = args.next().and_then(|host| IpAddr::from_str(&host).ok()),
            "-p" | "--port" => target.1 = args.next().and_then(|port| u16::from_str(&port).ok()),
            _ => {}
        };
    }
    SocketAddr::new(
        target.0.unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
        target.1.unwrap_or(3000),
    )
}

fn hack(id: usize, target: SocketAddr) {
    const PAYLOAD: &'static str = "1337 h4ck0rz\r\n";

    let mut count = 0;
    let mut now = std::time::SystemTime::now();
    loop {
        match TcpStream::connect_timeout(&target, std::time::Duration::from_secs(1)) {
            Ok(mut conn) => {
                conn.write(PAYLOAD.as_bytes()).unwrap();
                conn.read(&mut [0u8]).unwrap();
                count += 1;
            }
            Err(e) => {
                println!("{:?}", e);
            }
        };

        if count % 5000 == 0 {
            println!(
                "Job {id}: Sent {} packets in {} milliseconds",
                count,
                now.elapsed().unwrap().as_millis(),
            );
            now = std::time::SystemTime::now();
            count = 0;
        }
    }
}
