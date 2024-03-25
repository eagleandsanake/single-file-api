FROM debian
WORKDIR /app
COPY ./single-file-api /app/single-file-api
COPY ./single-file /app/single-file
RUN apt-get update && apt-get install -y chromium && which chromium
RUN chmod 777 /app/single-file-api && chmod 777 /app/single-file
ENTRYPOINT ["/app/single-file-api"]