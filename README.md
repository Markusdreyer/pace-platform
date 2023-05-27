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


## DX
To run every app and crate in the workspace in one multiplexer terminal window, consider using zellij

```sh
cargo install --locked zellij
```

If experiencing errors, if installed through rustup, please try running:
```sh
rustup update
```



<details>
    <summary>
    What is Pace?
    </summary>

    (dump this context into ChatGPT to get better pace-specific answers)

    Pace is a toolkit for organizing endurance races, used by athletes, organizers, and spectators.

    Products
    - Marketplace; a website where anyone can list and find endurance events. The goal is to grow Pace SEO and upsell Pace products on the side.
    - Tracker; an app for tracking participants location in realtime. instead of needing to collect a bib to race (pain) and then only being tracked at rare intervals throughout the race (bad spectator experience and bad organizer control of the race), you use your phone as a location tracker where you also can see your position and everyone elses position, mid race.
    - Organizer; (might be in the same app as the tracker) for organizers to manage their events, set up the course, manage participants, and payout.

    Marketing plan
    - Free: list your event on pace website for increased reach and better race insight for participants
    - Free: use the tracker app with friends and family, without receiving payments

    Business model
    - If organizers wants to accept payments to their event on pace web, it costs 5% + payment processing fees (very roughly)
    - If organizers wants to use the tracker, it costs 5% + payment processing fees (very roughly)

    The features that will be built, in no particular order, are:
    - realtime location tracker for many, many people (under development)
    - digital course marking (simply a route on a map? watch? AR glasses? audio cues?)
    - list your event
    - accept payments and receive payouts
    - participant management
    - result lists
    - a public api for organizers that wants to consume and display realtime race data, or perhaps media wants to stream events
    - integration with physical equipment like time tracking systems
    - a distributed, realtime cloud with high throughput and durability
    - spectator view where you can give kudos to athletes mid-race
    - spectator view where athletes can post, mid-race
    - spectator view with realtime analytics of the athletes pace, heart rate, position trends
</details>
