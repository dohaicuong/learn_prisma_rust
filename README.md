# Setup prisma rust client

## Rust Nightly
prisma rust client now has to run in rust nightly, to switch to rust nightly
```
rustup default nightly
```

## Prisma CLI
able to run prisma rust cli with
```
cargo prisma --your_command--
```

the config is in `.cargo/config.toml`

ref: https://github.com/Brendonovich/prisma-client-rust/blob/main/docs/01-installation.md

## Run app
in `src/main.rs`
```
cargo run
```

## Simple CRUD post
- create post
```
curl -X POST http://localhost:3000/api/post \
    -H 'Content-Type: application/json' \
    -d '{ "title": "Some title", "content": "Some content" }'
```

- get posts
```
curl http://localhost:3000/api/post
```

- get post
```
curl http://localhost:3000/api/post/:post_id
```

- update post

`TODO: found a convenient way to optional update a required field`

```
curl -X PUT http://localhost:3000/api/post/:post_id \
    -H 'Content-Type: application/json' \
    -d '{ "content": "Some content" }'
```

- delete post
```
curl -X DELETE http://localhost:3000/api/post/:post_id
```