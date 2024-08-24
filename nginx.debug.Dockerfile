FROM debian:bookworm-slim
RUN apt update -y
RUN apt install -y nginx
COPY etc/nginx/nginx-debug.conf ./etc/nginx/nginx.conf
COPY scripts/nginx-entrypoint-debug.sh .
RUN chmod +x ./nginx-entrypoint-debug.sh
ENTRYPOINT ["./nginx-entrypoint-debug.sh"]