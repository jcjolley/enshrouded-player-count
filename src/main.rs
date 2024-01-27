use std::io::{BufReader, BufRead};
use std::{env, fs};
use std::process::{ Command, Stdio };

fn main() {
    let mut cmd= Command::new("journalctl")
        .arg("-b")
        .arg("-u enshrouded.service")
        .arg("-f")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let out_file = env::var("ENSHROUDED_PLAYER_COUNT_FILE")
        .unwrap_or_else(|_e| "./enshrouded-player-count.txt".to_string());

    let mut player_count = 0u8;

    let stdout = cmd.stdout.as_mut().unwrap();
    let stdout_reader = BufReader::new(stdout);
    let stdout_lines = stdout_reader.lines();


    for result_line in stdout_lines {
        match result_line {
            Ok(line) => {
                if line.contains("Remote player added") {
                    player_count = player_count + 1;
                    println!("Player added!  There are {} active players", player_count);
                    fs::write(&out_file, player_count.to_string()).expect("Unable to write to file")
                }

                if line.contains("Player removed") {
                    player_count = player_count - 1;
                    println!("Player removed!  There are {} active players", player_count);
                    fs::write(&out_file, player_count.to_string()).expect("Unable to write to file")
                }
            },
            Err(_) => { /*do nothing*/ }
        }
    }
}
