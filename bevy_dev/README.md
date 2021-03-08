# to run:

    cargo run --release --bin quad_uvs

## Focus Point TODO

- elect image randomly in image_metadatas, fill array with all of them too
- fix bug sticky selection (will probabily require assigning systems to stages => https://bevy-cheatbook.github.io/basics/stages.html)
- display selected tile (colored border)
- function to detect puzzle completion (add original index to TileMetadata and, for all tiles, compare original_index and index are equal)
- apply puzzle logic from macroquad app
- states (menu screen, etc) => https://bevy-cheatbook.github.io/basics/states.html
- optimize change detection => https://bevy-cheatbook.github.io/basics/change-detection.html
- show credits
- high score
