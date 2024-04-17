# 🦀 Rust Axum Production code 🚀

🚀 This project is a comprehensive guide to building a production-ready web server using the Rust language and the Axum framework. It covers essential features that any production server should possess, including:

- 🔧 Environment variable-based configuration.
- 🧪 Client request testing for server verification.
- 🔑 Token generation and validation for secure transactions.
- 🔒 Password encryption and validation for enhanced security.
- ❗ Detailed error handling to ensure robustness.
- 🕵️‍♂️ Context handling for user authentication.
- 📝 Logging of each request and response for traceability.
- 🚦 Appropriate client response status codes.
- 🗄️ Database connection pooling for efficient resource utilization.
- 🧰 Test connections to the database for reliability.
- 📚 Generic-based CRUD operations in Rust for clean, maintainable code.
- 🏛️ Clean architecture principles applied to the model layer (database).
- 📡 JSON RPC architecture for handling requests.
- ✅ Unit tests for each module to ensure code quality.
- 🛡️ Middleware for user authentication using a context (ctx) extractor.
- 📊 Middleware for logging each request and response.
- 🛣️ Routes for handling file and JSON requests.
- 📦 Multi-service architecture for hosting multiple services within the same project.
- 🔀 Dynamic switching between hashing algorithms for enhanced password encryption.

These features form the foundation of a production server. The server, as built, is deployment-ready. 🎉

This project was developed following the tutorials on Jeremy Chone's YouTube channel. Here is the [link](https://www.youtube.com/watch?v=XZtlD_m59sM&list=PL7r-PXl6ZPcCTTxjmsb9bFZB9i01fAtI7&index=8&ab_channel=JeremyChone) to the playlist. 📺

## Completed Episode 01

In this episode, we accomplished the following tasks:

- *Axum Server Setup*: We set up the basic structure of the Axum Server.

- *Logging*: Implemented logging using the `tracing` and `tracing-subscriber` crates.

- *Configuration*: Configured the server using environment variables in the `.cargo/config.toml` file and the `config.rs` file.

- *Database Connection Test*: Created a test to verify the database connection using the `sqlx` crate. This is located in the `_dev_utils` module.

- *Model Layer*: Developed the model layer with the store in the `models` module. This includes some unit tests.

- *CRUD Operations*: Implemented the base CRUD operations with Generics in the `base` module. We used the `DbBmc` struct to identify the entity to work with and the `sqlb` crate to build the queries and manipulate the fields.

- *User Authentication*: Created user login, encryption, and validation functionality using the `Hmac`, `sha2`, and `base64-url` crates.

- *Token Generation and Validation*: Implemented token generation and validation using the `base64-url` and `time` crates. This allows us to store an identifier, expiration time, and a signature. We also added the token into the response header.

- *Server Routes*: Created routes for the server to manipulate the *task* entity. We used the **json rpc** API architecture with the `axum` and `serde_json` crates. This is handled in the `rpc` and `rpc_task` modules.

## Completed Episode 02

In this episode, we accomplished the following task:

- *Database Query Improvement*: We transitioned from using `sqlx`/`sqlb` to `sea-query`/`modql`. This change was made to enhance our capabilities in building, manipulating, and filtering database queries.

- *Modql & filters*: Using the `modql` crate, we were able to make a filter using certain `modql` operators in the JSON client request. This allows us to filter the results by using the `ListOptions` and `FilterGroups` types of `modql`.

## Completed Episode 03

In this episode, we accomplished the following tasks:

- *Destruct the project to make multi service arch*: We've restructured the project to adopt a multi-service architecture. This approach enables us to host multiple services within the same project while leveraging shared libraries across the workspace.

- *Utilizing Wrapper Pattern in `CtxW`*: We've employed a wrapper pattern within the `CtxW` struct to manage the user context in `Ctx`. This strategy facilitates the extraction of context from the request, allowing its utilization within the server.

## Completed Episode 04

In this episode, we accomplished the following tasks:

- *Refactor `pwd` module*: Refactored the `pwd` module to enhance the migration to other hashing algorithms. This restructuring enables the seamless transition to other hashing algorithms without affecting the existing codebase.

- *Implement `argon2` hashing*: Implemented the `argon2` hashing algorithm within the `pwd` module.

- *Updating legacy scheme to current scheme*: Updated the legacy hashing scheme to the current hashing algorithm to ensure the seamless transition to the new hashing algorithm.

- *Using `static dispatch` instead of `dynamic dispatch` for selecting the hashing algorithm*: Employed a `static dispatch` mechanism instead of `dynamic dispatch` to select the hashing algorithm. This is done in the `lib-auth/pwd/scheme` module.

- *Optimization in `hash_pwd` and `validate_pwd`*: Made the `hash_pwd` and `validate_pwd` functions async and add the `tokio::task::spawn_blocking` to handle the functions in an special thread so the other threads can continue to work without waiting for the hashing or validation to finish.

## Completed Episode 05

In this episode, we accomplished the following tasks:

- *Developed a standalone `lib_rpc` router*: We constructed a standalone `lib_rpc` router to manage JSON RPC requests. This router is tasked with processing both the incoming JSON RPC requests and the corresponding responses.

## Completed Episode 06

In this episode, we accomplished the following tasks:

- *Enhanced database connectivity*: We refined our database connection by allowing the `store` module to handle all database transactions. Additionally, we implemented robust error handling for these transactions.

## Deployment

This section provides instructions on how to deploy and test the server.

### Running the Server

To start the main server, use the following command:

```shell
cargo watch -c -q -w .\crates\services\web-server\src\ -w .\crates\libs\ -w .\.cargo\ -x "run -p web-server"
```

This command will start the server and watch the `src/` and `.cargo/` directories for changes. The server will automatically restart when changes are detected.

### Sending Test Requests to Server

To send some requests to the server for testing purposes, use the following command:

```shell
cargo watch -q -c -w .\crates\services\web-server\examples\ -x "run -p web-server --example quick_dev"
```

This command will run the `quick_dev` example, which sends requests to the server. It also watches the `examples/` directory for changes and reruns the example when changes are detected.

### Running Unit Tests

To run the unit tests, use the following command:

```shell
cargo watch -q -c -x test
```

This command will run all the tests in the project and watch for changes. The tests will be rerun when changes are detected.

### Generating a Base64-URL Key

To generate a base64-url key, use the following command:

```shell
cargo run -p gen_key
```

This command will run the `gen_key` example, which generates a base64-url key and prints it to the console.

### Safe commit
