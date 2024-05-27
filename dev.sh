set -x

parallel-sh "cd backend && cargo run --release" "cd frontend && pnpm build --watch" --verbose