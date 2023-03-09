# Ripinfo

> A personal tool to gather ip information using [ipinfo.io](https://ipinfo.io/)
> written in rust™

## Configuration

To use the ipinfo.io access token create/edit `ripinfo_config.json` located at

```sh
# Windows: C:\Users\Bob\AppData\Roaming\RipInfo\ripinfo\config\ripinfo_config.json
# Linux: /home/Bob/.config/ripinfo/ripinfo_config.json
# Mac: /Users/Bob/Library/Application Support/com.RipInfo.ripinfo/ripinfo_config.json
```

with the following content:

```json
{
  "use_token": true,
  "token": "<YOUT_ACCESS_TOKEN>"
}
```
