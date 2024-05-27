#!/usr/bin/env bash
(cd frontend && pnpm i && pnpm build)
(cd backend && cargo build --release && cargo run --release)