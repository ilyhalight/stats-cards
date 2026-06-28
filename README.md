# stats-cards

🦀 Blazing fast stats cards for everything

## Available cards

Current list of available cards:

- Wakatime languages (compact with progress)

  Endpoint: `/v1/top-langs/wakatime?username=Toil`

   <img src="https://stats-cards.nirel.cc/v1/top-langs/wakatime?username=Toil" alt="demo" />

- Github languages (compact with progress)

  Endpoint: `/v1/top-langs/github?username=ilyhalight`

   <img src="https://stats-cards.nirel.cc/v1/top-langs/github?username=ilyhalight" alt="demo" />

  \* Required `GITHUB_TOKEN` in `.env`

- Github Activity Graph

  Endpoint: `/v1/activity/github?username=ilyhalight&period=3_months&with_title=true`

   <img src="https://stats-cards.nirel.cc/v1/activity/github?username=ilyhalight&period=3_months&with_title=true" alt="demo" />

  Support disabling title with param `with_title`:

  - `true` (default)
  - `false`

  Support select period with param `period`:

  - `3_months` (default)
  - `6_months`
  - `year`

  \* Required `GITHUB_TOKEN` in `.env`

- Huggingface Pin Repository

  Endpoint: `/v1/pin/huggingface?username=openai&repo=whisper-large-v3-turbo&show_owner=true&type=model`

   <img src="https://stats-cards.nirel.cc/v1/pin/huggingface?username=openai&repo=whisper-large-v3-turbo&show_owner=true&type=model" alt="demo with show_owner" />

  Support select repo type with param `type`:

  - `model`
  - `dataset`
  - `space`

  Support show owner name with param `show_owner`:

  - `true`
  - `false` (default)

  \* Required `HUGGINGFACE_TOKEN` in `.env`

- Github Extra Pin Repository

  Endpoint: `/v1/pin/github?username=ilyhalight&repo=stats-cards&show_owner=true`

   <img src="https://stats-cards.nirel.cc/v1/pin/github?username=ilyhalight&repo=stats-cards&show_owner=true" alt="demo with show_owner" />

  Support show owner name with param `show_owner`:

  - `true`
  - `false` (default)

  **Cyrillic supported!**

- Github Extra Pin Gist

  Endpoint: `/v1/pin/gist?id=6f47e86d2510bce28f8e7f42ae84c716&show_owner=true`

   <img src="https://stats-cards.nirel.cc/v1/pin/gist?id=6f47e86d2510bce28f8e7f42ae84c716&show_owner=true" alt="demo with show_owner" />

  Support show owner name with param `show_owner`:

  - `true`
  - `false` (default)

  **Cyrillic supported!**

List of planned cards:

- [WIP] [Github Streak](https://github.com/DenverCoder1/github-readme-streak-stats)
- [WIP] [Wakatime Streak](https://github.com/DenverCoder1/github-readme-streak-stats)
- [WIP] [Github stats](https://github.com/anuraghazra/github-readme-stats)

\* links contain design examples

## Themes

You can select theme for your card with param `theme=THEME_NAME`. Default theme is `catppuccin-macchiato`.

Let's check other available themes [here](THEMES.md)

## How to run

To run your own instance:

### With Docker

1. Install [Docker](https://www.docker.com/)
2. Build the image

```bash
docker build -t "stats-cards" .
```

3. Run container

```bash
docker compose up -d
```

### Manually

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
