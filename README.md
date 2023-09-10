## Structure
- `src/main.rs` - Responsible for launching the webserver and configuring logging.
- `src/db` - Contains code responsible for accessing the database.
- `src/api` - Defines the API endpoints.
- `src/frontend` - Responsible for serving the frontend.
- `src/models` - Contains relevant datatypes used throughout the server.
- `templates` - Contains html templates that are served by the frontend.
- `static` - Contains static files that will be served by the webserver.

## Endpoints
### API (`/api`)
- `GET /articles`
- `POST /article`
- `GET /users`
- `GET /user/:id`
- `POST /user`
### Frontend (`/`)
- `GET /users`
- `GET /articles` with optional query parameter `author_id`
