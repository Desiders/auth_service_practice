# Reverse Proxy

## Configuration
As configuration example there are file `nginx.conf.template`.\
**You need to create** a `nginx.conf` file, and fill it in as specified in the template with own values.
You optionally need to update with own values:
- `http://127.0.0.1:4180`. It's an OAuth2-Proxy address. You need to update port if `API_PORT` in its `.env` has been changed.
- `http://127.0.0.1:3000/`. It's an API address. You need to update port if `rest_addr` in its config (`dev.toml` or `prod.toml`) has been changed.

## Deployment
You can run the project in development by script in `justfile`: `just run-dev` (with `sudo`).
If you want to run it in production, you probably need to copy `nginx.conf` file to `/usr/share/nginx` \
and use `systemctl` for scripts ([Guide](https://phoenixnap.com/kb/nginx-start-stop-restart)).
