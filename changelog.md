# 0.2.5

- Added new themes support:
  - `uchu` theme
  - `black-pink` theme (implemented by @maestroTW in #3)
- Micro refactor of `index.js` file

## 0.2.4

- Added support Github Extra Pin Gist (Cyrillic supported!)

## 0.2.3

- Added support Github Extra Pin Repository (Cyrillic supported!)

## 0.2.2

- Updated `html` language color
- Added `vue.js` (alias to `vue`) language color
- Bump depends

## 0.2.1

- Improved calculating Github languages stats. Now we count them based on the number of bytes in the first 100 repositories by the number of stars.
- (!) Now to use Github languages requires `GITHUB_TOKEN` in `.env` file
- Fixed Dockerfile
- Bump depends

## 0.2.0

- Added support select Theme with param `theme`:
  - `catppuccin-macchiato` (default)
  - `catppuccin-mocha` (**NEW**)
  - `catppuccin-latte` (**NEW**)
  - `catppuccin-frappe` (**NEW**)
  - `dark` (**NEW**)
  - `white` (**NEW**)
  - `onedark-pro-flat` (**NEW**)
  - `dracula` (**NEW**)
  - `kanagawa-wave` (**NEW**)
  - `ayu-mirage` (**NEW**)
  - `ayu-white` (**NEW**)
  - `monokai-classic` (**NEW**)

- Fixed `Unknown API error` if Huggingface space doesn't have `models` property in response
- (!) Fixed typo in `catppuccin-macchiato` theme (old `catpuccin-macchiato`). Check spelling in your urls

## 0.1.8

- Added extra check for Huggingface unauthorized api response
- Added support Pin dataset and space from Huggingface

## 0.1.7

- Added support Pin model from Huggingface
- Stats Cards Generator frontend now cache values only for similar cards e.g. github username uses only for `Languages (Github)` and `Activity (Github)` and etc

## 0.1.6

- Added Github "Bad Credentials" error handling
- Added Wakatime "Time range not matching user's public stats range." and hidden stats error handling
- Font `Ubuntu` replaced to `system-ui`
- Moved all hardcoded error template messages to Prepared Templates
- Removed test print

## 0.1.5

- Moved create request client to lazy_static
- Update rust edition to 2024
- Bump depends

## 0.1.4

- Move some editable global consts to config file
- Added set port (`SERVICE_PORT`) and hostname (`SERVICE_HOST`) with env
- Added Dockerfile

## 0.1.3

- Added fast action buttons (copy url as `markdown`, `plain`, `(html) code`) to index page

## 0.1.2

- Added index page (`/`) with cards generator
- Bump depends

## 0.1.1

- Added Github activity graph

## 0.1.0

- Initial release
