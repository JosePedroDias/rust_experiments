# focus point

To run Focus Point game:

    cargo run --release

# to run examples:

    cargo run --example circle
    cargo run --example rect
    cargo run --example rounded_rect
    cargo run --example stroked_rect

## References

I've been collecting notes on Bevy [here](./BEVY_REFERENCES.md)

## TODO:

- function to detect puzzle completion (add original index to TileMetadata and, for all tiles, compare original_index and index are equal)
- apply puzzle logic from macroquad app
- support resize & full screen => https://github.com/bevyengine/bevy/blob/main/examples/window/scale_factor_override.rs
- refactor large systems into several systems with events between them (ex: hovered tile changed event)
- check if we need to free tile's meshes when we create other ones
- states (menu screen, etc) => https://bevy-cheatbook.github.io/basics/states.html
- optimize change detection => https://bevy-cheatbook.github.io/basics/change-detection.html
- high score?
