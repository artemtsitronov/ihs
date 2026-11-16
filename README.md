# ihs - Images Hide Secrets

A micro CLI that hides text inside PNG images
Pronounced "ikhs" (rough h, like the "ch" in Bach)

The image looks exactly the same to the eye, yet the text is stored in the pixels themselves.

Built in Rust by, you guessed it, *Artem Tsitronov*.

## Install
```bash
cargo install --git https://github.com/artemtsitronov/ihs.git
```

## Build it yourself
```bash
git clone https://github.com/artemtsitronov/ihs.git
cd ihs
cargo build --release
```

## Usaje (a typo isn't a mistake)

`ihs` is like hide and seek

```bash
ihs hide Cat.png "meet me at midnight" -o secret.png
ihs reveal secret.png
```

## Why. Did. I. Do. It.

For [Hack Club.](https://hackclub.com)
