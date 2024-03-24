<a name="readme-top"></a>

<div align="center">

  # mov-cli-rs
  [![Stargazers][stars-shield]][stars-url]
  [![Issues][issues-shield]][issues-url]
  [![LGPLv2.1 License][license-shield]][license-url]
  <br>
  <a href="https://github.com/bzzzthe18th/mov-cli-rs/issues">Report Bug</a>
  ·
  <a href="https://github.com/bzzzthe18th/mov-cli-rs/issues">Request Feature</a>

</div>

<br>

> [!Note]
> this is currently a very early in-progress tool that is being developed as I learn rust, issues will arise

## Installation
Installing mov-cli-rs is as easy as ensuring cargo and rust are both installed and running `cargo install mov-cli-rs` in your terminal. Before running this, ensure you have all the dependencies listed below.

### Dependencies
- **Supported OS:**
  - Linux (**not tested**)
  - Windows
- **[![rust][rust-shield]](https://www.rust-lang.org/learn/get-started)** (**required**)
- **[fzf](https://github.com/junegunn/fzf?tab=readme-ov-file#installation)** (**required**)
- **[![mpv][mpv-shield]](https://mpv.io/installation/)** (recommended - **only** extraction will work without mpv)

## Usage
The basic CLI command is just `mov-cli-rs`, the only required argument is the search term. The most basic usage of this can look like `mov-cli-rs "bee and puppycat"`.
### Extraction Argument
The `--extract` argument prints the playlist URL to your terminal instead of opening the player with the URL.
### Quality Argument
The `-q` or `--quality` argument allows you to specify a quality that will be selected if found, if not, it will default to the highest quality. Values for this argument look like `720p` or `auto`, so `-q 360p` would use the 360p URL if available.
### First Argument
The `-f` or `--first` argument selects the first result instead of displaying the series menu.
### Season Argument
The `-s` or `--season` argument selects the specified season instead of diplsaying the season menu.
### Episode Argument
The `-e` or `--episode` argument selects the specified episode instead of displaying the episode menu.
### Examples
If you wanted to, for example, play season 21 episode 3 of south park at 720p and you know it'll be the first result, you could use
```
mov-cli-rs "south park" -f -s 21 -e 3 -q 720p
```

## TODO
- [x] Playlist extract argument
- [x] Media FZF menu
- [x] Quality argument
- [x] Finish README
- [ ] Download argument
- [ ] More source resolving
- [ ] More player support
- [ ] Promotion poster display
- [ ] Better documented code

## Contribution
I'm new to having contributors on any of my projects, I will welcome it but may require guidance or assistance on how to structure the repository to make it easier to contribute.

<a href = "https://github.com/bzzzthe18th/mov-cli-rs/graphs/contributors">
  <img src = "https://contrib.rocks/image?repo=bzzzthe18th/mov-cli-rs"/>
</a>

## Credit
- Inspired by [ani-cli](https://github.com/pystardust/ani-cli) and [mov-cli](https://github.com/mov-cli/mov-cli)
- README referenced from [mov-cli](https://github.com/mov-cli/mov-cli)

[contributors-shield]: https://img.shields.io/github/contributors/bzzzthe18th/mov-cli-rs.svg?style=for-the-badge
[contributors-url]: https://github.com/bzzzthe18th/mov-cli-rs/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/bzzzthe18th/mov-cli-rs.svg?style=for-the-badge
[forks-url]: https://github.com/bzzzthe18th/mov-cli-rs/network/members
[stars-shield]: https://img.shields.io/github/stars/bzzzthe18th/mov-cli-rs?style=flat
[stars-url]: https://github.com/bzzzthe18th/mov-cli-rs/stargazers
[rust-shield]: https://img.shields.io/badge/Rust-latest-red?style=flat&logo=rust
[mpv-shield]: https://img.shields.io/badge/MPV-latest-520053?style=flat&logo=mpv
[issues-shield]: https://img.shields.io/github/issues/bzzzthe18th/mov-cli-rs?style=flat
[issues-url]: https://github.com/bzzzthe18th/mov-cli-rs/issues
[license-shield]: https://img.shields.io/github/license/bzzzthe18th/mov-cli-rs?style=flat
[license-url]: ./LICENSE