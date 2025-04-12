# Code Review: `src/main.rs`

**Review Date:** 2025-04-11

Overall, this is a great start to building an HTTP server in Rust! You've successfully handled basic requests, routing, and even concurrency with a thread pool.

**Positive Observations:**

1.  **Concurrency Handling:** Great job using `rayon` for a thread pool (`ThreadPoolBuilder`)! This is a good approach for handling multiple client connections concurrently without blocking the main thread. It shows you're thinking about performance early on.
2.  **Clear Routing Logic:** The `match` statement in `handle_request` provides a clear and readable way to handle different routes (`/`, `/echo/*`, `/user-agent`). Using `strip_prefix` for the `/echo/` route is also a nice touch for extracting parameters cleanly.
3.  **Error Handling:** You've included basic error handling for stream reading and writing, logging errors using `eprintln`. This is crucial for diagnosing issues.
4.  **Header Parsing:** The `extract_headers` function correctly attempts to parse the request line and headers into a `HashMap`. Using `split_once` for headers is efficient.

**Areas for Learning & Improvement:**

1.  **Learning Objective:** Robust HTTP Request Parsing
    *   **Observation:** The current `extract_headers` function assumes a well-formed request and doesn't handle the request body or all edge cases.
    *   **Suggestion:** Consider using a dedicated HTTP parsing library (like `httparse` or `hyper`) for more robust parsing.
    *   **Why:** Libraries handle protocol complexities and edge cases, making your server more reliable than manual string splitting.
    *   **Resource:** [`httparse` Crate Documentation](https://docs.rs/httparse/latest/httparse/)

2.  **Learning Objective:** Refining Error Handling
    *   **Observation:** Returning `std::io::Error` directly lacks context for application-level errors.
    *   **Suggestion:** Define a custom error enum (e.g., `ServerError`) to differentiate error types (`IoError`, `ParseError`).
    *   **Why:** Custom errors provide semantic meaning, clearer logic, and allow for specific handling (e.g., returning different HTTP status codes).
    *   **Resource:** [Error Handling in Rust Book](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

3.  **Learning Objective:** Code Clarity and Idiomatic Rust
    *   **Observation:** The `extract_headers` function mixes request-line parsing (method, path, version) with header parsing by storing request-line components in the `headers` map.
    *   **Suggestion:** Separate the parsing of the request line from header parsing. Store method, path, and version in dedicated variables or a struct.
    *   **Why:** Improves clarity. The `headers` map should ideally only contain actual HTTP headers.

**Summary & Next Steps:**

You've built a functional foundation. Focus on making request handling more robust (consider a parsing library) and refining error management (custom error types).

Keep up the great work!