FROM debian:bookworm-slim
RUN apt update -y
RUN apt install -y nginx certbot python3-certbot-nginx cron
COPY etc/nginx/nginx-release.conf .
COPY scripts/nginx-entrypoint-release.sh .
RUN chmod +x ./nginx-entrypoint-release.sh
ENTRYPOINT ["./nginx-entrypoint-release.sh"]