# focus point

To run Focus Point game:

    cargo run --release

# to run examples:

    cargo run --example circle
    cargo run --example quad

## References

I've been collecting notes on Bevy [here](./BEVY_REFERENCES.md)

## TODO:

- display selected tile (colored border)
- function to detect puzzle completion (add original index to TileMetadata and, for all tiles, compare original_index and index are equal)
- apply puzzle logic from macroquad app
- support resize & full screen
- states (menu screen, etc) => https://bevy-cheatbook.github.io/basics/states.html
- optimize change detection => https://bevy-cheatbook.github.io/basics/change-detection.html
- high score?
