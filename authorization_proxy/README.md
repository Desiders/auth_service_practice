# Authorization Proxy

## Configuration
As configuration example there are file `oauth2-proxy.cfg.template`.\
**You need to create** a `oauth2-proxy.cfg` file, and fill it in as specified in the template with own values.
You definitely need to specify your values for:
- `client_id`. ID of authorization server client;
- `client_secret`. Secret of authorization server client;
- `oidc_issuer_url`. `https://` + hostname of authorization server + `/realms/` + realm name;
- `cookie_secret`. [Guide](https://oauth2-proxy.github.io/oauth2-proxy/configuration/overview#generating-a-cookie-secret).

If you run project by `docker-compose.yaml`, you need to specify environment variables. An configuration example is `.env.template`, you need to create `.env` file and fill it in as specified in the template with own values.

## Deployment
You can run the project by script in `justfile`: `run`.
