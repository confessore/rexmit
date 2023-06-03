[![guild-badge][]][guild]
# rexmit

![rexmit logo][logo]

follow these instructions to deploy rexmit

## requirements

- git
- docker
- discord bot token

- cmake (only if you require rust-analyzer for dev)

## installation

1. clone this repository

2. change directory to this repository

3. create an .env file from .env.example

```
    DEBUG can be 1 for debug or 0 for release
    DISCORD_TOKEN sourced from discord developer portal
    DATABASE_URL can be a mongodb connection string or blank
    
    the idea is to feature support for mongodb but also to feature support for no database by leaving DATABASE_URL blank
```

4. build the image
    ```
    docker build -t rexmit .
    ```

5. run the container
    ```
    docker run --env-file .env -d rexmit
    ```

6. rexmit will connect to discord and be ready for guild invitations

7. some commands while in a voice channel to get started
    ```
    ~join
    ~leave
    ~mute
    ~q https://www.youtube.com/watch?v=jfKfPfyJRdk
    ~queue https://www.youtube.com/watch?v=jfKfPfyJRdk
    ~s
    ~skip
    ~stop
    ~c
    ~clear
    ```

## database

here is an example of running a document db


```
sudo docker run --name mongo -v ./data/db:/data/db \
	-p 27017:27017 --restart always \
	-e MONGO_INITDB_ROOT_USERNAME= \
	-e MONGO_INITDB_ROOT_PASSWORD= \
	-d mongo
```


[guild]: https://discord.gg/invite/95eUjKqT7e
[guild-badge]: https://img.shields.io/discord/1100799461581668372.svg?style=flat-square&colorB=7289DA
[logo]: https://repository-images.githubusercontent.com/538283283/d59b0b4f-63a7-429a-a5b7-44d067245e0c