#!/usr/bin/env bash
(cd frontend && pnpm build)
(cd backend && cargo run --release)