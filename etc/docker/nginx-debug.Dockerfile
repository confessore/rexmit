FROM debian:bookworm-slim
RUN apt-get update -y
RUN apt-get install -y nginx
COPY etc/openssl/localhost.key ./etc/ssl/localhost.key
COPY etc/openssl/localhost.crt ./etc/ssl/localhost.crt
COPY etc/nginx/nginx-debug.conf ./etc/nginx/nginx.conf
COPY scripts/nginx-entrypoint-debug.sh .
RUN chmod +x ./nginx-entrypoint-debug.sh
ENTRYPOINT ["./nginx-entrypoint-debug.sh"]