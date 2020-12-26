\*Change name to timebar

# Time Bar Cli

Time progress bars to look at while you're stuck on your terminal, to remind you time is passing by.

## Usage

### Year progress bar

To get how your year's progress,
`timebar year`

```
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░ 97.7%
```

This is what [this twitter bot](https://twitter.com/year_progress) does, just in terminal.

### Life progress bar

Checking in how your life timeline bar is doing..
`timebar life` -> promts birthday and lifespan

```
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░ 57.7%

You want to exist on this planet for 80 years.
You have:
13 years
23718 months
12329384698 minutes
129837192849 seconds
left for your life. Have a good day!
```

(-> save to environment variable)
To set your info again,
`timebar lifespan` -> prompts birthday lifespan (takes integer) Age 72
life expectancy at birth was 71 years
`date-month-year` order.

### Get help

`./timebar -h`
