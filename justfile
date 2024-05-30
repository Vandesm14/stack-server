build:
  (cd backend && wasm-pack build --target web --release)
  (cp -r backend/pkg frontend/src)

run:
  (cd frontend && pnpm dev)
