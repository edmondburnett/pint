# Pint 🥤

WIP. Track and record personal daily water intake.

The existing landscape of hydration trackers I've tried are not to my taste, thus this little project.

## Todo

- Customizable units (metric + imperial/US)
- Customizable cup sizes
- Graph/viz of some kind to show daily hydration history
- Support for coffee w/adjusted estimated absorption rate
- config file under `~/.config/pint`
- Store/load updates in sqlite under `~/.local/share/pint/`
  - tz-aware timestamp
  - amount
  - unit
  - beverage type?
- csv data export from sqlite
- Perhaps a prometheus exporter or something for Grafana homelab nerds
- CLI args: updates from command line, exports etc
- help/usage menu?
- Rename project to something available on crates.io
- On Halloween, water turns into blood (obviously)
