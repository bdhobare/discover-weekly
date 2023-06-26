# Discover Weekly Archive
A Rust service to save your Spotify Discover Weekly songs to another playlist before they disappear. Built on Actix-web. 
Deployed on AWS with Kubernetes and Terraform.

Running locally with Redis
1. Create an app on https://developer.spotify.com/dashboard and get your client id and secret.
2. Copy the `.env.template` file into `.env` file and replace the values inside accordingly:
```cp .env.template .env```
3. Use [ngrok](https://ngrok.com/download) or another service to expose the application to the internet: 
      `ngrok http 8080` and replace this value in your `.env` file(without the trailing `/`):

      ```HOME_URI=<ngrok url> ```

   Spotify will call this **callback** url after authentication.
4. Run the application locally
    ```
    cargo run -- 127.0.0.1 8080
    ```
5. Go back to the spotify dashboard and pass-list the ngrok URL:

![spotify.png](docs%2Fspotify.png)

6. Finally, go this url in the browser to authenticate your spotify account and allow the application to read and create playlists. Replace the client id and Ngrok url accordingly:

```localhost:8080```