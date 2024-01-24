
> cat test_input.txt | target/release/enshrouded-player-count
```
Player added!  There are 1 active players
Player added!  There are 2 active players
Player added!  There are 3 active players
Player removed!  There are 2 active players
Player added!  There are 3 active players
Player removed!  There are 2 active players
Player removed!  There are 1 active players
Player removed!  There are 0 active players
``` 

Pipe your enshrouded output into this program, and it will update enshrouded-player-count.txt 
with the active player count.  You can change the file name with the ENSHROUDED_PLAYER_COUNT_FILE env var 