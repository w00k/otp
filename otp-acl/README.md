## First Run

Setup the database, first you need to up docker compose. 

1. Up the database, go to the root folder (when is a file docker-compose.yml).
```bash
$ docker-compose up
```

2. Enter in the project **rust-orm-diesel** and download the CLI client for postgres.
```bash 
$ cargo install diesel_cli --no-default-features --features postgres
```
You must install libpq
```bash 
$ apt install libpq-dev or brew install libpq
```

3. Create and insert data into data bases 


3.1 Create a database connection.
```bash 
$ diesel migration run --database-url postgres://postgres:postgres@localhost:5432/otp
```

3.2 This is only for first run.
```bash 
$ diesel migration redo
```

## Collection 

Create OPT Key

```json
curl --request PUT \
  --url http://127.0.0.1:8080/create \
  --header 'Content-Type: application/json' \
  --header 'User-Agent: insomnia/8.6.1' \
  --data '{
	"otp_public_key": "oweirndfpb$jslkdfj&",
  "otp_private_key": "34567",
  "otp_user": "w00k",
  "retry": 3,
  "expiration_date": "2024-06-03T12:34:56.789",
  "otp_key_enable": true
}'
```

Validate OTP Key

```json
curl --request POST \
  --url http://127.0.0.1:8080/validate \
  --header 'Content-Type: application/json' \
  --header 'User-Agent: insomnia/8.6.1' \
  --data '{
	"otp_public_key": "oweirndfpb$jslkdfj&",
  "otp_private_key": "34567",
	"otp_user": "w00k"
}'
```