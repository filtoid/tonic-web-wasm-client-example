# tonic-web-wasm-client-example
An example of how to set up tonic web wasm client with a tonic based gRPC server 

Uses Yew for the front end and Tonic (with Tonic-Web) for the backend with tonic-web-wasm-client to glue them together. 

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

### Coming soon
I hope to update this to make it a bit more involved, including
database storage, sending data etc. But for now hopefully this helps 
people not have to dwell on problems that I had. 