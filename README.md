# Pace platform

This repo contains all services related to the pace platform.
## pace-backend
The pace backend is written in Rust with a monorepo approach using cargo workspace to structure the microservices.

### Requirements
To compile and run the project, you will need the Rust toolchain (rustc, cargo, clippy etc.)

#### Installation
Install the Rust by following the instructions on the official [Rust website](https://www.rust-lang.org/tools/install).

*Note: Don't install Rust with Homebrew, it's a mess.*

#### Compile and Run
To get up and running, navigate into the `pace-backend`-folder and run the following commands:

Compile the project by running the following command:
```bash
cargo build
```
Run the project by running the following command:
```bash
cargo run
```

That's it! You should now be able to compile and run the project on your local machine. 

### Creating new backend services
If you want to add another service, for example `test-service`, run the following commands:

Create the service boilerplate:
```bash
cargo new test-service
```
This will create a new folder in the `pace-backend`-folder with all the necessary boilerplate to start fleshing out your service.

Edit the root Cargo.toml to include the `test-service`:
```js
[workspace]

members = [
    "websockets",
    "test-service",
]
```

If the service is not added to this file, then the Rust compiler can not find it and will not compile it.

## pace-app
Contains the native iOS app code. This is currently a very simple app showing the current location of the user on a map. The app also sends this location data to the pace backend with websockets.
