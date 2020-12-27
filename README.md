# Time Bar Cli

Some time progress bars to look at on your terminal. Pretty much doing what [this twitter bot](https://twitter.com/year_progress) does, just in terminal.

## Installation

### From [crates.io](https://crates.io/crates/timebar)

```
cargo install timebar
```

### From source

You need cargo to compile from source, which is shipped with Rust.

```
git clone https://github.com/sungyeonkwon/timebar
cargo install --path ./timebar # Compile and install
```

## Usage

### Year progress

To get this year's time bar, run `timebar year`.

```
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░ 97.7%

This year's almost over!
```

### Life progress

To get your life time bar, run `timebar life`. This prompts to enter your birthday and your expected lifespan.

```
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░ 51.1%

You expect to exist on this planet for 70 years.

You have:

34 in years
411 in months
1786 in weeks
12502 in days
300057 in hours
18003447 in minutes
1080206822 in seconds.

Have a good day!
```

### Timer progress

To get a timer bar, run `timebar timer`. This prompts to enter duration.

```
▓░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 0.6%

Time is ticking... You have:

0 in hours
10 in minutes
616 in seconds.
```

## TODO

- [ ] Add a d-day feature
- [ ] Cache life and dday with environment variable
