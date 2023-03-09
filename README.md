# Ripinfo

> A personal tool to gather ip information using the
> [ipinfo.io](https://ipinfo.io/) API written in rust™

![command gif](./resources/gif.gif)

## Installation

Ripinfo can be installed using cargo

```sh
cargo install ripinfo
```

## Configuration

Ripinfo can get ip information in two modes: `demo mode` and `token mode`

### Demo Mode

- uses the demo found on the [ipinfo web site](https://ipinfo.io/)
- limited requests
- the response is the business plan response. More about responses here:
  [ipinfo responses](https://ipinfo.io/developers/responses)

### Token Mode

- uses the access token to get the data
- response depends on your plan
- requires you to configure `ripinfo_config.json`

### How to get an access token?

In order to get an access token you need an account on ipinfo.io. You can create
one here: https://ipinfo.io/login

Once you've created your account, you can check your access token here:
https://ipinfo.io/account/token

### Configure ripinfo_config.json

By default ripinfo uses demo mode, and in the case that `ripinfo_config.json`
doesn't exists ripinfo will create it.

To use the ipinfo.io access token create/edit `ripinfo_config.json` located at:

```sh
# Windows: C:\Users\<USER>\AppData\Roaming\RipInfo\ripinfo\config\ripinfo_config.json
# Linux: /home/<USER>/.config/ripinfo/ripinfo_config.json
# Mac: /Users/<USER>/Library/Application Support/com.RipInfo.ripinfo/ripinfo_config.json
```

Ripinfo uses the [directories crate](https://crates.io/crates/directories) to
get config directory.

Insert following content to `ripinfo_config.json`:

```json
{
  "use_token": true,
  "token": "<YOUR_ACCESS_TOKEN>"
}
```

## About Caching ⚠️

Every time a request is successful ripinfo stores the ip data inside
`ripinfo.json` (same directory as `ripinfo_config.json`).

So the next time you request a cached ip, ripinfo will return the cached ip
data. This is done to save some requests to ipinfo.io and to reduce usage of the
ipinfo API.

Since this used to be a personal tool I'd usually delete items from the cache
manually. So in the future I'm planning to add commands to bypass the cache and
delete specific items from the cache.
