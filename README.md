# rexmit


follow these instructions to deploy rexmit

## requirements

- git
- docker
- discord bot token

- cmake (only if you require rust-analyzer for dev)

- some operating systems may require modifying `/etc/hosts`
    ```
    127.0.0.1       localhost       www.localhost   api.localhost
    ```

## installation

1. clone this repository

2. change directory to this repository

3. enter environment variables
    ```
    chmod +x scripts/define-secrets.sh

    scripts/define-secrets.sh
    ```
    ```
    example:
        postgres-user = postgres
        postgres-password = postgres
        discord-token = <token>
    ```

4. create required docker volumes
    ```
    docker volume create rexmit-redis
    
    docker volume create rexmit-postgres
    ```

5. orchestrate the composition
    ```
    docker compose -f docker-compose-debug.yml up -d --build
    ```

6. rexmit will connect to discord and be ready for guild invitations

7. some commands while in a voice channel to get started
    ```
    ~join
    ~leave
    ~play https://www.youtube.com/watch?v=jfKfPfyJRdk
    ```

## features
- actix for backend api

- serenity with songbird for discord

- yew for frontend api

- nginx for http/https

- postgres for sql storage

- redis for caching