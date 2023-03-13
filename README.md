# saint-sorting

## how to run on your local

place firebase-service-account.json in /saint-sorting/

### 1. execute in 2steps
Terminal:
```
Cargo run
```

Browser:
access http://localhost:8080/app/top

### 2. execute in 1steps
Terminal:
```
cd /saint-sorting
./run_with_browser.zsh
```
## solve openssl installation
`sudo apt install libssl-dev pkg-config`

## ssl certificate
```
sudo apt install -y wget curl libnss3-tools
curl -s https://api.github.com/repos/FiloSottile/mkcert/releases/latest | grep browser_download_url | grep linux-amd64 | cut -d '"' -f 4 | wget -qi - \
    && mv mkcert-v*-linux-amd64 mkcert \
    && chmod a+x mkcert \
    && mv mkcert /usr/local/bin/
mkcert -install
mkcert localhost 127.0.0.1 ::1  # move these *.pem to /saint-sorting/
```
