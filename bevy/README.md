# focus point

To run Focus Point game:

    cargo run --release

or (untested):

    cargo build --release
    cp target/release/* .
    focus_point

you can override the defaults. check them with `--help`. ex:

    cargo run --release -- --fs --pieces=60


to run examples (custom shapes):

    cargo run --example circle
    cargo run --example rect
    cargo run --example rounded_rect
    cargo run --example stroked_rect

## demo playing session

[<img src="https://i.vimeocdn.com/video/1083654572.jpg?mw=1700&mh=1101&q=70">](https://vimeo.com/523244502)

or visit [this vimeo playlist](https://vimeo.com/manage/showcases/8210937/info)


## References

I've been collecting notes on Bevy [here](./BEVY_REFERENCES.md)

## TODO:

- display elapsed time
- display game ended
- animate piece changes (alpha and scale down?) + sfx
- support resize
- refactor large systems into several systems with events between them (ex: hovered tile changed event)
- check if we need to free tile's meshes when we create other ones
- states (menu screen, etc) => https://bevy-cheatbook.github.io/basics/states.html
- optimize change detection => https://bevy-cheatbook.github.io/basics/change-detection.html
- high score?

## Auxiliary work

As part of the development of this game, and anticipating the need for other shapes, created these:
(it's unclear whether one needs to drop the meshes from the mesh resource after they're used)

### circle

<img src="aux_resources/circle.png" width="400">

- [code](src/shapes/circle.rs)
- [example usage](examples/circle.rs)

### rounded_rect

<img src="aux_resources/rounded_rect.png" width="400">

- [code](src/shapes/rounded_rect.rs)
- [example usage](examples/rounded_rect.rs)

### stroked_rect

<img src="aux_resources/stroked_rect.png" width="400">

- [code](src/shapes/stroked_rect.rs)
- [example usage](examples/stroked_rect.rs)

