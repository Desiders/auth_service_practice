# Sish (Client)

`sish` used only in development purposes for SSH tunneling.

## Configuration
You need to configure `sish` on remote server.

## Deployment
Just check [guide](https://docs.ssi.sh/).
</br>
Examples:
```bash
ssh -R {keycloak_subdomain}:80:localhost:{keycloak_port} -R {api_subdomain}:80:localhost:{reverse_proxy_port} tunnel.touhou.dad`
```
