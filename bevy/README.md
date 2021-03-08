# bevy experiments

To run the WIP Focus Point game:

```sh
    cargo run --release --bin focus_point
```

# to run examples (NOT WORKING?):

compiles the tests:

```sh
    cargo test
```

runs examples:

```sh
    cargo run --example circle
    cargo run --example quad
```

this example is from bevy repo. use use it sometimes as reference

```sh
    cargo run --example breakout
```

this example is just a mixture of samples, not really relevant:

```sh
    cargo run --example one
```

## References

I've been collecting notes on Bevy [here](./BEVY_REFERENCES.md)
    

## TODO:

- elect image randomly in image_metadatas, fill array with all of them too
- fix bug sticky selection (will probabily require assigning systems to stages => https://bevy-cheatbook.github.io/basics/stages.html)
- display selected tile (colored border)
- function to detect puzzle completion (add original index to TileMetadata and, for all tiles, compare original_index and index are equal)
- apply puzzle logic from macroquad app
- states (menu screen, etc) => https://bevy-cheatbook.github.io/basics/states.html
- optimize change detection => https://bevy-cheatbook.github.io/basics/change-detection.html
- show credits
- high score
