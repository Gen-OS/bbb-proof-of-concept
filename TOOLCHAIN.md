# How to get a working(???) rust tool chain for `thumbv7a-none-eabi`

[Instruction for a Compiler](https://www.somethingawful.com/news/instruction-fruit-below/1/)

## Prerequisites
I only have easy access to an OS X El Capitan system to test this on at the moment, although the only stuff that should be platform-specific doesn't need much testing.

I'll assume that you have Xcode Dev Tools and Homebrew already installed.

## Step 1: Get `arm-none-eabi-gcc`

### OS X
```
$ brew cask install gcc-arm-embedded
```

### Ubuntu
```
$ sudo apt-get install gcc-arm-none-eabi
```

## Step 2: Get `rustup` and a native nightly `rustc`
```
$ curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly
```

Also this randomly breaks sometimes, because of course it does. Blame CloudFlare for taking forever to actually listen to invalidation requests because reasons.

Ensure that `~/.cargo/bin` is on your PATH before continuing.

## Step 3: Compile libcore for `thumbv7a-none-eabi`
Download install-libcore.sh and execute it with the proper options, insuring that you are in the same folder as `thumbv7a-none-eabi.json`.

```
$ curl https://raw.githubusercontent.com/phil-opp/nightly-libcore/master/install-libcore.sh | sed -e 's/multirust/rustup/' -e 's/rm "\$0"//' | bash -s - thumbv7a-none-eabi
```

## Bonus: How to actually build it (for the BBB, at least)
```
$ cargo build --target thumbv7a-none-eabi --features "board-beaglebone-black" --verbose
```
