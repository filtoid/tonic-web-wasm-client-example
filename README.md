# tonic-web-wasm-client-example
An example of how to set up tonic web wasm client with a tonic based gRPC server 

Uses Yew for the front end and Tonic for the backend with tonic-web-wasm-client to glue them together. 

## Yew applications
To run Yew applications you need to install [trunk](https://trunkrs.dev/) 

```bash
cargo install --locked trunk
```

## Running the application
You will need two console windows

```bash 
cd server
cargo run
```

```bash
cd client
trunk serve
```

Then navigate to [http://localhost:8080](http://localhost:8080) and you can hit the button to get a message from the server