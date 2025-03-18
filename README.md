# Sisyphus32
Welcome to the GitHub page for Sisyphus32, a chess engine!

# Features
In progress... ✍

# [Lichess Bot](https://lichess.org/@/Sisyphus32)
![](https://lichess-shield.vercel.app/api?username=sisyphus32&format=rapid)
![](https://lichess-shield.vercel.app/api?username=sisyphus32&format=blitz)
![](https://lichess-shield.vercel.app/api?username=sisyphus32&format=bullet)

# UCI Commands ([documentation](https://official-stockfish.github.io/docs/stockfish-wiki/UCI-&-Commands.html))
Sisyphus32 is UCI-compliant, implementing the following UCI commands:
- `uci`
- `ucinewgame`
- `isready`
- `position (fen <fenstring> | startpos) [moves <move1> ... <movei>]`
- `go [perft <plies> | [depth <plies>] [wtime <ms>] [btime <ms>] [winc <ms>] [binc <ms>] [movetime <ms>]]`
- `stop | s`
- `quit | q`
- `exit | e`
- `eval`
- `display | d`
- `bench | benchmedium`
- `benchlong`
- `benchshort`
- `setoption name Clear Hash`
- `setoption name Threads value <n>`
