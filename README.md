# stats-cards

🦀 Blazing fast stats cards for everything

## Available cards

Current list of available cards:

- Wakatime languages (compact with progress)

   <img src="https://stats-cards.toil.cc/v1/top-langs/wakatime?username=Toil" alt="demo" />

- Github languages (compact with progress)

   <img src="https://stats-cards.toil.cc/v1/top-langs/github?username=ilyhalight" alt="demo" />

- Github Activity Graph

   <img src="https://stats-cards.toil.cc/v1/activity/github?username=ilyhalight&period=3_months&with_title=true" alt="demo" />

  Support disabling title with param `with_title`:

  - `true` (default)
  - `false`

  Support select period with param `period`:

  - `3_months` (default)
  - `6_months`
  - `year`

  \* Required GITHUB_TOKEN in `.env`

List of planned cards:

- [WIP] [Github Streak](https://github.com/DenverCoder1/github-readme-streak-stats)
- [WIP] [Wakatime Streak](https://github.com/DenverCoder1/github-readme-streak-stats)
- [WIP] [Github stats](https://github.com/anuraghazra/github-readme-stats)
- [WIP] [GitHub Extra Pins](https://github.com/anuraghazra/github-readme-stats)
- [WIP] [GitHub Gist Pins](https://github.com/anuraghazra/github-readme-stats)

\* links contain design examples

## Themes

Available themes:

- [Catpuccin Macchiato](https://github.com/catppuccin/catppuccin) (default)

## How to run

To run your own instance:

1. Install [Rust 1.75+](https://www.rust-lang.org/learn/get-started)

2. (Optional) Run for developing:

   2.1. Install cargo watch:

   ```bash
   cargo install cargo-watch
   ```

   2.2. Run live server:

   ```bash
   cargo watch -x run
   ```

3. Run for Production:

   3.1. Build:

   ```bash
   cargo build --release
   ```

   3.2. Run a stats-cards file:

   ```bash
   ./target/release/stats-cards
   ```

Written with ❤️ & 🦀
