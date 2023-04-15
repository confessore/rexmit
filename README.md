# rexmit

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