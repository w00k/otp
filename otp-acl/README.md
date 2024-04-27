Collection 

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