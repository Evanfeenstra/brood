# build stage
FROM golang as builder

RUN update-ca-certificates

ENV GO111MODULE=on

WORKDIR /app

COPY go.mod .

RUN go mod download

COPY . .

# skip the Webview
RUN mv main.no tmp.no \
    && mv main.go main.no \
    && mv tmp.no main.go 

RUN go build

EXPOSE 3888
ENTRYPOINT ["./brood"]