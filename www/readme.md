# Roman Sunclock Time web app

## Dev setup

```bash
cd www
mkdir .secrets
openssl genrsa -out .secrets/https-key.pem 2048
openssl req -new -key .secrets/https-key.pem -out .secrets/https-csr.pem
openssl x509 -req -days 9999 -in .secrets/https-csr.pem -signkey .secrets/https-key.pem -out .secrets/https-cert.pem
rm .secrets/https-csr.pem
```
