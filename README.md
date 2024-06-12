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
`sqlx migrate run`
`docker volume create e_transfer_postgres_data`
`docker network create e_transfer_dev`