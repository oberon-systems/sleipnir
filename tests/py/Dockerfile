FROM python:3.12.11-slim

RUN groupadd -g 9000 app && \
    useradd -d /app -g 9000 -u 9000 -s /usr/sbin/nologin app

COPY ./* /app/

WORKDIR /app
USER app

ENTRYPOINT ["python3.12"]

CMD ["/app/generator.py"]
