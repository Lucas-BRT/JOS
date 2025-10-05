FROM alpine:3.22

RUN apk add --no-cache ca-certificates tzdata

COPY jos /jos

EXPOSE 3000

CMD ["/jos"]