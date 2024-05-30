build:
  (cd backend && wasm-pack build --target web --release)

run:
  (cd frontend && pnpm dev)