# Pace platform

This repo contains all services related to the pace platform.
## Backend
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
Run services by using the relevant `make` alias, e.g.:
```bash
make run-websockets
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

## Observability
As the services in this repo are designed to handle large amounts of real-time data, logging is insufficient for metrics. Instead, various observability tools are included to provide application insights (At the moment only prometheus). 

# Prometheus
Prometheus is an open-source system for collecting and processing metrics from applications, which expose data via a `/metrics` endpoint. It scrapes this data regularly, storing it in a time-series database, and can trigger alerts based on defined conditions.
The data exposed by the `/metrics`-endpoint are implemeted as a variety of data-types, such as counters. See example below:

```rust
//We initialise the references
//prom.rs
lazy_static {
    pub static ref DISCONNECTED_CLIENTS: IntGauge = IntGauge::new("disconnected_clients", "Disconnected Clients").expect("metric can be created");
}

//We add the functions to our prometheus register
//main.rs
REGISTRY.register(Box::new(DISCONNECTED_CLIENTS.clone())).expect("collector can be registered");


//This way, we can increment our counter wherever needed 
//race.rs
use::crate::prom::DISCONNECTED_CLIENTS

DISCONNECTED_CLIENTS.inc();
```
### Enable application insights

## Deployment
This project uses [fly.io](https://fly.io/) to host its backend services. As the services are still in their infancy, there is no CD pipeline setup, but this will come shortly after the services have stabilized somewhat. For now, manual deployments are the way to go, but fear not; it's super simple. 

**Install the flyctl CLI:**
```
brew install flyctl
```

**Login to the flyctl CLI:**
```
fly auth login
```
Still need to figure out how to give others access to my project. Hopefully you should get access if you're logging with the same email as I've added to the project.

**Deploy a new version:**
```
make deploy-websockets
```
This step uses an alias in the Makefile to tidy up the deployment command, but we're still using the flyctl CLI under the hood. Just check out the Makefile if you're curious about what's going on.

🕰️ Note that this step will unfortunately take an eternity to complete because Rust compilation combined with the Docker runtime without caching is not a great match at the moment. 🕰️


## App
Contains the native iOS app code. This is currently a very simple app showing the current location of the user on a map. The app also sends this location data to the pace backend with websockets.
