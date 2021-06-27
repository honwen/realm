use serde::{Deserialize, Serialize};
use std::str::FromStr;
use structopt::StructOpt;

const VALID_PROTOCOLS: &'static [&'static str] = &["tcp", "udp"];

#[derive(StructOpt)]
#[structopt(name = "realm", about = "A high efficiency proxy tool.")]
pub struct Cli {
    #[structopt(
        short = "L",
        long = "listen",
        help = "default: -L=:8080/127.0.0.1:1080; example: -L=:5300/9.9.9.11:9953"
    )]
    pub listen: Option<Vec<String>>,
}

pub struct RelayConfig {
    pub listening_address: String,
    pub listening_port: String,
    pub remote_address: String,
    pub remote_port: String,
    pub protocol: String,
}

impl Default for RelayConfig {
    fn default() -> RelayConfig {
        RelayConfig {
            listening_address: String::from("0.0.0.0"),
            listening_port: String::from("1080"),
            remote_address: String::from("127.0.0.1"),
            remote_port: String::from("8080"),
            protocol: String::from(""),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    pub listening_addresses: Vec<String>,
    pub listening_ports: Vec<String>,
    pub remote_addresses: Vec<String>,
    pub remote_ports: Vec<String>,
}

pub fn parse_arguments() -> Vec<RelayConfig> {
    let input = Cli::from_args();

    // parse command line arguments
    let mut configs = vec![];

    if input.listen == None {
        configs.push(Default::default());
        println!("# configs: {}", "-L=:8080/127.0.0.1:1080");
        return configs;
    }

    let listen = match input.listen {
        Some(listen) => listen,
        None => panic!("No listening socket"),
    };

    for it in listen {
        let protocol_parse: Vec<&str> = it.split("://").collect();
        let protocol = if protocol_parse.len() == 2 {
            let protocol = String::from_str(protocol_parse[0]).unwrap().to_lowercase();
            if !VALID_PROTOCOLS.contains(&protocol.as_str()) {
                panic!("protocol must be {} !", VALID_PROTOCOLS.join(" or "))
            }
            protocol
        } else {
            String::from("")
        };
        let listen_parse: Vec<&str> = if protocol_parse.len() == 2 {
            protocol_parse[1].split("/").collect()
        } else {
            protocol_parse[0].split("/").collect()
        };
        if listen_parse.len() != 2 {
            panic!("listen config is incorrect!");
        }
        let client = String::from_str(listen_parse[0]).unwrap();
        let remote = String::from_str(listen_parse[1]).unwrap();

        let client_parse: Vec<&str> = client.rsplitn(2, ":").collect::<Vec<&str>>().into_iter().rev().collect();
        if client_parse.len() != 2 {
            panic!("client address is incorrect!");
        }
        let listening_address = String::from_str(client_parse[0]).unwrap();
        let remote_parse: Vec<&str> = remote.rsplitn(2, ":").collect::<Vec<&str>>().into_iter().rev().collect();
        if remote_parse.len() != 2 {
            panic!("remote address is incorrect!");
        }

        println!("# configs: {}", it);

        configs.push(RelayConfig {
            listening_address: if listening_address == "" {
                String::from("0.0.0.0")
            } else {
                listening_address
            },
            listening_port: String::from_str(client_parse[1]).unwrap(),
            remote_address: String::from_str(remote_parse[0]).unwrap(),
            remote_port: String::from_str(remote_parse[1]).unwrap(),
            protocol: protocol,
        })
    }

    configs
}
