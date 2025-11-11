# Web Service Uncovr Example

The most basic Uncovr API - one endpoint, no complexity.

## What's Inside

Single GET endpoint that returns "Hello, World!"

## Run

```bash
cargo run --package simple
```

Visit:
- API: http://127.0.0.1:3000/hello
- Docs: http://127.0.0.1:3000/docs

## Test

```bash
curl http://127.0.0.1:3000/hello
```

Output: `Hello, World!`
