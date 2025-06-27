# Notes

docker compose up -d

## DB

Connection à la DB : 
docker ps (pour récup l'id)
docker exec -it **ID** bash
psql -U hcount_user -d hcount -h localhost

pour jouer un fichier : psql -U hcount_user -d hcount -h localhost -f schema.sql

user: hcount_user
password : supersecret

## Frontend
cd frontend
 <!-- dx build --package frontend --release -->
 <!-- dx serve --package frontend --port 3000 -- -- --proxy "http://localhost:8080/api=>/api" -->
dx serve --package frontend --port 3000

## Backend
cd backend
cargo run --package backend

http://localhost:3000

## Docker
### Copier le script dans le conteneur et l'executer
docker cp schema.sql hcount-db-1:/tmp/schema.sql
docker exec -it hcount-db-1 psql -U hcount_user -d hcount -f /tmp/schema.sql

# Development

Your new workspace contains a member crate for each of the web, desktop and mobile platforms, a `ui` crate for shared components and a `api` crate for shared backend logic:

```
your_project/
├─ web/
│  ├─ ... # Web specific UI/logic
├─ desktop/
│  ├─ ... # Desktop specific UI/logic
├─ mobile/
│  ├─ ... # Mobile specific UI/logic
├─ api/
│  ├─ ... # All shared server logic
├─ ui/
│  ├─ ... # Component shared between multiple platforms
```

## Platform crates

Each platform crate contains the entry point for the platform, and any assets, components and dependencies that are specific to that platform. For example, the desktop crate in the workspace looks something like this:

```
desktop/ # The desktop crate contains all platform specific UI, logic and dependencies for the desktop app
├─ assets/ # Assets used by the desktop app - Any platform specific assets should go in this folder
├─ src/
│  ├─ main.rs # The entrypoint for the desktop app. It also defines the routes for the desktop platform
│  ├─ views/ # The views each route will render in the desktop version of the app
│  │  ├─ mod.rs # Defines the module for the views route and re-exports the components for each route
│  │  ├─ blog.rs # The component that will render at the /blog/:id route
│  │  ├─ home.rs # The component that will render at the / route
├─ Cargo.toml # The desktop crate's Cargo.toml - This should include all desktop specific dependencies
```

When you start developing with the workspace setup each of the platform crates will look almost identical. The UI starts out exactly the same on all platforms. However, as you continue developing your application, this setup makes it easy to let the views for each platform change independently.

## Shared UI crate

The workspace contains a `ui` crate with components that are shared between multiple platforms. You should put any UI elements you want to use in multiple platforms in this crate. You can also put some shared client side logic in this crate, but be careful to not pull in platform specific dependencies. The `ui` crate starts out something like this:

```
ui/
├─ src/
│  ├─ lib.rs # The entrypoint for the ui crate
│  ├─ hero.rs # The Hero component that will be used in every platform
│  ├─ echo.rs # The shared echo component that communicates with the server
│  ├─ navbar.rs # The Navbar component that will be used in the layout of every platform's router
```

## Shared backend logic

The workspace contains a `api` crate with shared backend logic. This crate defines all of the shared server functions for all platforms. Server functions are async functions that expose a public API on the server. They can be called like a normal async function from the client. When you run `dx serve`, all of the server functions will be collected in the server build and hosted on a public API for the client to call. The `api` crate starts out something like this:

```
api/
├─ src/
│  ├─ lib.rs # Exports a server function that echos the input string
```

### Tailwind
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o .web/assets/tailwind.css --watch
```


### Serving Your App

Navigate to the platform crate of your choice:
```bash
cd web
```

and serve:

```bash
dx serve
```


### SQLX

sqlx migrate add -r <name>
sqlx migrate run
sqlx migrate revert