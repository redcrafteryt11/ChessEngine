# ChessEngine

A UCI-compatible chess engine written in Rust from scratch.

## Features

- Bitboard-based board representation (12× u64)
- Complete FEN parsing
- Full legal move generation
  - Pawns (single push, double push, captures, en passant, promotion)
  - Knights, bishops, rooks, queens, king
  - Castling (kingside + queenside)
- Check detection & legal move filtering
- Hyperbola Quintessence sliding piece attacks
- Alpha-Beta search with iterative deepening
- Material + Piece-Square Table evaluation
- UCI protocol (compatible with Arena, Cute Chess, Lucas Chess, etc.)

## Verified Perft Results

| Depth | Nodes     |
|-------|-----------|
| 1     | 20        |
| 2     | 400       |
| 3     | 8,902     |
| 4     | 197,281   |
| 5     | 4,865,609 |

## Build

Requires Rust 1.70+

```bash
cargo build --release
```

Binary: `target/release/ChessEngine.exe`

## Usage

Load `target/release/ChessEngine.exe` as a UCI engine in any chess GUI.

Tested with [Arena Chess](http://www.playwitharena.de/).

### Manual UCI
