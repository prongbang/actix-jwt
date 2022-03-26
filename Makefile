run:
	cargo run src/main.rs

test_user_me:
	curl -X GET -H 'Content-Type: application/json' \
      -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwiZXhwIjoxNTE2MjM5MDIyMDAwMDAwfQ.rSO8TdzVPu95H54866GV78KhsUWocXO1ryklpXmR0OU' \
      -i 'http://127.0.0.1:8080/v1/user/me'

test_search:
	curl 'http://127.0.0.1:8080/v1/search'