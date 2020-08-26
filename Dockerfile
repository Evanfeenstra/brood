# build stage
FROM golang as builder

ENV GO111MODULE=on

WORKDIR /app

COPY go.mod .
COPY go.sum .

RUN go mod download

COPY . .

RUN apt-get update && \
    apt-get install libwebkit2gtk-4.0-dev libgtk-3-dev

RUN go build

# final stage
FROM alpine:latest

RUN apk add --no-cache ca-certificates openssl

COPY --from=builder /app/brood /app/

RUN ls app

EXPOSE 8089
ENTRYPOINT ["/app/brood"]