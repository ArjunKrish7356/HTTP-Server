# http-server

A basic HTTP server implemented in Rust.

## Usage

1.  **Build the project:**
    ```bash
    cargo build --release
    ```

2.  **Run the server:**
    ```bash
    cargo run
    ```

3.  **Access the server:**
    Open your browser or use `curl` to access the server at `http://127.0.0.1:4221`.

## Endpoints

*   `/`: Returns a 200 OK response.
*   `/echo/<message>`: Echoes back the message in the response body.
*   `/user-agent`: Returns the User-Agent header from the request.
*   `/files/<filename>`: Serves files from the specified directory.
*   `POST /files/<filename>`: Creates a new file with the request body in the specified directory.

## License

This project is licensed under the MIT License.
