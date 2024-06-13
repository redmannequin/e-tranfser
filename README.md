# e-tranfser


```bash
DB_NAME="db_name"
DB_USERNAME="db_username"
DB_PASSWORD="db_password"

TL_ENVIORNMENT="sandbox|production"
TL_CLIENT_ID="truelayer client id"
TL_CLIENT_SECRET="truelayer client secret"
TL_KID="truelayer client certificate id"
TL_MERCHANT_ACCOUNT_ID="truelayer client merchant account id"
TL_PRIVATE_KEY="truelayer client private key"
TL_REDIRECT_URI="app truelayer redirect uri"
TL_DATA_REDIRECT_URI="app truelayer data dedirect uri"
```

`docker-compose --env-file sbx.env build`

`docker-compose --env-file sbx.env up`

`docker build --target sqlx-migrator -t sqlx-migrator .`

`docker run --env-file sbx.env --network e_transfer_dev sqlx-migrator`

`docker volume create e_transfer_postgres_data`

`docker network create e_transfer_dev`

## Setup Linode Instance

image: debian 11
update and install required packages
```
sudo apt-get update
sudo apt upgrade

sudo apt-get update
sudo apt remove docker docker-engine docker.io
sudo apt install apt-transport-https ca-certificates curl gnupg lsb-release curl -fsSL https://download.docker.com/linux/debian/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg

echo "deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

sudo apt update
sudo apt install docker-ce docker-ce-cli containerd.io
sudo systemctl start docker
sudo systemctl enable docker
sudo systemctl enable containerd

sudo apt install nginx
sudo apt install certbot python3-certbot-nginx

"server {
    listen 80 default_server;
    listen [::]:80 default_server;

    server_name domain.top_level_domain ; # Adjust according to your setup

    # Basic root directive or proxy_pass directive
    location / {
        proxy_pass http://localhost:8080;  # Adjust according to your setup
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}" > /etc/nginx/sites-available/default

sudo systemctl reload nginx

sudo certbot --nginx -d domain.top_level_domain

sudo systemctl reload nginx
```