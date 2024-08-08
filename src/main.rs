use std::io::{stdout, IsTerminal};
use std::process::exit;
use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use uuid::Uuid;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command
}

#[derive(Subcommand)]
enum Command {
    ExtractTimestamp {
        uuid: String
    },
    ExtractVersion {
        uuid: String
    },
    ExtractNode {
        uuid: String
    },
    V4,
    V7,
}

fn parse_uuid(uuid: &str) -> Uuid {
    match Uuid::parse_str(uuid) {
        Ok(uuid) => return uuid,
        Err(_) => {
            println!("Invalid UUID");
            exit(1)
        },
    };
}

fn extract_timestamp(uuid_str: &str) -> DateTime<Utc> {
    let uuid = parse_uuid(uuid_str);

    let ts = match uuid.get_timestamp() {
        Some(ts) => ts.to_unix(),
        None => {
            println!("No timestamp found");
            exit(2)
        },
    };

    match DateTime::from_timestamp(
        ts.0 as i64,
        ts.1,
    ) {
        Some(date_time) => return date_time,
        None => {
            println!("Invalid timestamp");
            exit(3)
        },
    }
}

fn extract_version(uuid_str: &str) -> usize {
    let uuid = parse_uuid(uuid_str);
    uuid.get_version_num()
}

fn extract_node(uuid_str: &str) -> [u8; 6] {
    let uuid = parse_uuid(uuid_str);
    match uuid.get_node_id() {
        Some(node) => return node,
        None => {
            println!("No node found");
            exit(5)
        }
    }
}

// Add a newline only if output isn't piped
macro_rules! print_result {
    ($($x:tt)*) => {{
        if (stdout().is_terminal()) {
            println!($($x)*)
        } else {
            print!($($x)*)
        }
    }}
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::ExtractTimestamp{ uuid } => {
            let date_time = extract_timestamp(&uuid);
            print_result!("{}", date_time)
        },
        Command::ExtractVersion { uuid } => {
            let version = extract_version(&uuid);
            print_result!("{}", version)
        },
        Command::ExtractNode { uuid} => {
            let node_bytes = extract_node(&uuid);
            // probably a smarter way to do this
            print_result!("{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                     node_bytes[0], node_bytes[1], node_bytes[2],
                     node_bytes[3], node_bytes[4], node_bytes[5])
        }
        Command::V4 => {
            print_result!("{}", Uuid::new_v4())
        },
        Command::V7 => {
            print_result!("{}", Uuid::now_v7())
        }
    };
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_extract_version_v4() {
        let v4_uuid = "09b2c736-21fa-491a-84b0-56f921c6a8dc";
        let version = extract_version(v4_uuid);
        assert_eq!(4, version);
    }

    #[test]
    fn test_extract_version_v7() {
        let v7_uuid = "01912d1f-8a87-7961-84fc-fd323aaa1260";
        let version = extract_version(v7_uuid);
        assert_eq!(7, version);
    }

    #[test]
    fn test_extract_timestamp() {
        let uuid = "01912d79-dfb5-7602-89b5-7a8b05bb221c";
        let expected: DateTime<Utc> = DateTime::from_str("2024-08-07 15:34:07.541 UTC")
            .unwrap();
        let actual = extract_timestamp(uuid);
        assert_eq!(actual, expected);
    }
}