# 프론트엔드 빌드
FROM node:20-alpine AS frontend
COPY frontend .
RUN yarn install
RUN yarn run vite build --outDir dist

# 러스트 빌드
FROM rust:1.73 AS backend
COPY backend .
RUN cargo build --release --bin docker

# 프로덕션 스테이지
FROM rust:1.73

# 파일 복사
COPY --from=frontend dist static

COPY --from=backend target/release/docker app
COPY --from=backend .env .env

ENTRYPOINT ["./app"]
