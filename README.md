## Sysmon1: A simple system monitor
Uses [Stopwatch example](https://github.com/hecrj/iced/tree/master/examples/stopwatch) as basis to develop simple cpu usage monitor.

Made simply to practice and explore using the [iced](https://crates.io/crates/iced) crate.

The stop / start button is not really useful at all, but is included just to include some UI interaction.

Thanks to Héctor Ramón Jiménez (owner of [Iced repo.](https://github.com/jinjagit/iced) on GitHub) for his help with a couple of issues I had while writing this example app.

### Next steps:
1. Try creating an indicator bar ( arectangle which changes size according to the % usage var. value)
2. Use columns, and include more data (each virtual core, temps, RAM usage, etc.)

## Screen-capture:
![iced_gui.gif](https://github.com/jinjagit/sysmon1/blob/main/gif/iced_gui.gif)
