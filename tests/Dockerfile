FROM alpine:3.22.1

RUN addgroup -g 9000 app && \
    adduser -u 9000 -D -G app -h /app app

COPY --chmod=555 ./sleipnir /app/sleipnir

WORKDIR /app

USER app

CMD ["/app/sleipnir"]
