use std::io::{self, Read};
use std::{env, fs};
use std::{thread, time};

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let out_file = env::var("ENSHROUDED_PLAYER_COUNT_FILE")
        .unwrap_or_else(|_e| "./enshrouded-player-count.txt".to_string());

    let mut player_count = 0u8;
    let mut buffer = String::new();

    while let Ok(n_bytes) = stdin.read_to_string(&mut buffer) {
        if n_bytes == 0 {
            thread::sleep(time::Duration::from_millis(500));
            break
        }

        for line in buffer.lines() {
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
        }

        buffer.clear();
    }
}
