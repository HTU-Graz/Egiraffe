FROM node:18-slim AS frontend_builder
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
COPY frontend /frontend
WORKDIR /frontend

FROM rust:1-bookworm AS backend_builder
COPY backend .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=backend_builder /usr/local/cargo/bin/egiraffe /usr/local/bin/egiraffe
COPY --from=frontend_builder /frontend/dist /usr/local/frontend/dist
EXPOSE 42002
CMD ["egiraffe"]
