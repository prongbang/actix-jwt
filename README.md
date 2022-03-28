# actix-jwt

### JWT Secret key

```
secret69
```

### Start Server

- Run with main.rs

```shell
cargo run src/main.rs
```

- Run with docker

```shell
docker build -t actix_jwt .
docker run -d -p 8080:8080 actix_jwt
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

### Document

https://doc.rust-lang.org/rust-by-example/std/arc.html

- `Box` is for single ownership. A great use case is to use this when we want to store primitive types (stored on stack) on heap.
- `Rc` is for multiple ownership.
- `Rc` เป็น smart pointer ที่ย้ายข้อมูลจาก stack ไปอยู่ใน heap มันช่วยให้เราโคลน smart pointer 
- `Rc` ตัวอื่นๆ โดยทุกตัวจะสามารถ ยืมโดยไม่เปลี่ยนแปลง ข้อมูลที่อยู่ใน heap ได้ เมื่อ smart pointer ตัวสุดท้ายถูก drop เท่านั้นจึงจะคืนหน่วยความจำใน heap
- `Rc<RefCell<T>>` is Shared ownership with interior mutability and dynamically checked borrow rules.
- `Arc` is for multiple ownership, but threadsafe.
- `Arc<Mutex<RefCell<T>>>` is Thread-safe shared ownership with interior mutability and mutual exclusion.
- `RefCell` is for “interior mutability”; that is, when you need to mutate something behind a &T.
- `Cell` is for “interior mutability” for Copy types; that is, when you need to mutate something behind a &T. Cell, is similar to RefCell except that instead of giving references to the inner value, the value is copied in and out of the Cell.
- `Mutex`, which offers interior mutability that’s safe to use across threads
