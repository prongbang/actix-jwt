# actix-jwt

### Start Server

```shell
cargo run src/main.rs
```

### Get user me with JWT

```shell
curl -X GET -H 'Content-Type: application/json' \
  -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwiZXhwIjoxNTE2MjM5MDIyMDAwMDAwfQ.rSO8TdzVPu95H54866GV78KhsUWocXO1ryklpXmR0OU' \
  -i 'http://127.0.0.1:8080/v1/user/me'
```

Output

```shell
I am Devไปวันๆ
```

### Search without JWT

```shell
curl http://127.0.0.1:8080/v1/search
```

Output

```shell
Search: Devไปวันๆ
```