FROM alpine:3.22

RUN apk add --no-cache ca-certificates tzdata

WORKDIR /app

COPY target/release/jos .

RUN chmod +x ./jos

EXPOSE 3000

CMD ["./jos"]
