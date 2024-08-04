# hn-vizia

Experimental desktop client for Hackernews

## Current status

**Archived**: Vizia is replacing its renderer from femtovg(Rust) to skia (C++).
Recommend migration to [iced](https://github.com/iced-rs/iced) which uses skia only as a wgpu fallback.


**Behavior:** It's just doing fetch and show the top 15 items but the links are not working.

## Building locally

```sh
git clone https://github.com/ancos2505/hn-vizia

cd hn-vizia

# Dep: Task runner
cargo install just

just run

# or

cargo run
```

## Screenshot

![Screenshot](/doc/img/screenshot.png)

## TODO

- [ ] Define synchronization strategy before async implementation
