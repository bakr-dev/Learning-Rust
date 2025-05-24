// This file covers fundamental Rust Asynchronous Programming concepts, explaining
// why it's necessary for concurrent operations, how it differs from traditional
// multi-threading, and how to use async/await for non-blocking I/O.

// -------------------------------------------------------------------------
// 0. The Problem Asynchronous Programming Solves: Efficient I/O and Concurrency
// -------------------------------------------------------------------------
// Traditional synchronous programming blocks the execution of a program
// while waiting for I/O operations (like network requests, file reads,
// or database queries) to complete. This can lead to inefficient use of
// CPU resources, as the program sits idle.
//
// While multi-threading can address this by running I/O operations in
// separate threads, it introduces significant complexity around shared state,
// synchronization (locks, mutexes), and higher memory/CPU overhead per thread.
//
// Asynchronous programming allows a single thread to manage multiple
// concurrent I/O operations without blocking. Instead of waiting, the program
// "yields" control and tells the runtime to notify it when the I/O is ready.
// This is ideal for applications that spend a lot of time waiting for external
// resources (e.g., web servers, proxies, streaming applications).

/*
// Illustrative (synchronous, blocking) example:
// Imagine this takes 5 seconds to complete. The entire program pauses.
fn fetch_data_sync() -> String {
    println!("Fetching data synchronously...");
    std::thread::sleep(std::time::Duration::from_secs(5)); // Simulate network delay
    println!("Data fetched synchronously!");
    String::from("Synchronous Data")
}

fn main() {
    println!("--- Rust Asynchronous Programming: Non-Blocking Concurrency ---");
    println!("Starting synchronous operation...");
    let data = fetch_data_sync();
    println!("Synchronous result: {}", data);
    println!("Synchronous operation finished. This line only runs after the fetch completes.");
    // In a real application, the UI would freeze or the server would stop responding during `Workspace_data_sync`.
}
*/

// To run async code, you need an asynchronous runtime.
// The most popular one in Rust is `tokio`. Add this to your `Cargo.toml`:
// [dependencies]
// tokio = { version = "1", features = ["full"] } // "full" for convenience, narrow down features for production

#[tokio::main] // This macro transforms `main` into an async function and sets up the tokio runtime
async fn main() {
    println!("--- Rust Asynchronous Programming: Non-Blocking Concurrency ---");

    // -------------------------------------------------------------------------
    // 1. What is Asynchronous Programming? Futures and Non-Blocking I/O
    // -------------------------------------------------------------------------
    // Asynchronous programming in Rust is built around the concept of `Futures`.
    // A `Future` is a trait that represents an asynchronous computation that
    // may complete at some point in the future. It's similar to a "promise"
    // in JavaScript or a "Task" in C#.
    //
    // When you call an `async` function, it doesn't immediately execute its
    // entire body. Instead, it returns a `Future`. This `Future` can then be
    // "polled" by an asynchronous runtime (like Tokio, async-std, etc.) to
    // check its progress. The runtime manages the execution of multiple futures
    // concurrently on a limited number of threads.

    println!("\n--- 1. What is Asynchronous Programming? Futures & Non-Blocking I/O ---");
    println!(
        "`async` functions return `Future`s, which represent a value that will be available later."
    );
    println!("An async runtime executes and polls these futures.");

    // -------------------------------------------------------------------------
    // 2. The `async`/`await` Keywords: Syntactic Sugar for Futures
    // -------------------------------------------------------------------------
    // The `async` and `await` keywords provide ergonomic syntax for writing
    // asynchronous code, making it look and feel more like synchronous code.
    //
    // - `async fn`: Marks a function as asynchronous. It returns a `Future`.
    //   The code inside an `async fn` can contain `await` expressions.
    // - `.await`: Pauses the execution of the current `async` function until
    //   the `Future` it's `await`ing completes. While paused, the runtime can
    //   switch to execute other pending futures.

    println!("\n--- 2. The `async`/`await` Keywords ---");

    // An `async` function. Notice the `async` keyword before `fn`.
    async fn fetch_data_async(id: u32) -> String {
        println!("[Task {}] Fetching data asynchronously...", id);
        // Simulate a non-blocking I/O operation (e.g., network request)
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await; // `.await` here!
        println!("[Task {}] Data fetched asynchronously!", id);
        format!("Asynchronous Data from Task {}", id)
    }

    // Call the async function. It returns a Future.
    let future1 = fetch_data_async(1);
    let future2 = fetch_data_async(2);

    println!("Futures created, but not yet executed.");
    println!("This line runs immediately after `Workspace_data_async` returns Futures.");

    // `await` the futures to get their results.
    // The `await!` points tell the runtime where it can switch tasks.
    let data1 = future1.await; // Program pauses here, allowing other futures to run
    let data2 = future2.await; // Program pauses here, allowing other futures to run

    println!("Result from Task 1: {}", data1);
    println!("Result from Task 2: {}", data2);
    println!("All asynchronous operations finished.");

    // Notice how "[Task 1] Fetching..." and "[Task 2] Fetching..." print
    // almost simultaneously, then after a 2-second delay, both "Data fetched!"
    // messages appear, demonstrating concurrent execution.

    // -------------------------------------------------------------------------
    // 3. Spawning Tasks: Running Futures Concurrently
    // -------------------------------------------------------------------------
    // To run multiple `Future`s truly concurrently (in parallel if multiple
    // CPU cores are available, or interleaved if on a single core), you need
    // to "spawn" them onto the async runtime. The `tokio::spawn` function
    // takes a `Future` and schedules it for execution. It returns a `JoinHandle`.

    println!("\n--- 3. Spawning Tasks: Running Futures Concurrently ---");

    async fn background_task(name: &str, delay_secs: u64) -> String {
        println!("[{}] Starting...", name);
        tokio::time::sleep(tokio::time::Duration::from_secs(delay_secs)).await;
        println!("[{}] Finished!", name);
        format!("Result from {}", name)
    }

    let handle1 = tokio::spawn(background_task("Task A", 3)); // Spawn a task
    let handle2 = tokio::spawn(background_task("Task B", 1)); // Spawn another task
    let handle3 = tokio::spawn(background_task("Task C", 2)); // Spawn a third task

    println!("Main function continues while tasks are running in background.");

    // `await`ing the `JoinHandle` blocks the current async function until
    // the spawned task completes.
    let result_a = handle1.await.expect("Task A failed");
    let result_b = handle2.await.expect("Task B failed");
    let result_c = handle3.await.expect("Task C failed");

    println!(
        "Collected results: {}, {}, {}",
        result_a, result_b, result_c
    );
    println!("All spawned tasks completed.");

    // Observe the output: "Task B Finished!" will likely appear before "Task A Finished!"
    // even though Task A was spawned first, because Task B has a shorter delay.
    // This highlights the non-blocking, concurrent nature.

    // -------------------------------------------------------------------------
    // 4. Asynchronous I/O Operations
    // -------------------------------------------------------------------------
    // The power of async programming comes from its use with I/O-bound operations.
    // Asynchronous runtimes provide their own versions of I/O primitives that
    // are non-blocking. For example, `tokio::fs` for file operations, `tokio::net`
    // for network operations, `tokio::io` for general I/O traits.

    println!("\n--- 4. Asynchronous I/O Operations ---");

    use tokio::fs; // For asynchronous file operations
    use tokio::io::{self, AsyncReadExt, AsyncWriteExt}; // For async I/O traits

    let file_path = "async_example.txt";
    let content = "Hello from async Rust!";

    async fn write_and_read_file(path: &str, data: &str) -> io::Result<String> {
        println!("Writing to file: {}", path);
        let mut file = fs::File::create(path).await?; // Create file asynchronously
        file.write_all(data.as_bytes()).await?; // Write asynchronously
        println!("Finished writing to file.");

        println!("Reading from file: {}", path);
        let mut file = fs::File::open(path).await?; // Open file asynchronously
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?; // Read asynchronously
        println!("Finished reading from file.");

        Ok(String::from_utf8_lossy(&buffer).into_owned())
    }

    // Spawn the file operation as a task
    let file_handle = tokio::spawn(write_and_read_file(file_path, content));

    // Do other work while file I/O is happening
    println!("Performing other tasks while file I/O is in progress...");
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    println!("Other tasks continue...");

    // Await the file operation result
    match file_handle.await {
        Ok(Ok(read_content)) => println!("Content read from file: '{}'", read_content),
        Ok(Err(e)) => eprintln!("File operation error: {}", e),
        Err(e) => eprintln!("Task join error: {}", e), // Error from `tokio::spawn` itself
    }

    // Clean up the created file (synchronously for simplicity here)
    if let Err(e) = std::fs::remove_file(file_path) {
        eprintln!("Failed to clean up file {}: {}", file_path, e);
    }

    // -------------------------------------------------------------------------
    // 5. Channels for Async Communication (Brief Mention)
    // -------------------------------------------------------------------------
    // When you have multiple async tasks, you often need them to communicate.
    // Asynchronous channels (e.g., `tokio::sync::mpsc` for multi-producer, single-consumer)
    // are used for safe, non-blocking communication between tasks.

    println!("\n--- 5. Channels for Async Communication ---");

    use tokio::sync::mpsc;

    async fn producer(sender: mpsc::Sender<String>) {
        for i in 0..3 {
            let msg = format!("Message {}", i);
            println!("[Producer] Sending: {}", msg);
            sender.send(msg).await.expect("Failed to send message"); // Non-blocking send
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }

    async fn consumer(mut receiver: mpsc::Receiver<String>) {
        while let Some(msg) = receiver.recv().await {
            // Non-blocking receive
            println!("[Consumer] Received: {}", msg);
        }
        println!("[Consumer] Channel closed.");
    }

    let (tx, rx) = mpsc::channel(10); // Create an async channel with a buffer of 10
    tokio::spawn(producer(tx));
    tokio::spawn(consumer(rx))
        .await
        .expect("Consumer task failed"); // Await consumer to finish

    println!("\n--- End of Asynchronous Programming Examples ---");
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////////

// This file covers fundamental Rust Asynchronous Programming concepts, explaining
// why it's necessary for concurrent operations, how it differs from traditional
// multi-threading, and how to use async/await for non-blocking I/O.
// Rust supports async/await like many other modern languages, providing a
// powerful way to write concurrent code without the overhead of traditional threads.

// To run async code, you need an asynchronous runtime.
// The most popular one in Rust is `tokio`. Add this to your `Cargo.toml`:
// [dependencies]
// tokio = { version = "1", features = ["full"] } // "full" for convenience, narrow down features for production

// Import required headers for async operations
use std::future::Future;
use tokio::fs; // For asynchronous file operations
use tokio::io::{self, AsyncReadExt, AsyncWriteExt}; // For async I/O traits
use tokio::sync::mpsc; // For asynchronous channels // Required for `impl Future` examples

#[tokio::main] // This macro transforms `main` into an async function and sets up the tokio runtime
async fn main() {
    println!("--- Rust Asynchronous Programming: Non-Blocking Concurrency ---");

    // -------------------------------------------------------------------------
    // 0. The Problem Asynchronous Programming Solves: Efficient I/O and Concurrency
    // -------------------------------------------------------------------------
    // Traditional synchronous programming blocks the execution of a program
    // while waiting for I/O operations (like network requests, file reads,
    // or database queries) to complete. This can lead to inefficient use of
    // CPU resources, as the program sits idle.
    //
    // While multi-threading can address this by running I/O operations in
    // separate threads, it introduces significant complexity around shared state,
    // synchronization (locks, mutexes), and higher memory/CPU overhead per thread.
    //
    // Asynchronous programming allows a single thread to manage multiple
    // concurrent I/O operations without blocking. Instead of waiting, the program
    // "yields" control and tells the runtime to notify it when the I/O is ready.
    // This is ideal for applications that spend a lot of time waiting for external
    // resources (e.g., web servers, proxies, streaming applications).

    // -------------------------------------------------------------------------
    // 1. What is Asynchronous Programming? Futures and Non-Blocking I/O
    // -------------------------------------------------------------------------
    // Asynchronous programming in Rust is built around the concept of `Futures`.
    // A `Future` is a trait that represents an asynchronous computation that
    // may complete at some point in the future. It's similar to a "promise"
    // in JavaScript or a "Task" in C#.
    //
    // When you call an `async` function, it doesn't immediately execute its
    // entire body. Instead, it returns a `Future`. This `Future` can then be
    // "polled" by an asynchronous runtime (like Tokio, async-std, etc.) to
    // check its progress. The runtime manages the execution of multiple futures
    // concurrently on a limited number of threads.

    println!("\n--- 1. What is Asynchronous Programming? Futures & Non-Blocking I/O ---");
    println!(
        "`async` functions return `Future`s, which represent a value that will be available later."
    );
    println!("An async runtime executes and polls these futures.");

    // -------------------------------------------------------------------------
    // 2. The `async`/`await` Keywords: Syntactic Sugar for Futures
    // -------------------------------------------------------------------------
    // The `async` and `await` keywords provide ergonomic syntax for writing
    // asynchronous code, making it look and feel more like synchronous code.
    //
    // - `async fn`: Marks a function as asynchronous. It returns a `Future`.
    //   The code inside an `async fn` can contain `await` expressions.
    // - `.await`: Pauses the execution of the current `async` function until
    //   the `Future` it's `await`ing completes. While paused, the runtime can
    //   switch to execute other pending futures.

    println!("\n--- 2. The `async`/`await` Keywords ---");

    // Write the first asynchronous function header
    async fn fetch_data_async1(id: u32) -> String {
        println!("[Task {}] Fetching data asynchronously...", id);
        // Sleep in the function: Simulate a non-blocking I/O operation (e.g., network request)
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await; // `.await` here!
        println!("[Task {}] Data fetched asynchronously!", id);
        // Return a value from the function
        format!("Asynchronous Data from Task {}", id)
    }

    // Create a second similar function
    async fn fetch_data_async2(id: u32) -> String {
        println!("[Task {}] Fetching data asynchronously...", id);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Shorter delay
        println!("[Task {}] Data fetched asynchronously!", id);
        format!("Asynchronous Data from Task {}", id)
    }

    println!("\n--- Calling Async Functions (Initial Blocking Scenario) ---");
    println!("Calling async functions without `.await` returns Futures, it doesn't execute them.");

    // Call the first API and print its result out
    // If we just called `fetch_data_async1(1);` it would return a Future, but not run it.
    // To execute the Future and get its result, we must `.await` it.
    // At the moment, API 1 blocks API 2 because `await` is called sequentially.
    let data1 = fetch_data_async1(1).await;
    println!("Result from Task 1: {}", data1);

    // Do the same thing with the second API
    let data2 = fetch_data_async2(2).await;
    println!("Result from Task 2: {}", data2);
    println!(
        "Notice: Task 1 finishes completely before Task 2 starts, even though they are `async` functions. This is because we `await` them sequentially."
    );

    // -------------------------------------------------------------------------
    // 3. Spawning Tasks: Running Futures Concurrently
    // -------------------------------------------------------------------------
    // To run multiple `Future`s truly concurrently (in parallel if multiple
    // CPU cores are available, or interleaved if on a single core), you need
    // to "spawn" them onto the async runtime. The `tokio::spawn` function
    // takes a `Future` and schedules it for execution. It returns a `JoinHandle`.

    println!("\n--- 3. Spawning Tasks: Running Futures Concurrently ---");

    async fn background_task(name: &str, delay_secs: u64) -> String {
        println!("[{}] Starting...", name);
        tokio::time::sleep(tokio::time::Duration::from_secs(delay_secs)).await;
        println!("[{}] Finished!", name);
        format!("Result from {}", name)
    }

    let handle1 = tokio::spawn(background_task("Task A", 3)); // Spawn a task
    let handle2 = tokio::spawn(background_task("Task B", 1)); // Spawn another task
    let handle3 = tokio::spawn(background_task("Task C", 2)); // Spawn a third task

    println!("Main function continues while tasks are running in background.");

    // `await`ing the `JoinHandle` blocks the current async function until
    // the spawned task completes.
    let result_a = handle1.await.expect("Task A failed");
    let result_b = handle2.await.expect("Task B failed");
    let result_c = handle3.await.expect("Task C failed");

    println!(
        "Collected results: {}, {}, {}",
        result_a, result_b, result_c
    );
    println!("All spawned tasks completed.");

    // Observe the output: "Task B Finished!" will likely appear before "Task A Finished!"
    // even though Task A was spawned first, because Task B has a shorter delay.
    // This highlights the non-blocking, concurrent nature.

    // -------------------------------------------------------------------------
    // 4. Asynchronous Functions Don't *Have* to Use `async fn`
    // -------------------------------------------------------------------------
    // The `async fn` syntax is syntactic sugar. Under the hood, an `async fn`
    // is just a regular function that returns an `impl Future`. You can manually
    // create a function that returns `impl Future` if you need more control,
    // though `async fn` is preferred for simplicity.

    println!("\n--- 4. Asynchronous Functions Don't *Have* to Use `async fn` ---");

    // Change function signature: This function returns an `impl Future` directly.
    fn manual_async_function(value: u32) -> impl Future<Output = String> {
        // Add function code into async block
        async move {
            // `async move` is often needed here to capture variables by value
            println!("[Manual Async] Starting with value: {}", value);
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            println!("[Manual Async] Finished with value: {}", value);
            format!("Result from manual async: {}", value)
        }
    }

    // The call-site remains the same scenario:
    let manual_future = manual_async_function(99);
    let manual_result = manual_future.await;
    println!("Manual async function result: {}", manual_result);

    // -------------------------------------------------------------------------
    // 5. Asynchronous Functions and Variable Lifetimes (`async move`)
    // -------------------------------------------------------------------------
    // When you create a `Future` (either via `async fn` or `async {}` block),
    // it captures the variables it needs from its environment. By default,
    // it captures them by reference. However, if the `Future` outlives the
    // scope of the variables it needs, this will cause a compile error.
    //
    // The `async move` block explicitly moves (takes ownership of) any variables
    // captured from the environment into the `Future`. This ensures the `Future`
    // owns all the data it needs to complete, regardless of when it's executed.
    // This also helps with the lifetime of variables returned by Future.

    println!("\n--- 5. Asynchronous Functions and Variable Lifetimes (`async move`) ---");

    // Asynchronous functions can move variables:
    async fn process_string_async_move() -> String {
        // Create local string variable
        let my_string = String::from("This string is owned by the async block.");
        println!("[Async Move] Processing string: '{}'", my_string);
        tokio::time::sleep(tokio::time::Duration::from_millis(700)).await;
        // Return the local variable inside an async move
        my_string // `my_string` is moved out of the async block
    }

    let moved_string_future = process_string_async_move();
    println!("`process_string_async_move` returned a Future.");
    let final_string = moved_string_future.await;
    println!("Received string from async move: '{}'", final_string);
    // The string was moved into the future, and then moved out when the future completed.

    // Example of a scenario where `move` is crucial:
    // If `my_string` was defined outside and captured by reference,
    // and the `Future` was spawned to run in the background, `my_string`
    // might go out of scope before the Future completes, leading to a dangling reference.
    // `async move` bundles variables inside async, ensuring the Future owns its context.

    /*
    // This would cause a compile error without `move` if spawned:
    let outer_string = String::from("I am outside.");
    let future_without_move = async {
        // `outer_string` is borrowed here
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        println!("Accessing outer string: {}", outer_string);
    };
    // tokio::spawn(future_without_move); // Error: `outer_string` does not live long enough
    // `outer_string` would be dropped at the end of `main`'s scope, but the future might still be running.
    */
    println!(
        "Note: An example demonstrating why `async move` is crucial for spawned tasks is commented out."
    );

    // -------------------------------------------------------------------------
    // 6. Asynchronous I/O Operations
    // -------------------------------------------------------------------------
    // The power of async programming comes from its use with I/O-bound operations.
    // Asynchronous runtimes provide their own versions of I/O primitives that
    // are non-blocking. For example, `tokio::fs` for file operations, `tokio::net`
    // for network operations, `tokio::io` for general I/O traits.

    println!("\n--- 6. Asynchronous I/O Operations ---");

    let file_path = "async_example.txt";
    let content = "Hello from async Rust!";

    async fn write_and_read_file(path: &str, data: &str) -> io::Result<String> {
        println!("Writing to file: {}", path);
        let mut file = fs::File::create(path).await?; // Create file asynchronously
        file.write_all(data.as_bytes()).await?; // Write asynchronously
        println!("Finished writing to file.");

        println!("Reading from file: {}", path);
        let mut file = fs::File::open(path).await?; // Open file asynchronously
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?; // Read asynchronously
        println!("Finished reading from file.");

        Ok(String::from_utf8_lossy(&buffer).into_owned())
    }

    // Spawn the file operation as a task
    let file_handle = tokio::spawn(write_and_read_file(file_path, content));

    // Do other work while file I/O is happening
    println!("Performing other tasks while file I/O is in progress...");
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    println!("Other tasks continue...");

    // Await the file operation result
    match file_handle.await {
        Ok(Ok(read_content)) => println!("Content read from file: '{}'", read_content),
        Ok(Err(e)) => eprintln!("File operation error: {}", e),
        Err(e) => eprintln!("Task join error: {}", e), // Error from `tokio::spawn` itself
    }

    // Clean up the created file (synchronously for simplicity here)
    if let Err(e) = std::fs::remove_file(file_path) {
        eprintln!("Failed to clean up file {}: {}", file_path, e);
    }

    // -------------------------------------------------------------------------
    // 7. Channels for Async Communication (Brief Mention)
    // -------------------------------------------------------------------------
    // When you have multiple async tasks, you often need them to communicate.
    // Asynchronous channels (e.g., `tokio::sync::mpsc` for multi-producer, single-consumer)
    // are used for safe, non-blocking communication between tasks.

    println!("\n--- 7. Channels for Async Communication ---");

    async fn producer(sender: mpsc::Sender<String>) {
        for i in 0..3 {
            let msg = format!("Message {}", i);
            println!("[Producer] Sending: {}", msg);
            sender.send(msg).await.expect("Failed to send message"); // Non-blocking send
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }

    async fn consumer(mut receiver: mpsc::Receiver<String>) {
        while let Some(msg) = receiver.recv().await {
            // Non-blocking receive
            println!("[Consumer] Received: {}", msg);
        }
        println!("[Consumer] Channel closed.");
    }

    let (tx, rx) = mpsc::channel(10); // Create an async channel with a buffer of 10
    tokio::spawn(producer(tx));
    tokio::spawn(consumer(rx))
        .await
        .expect("Consumer task failed"); // Await consumer to finish

    println!("\n--- End of Asynchronous Programming Examples ---");
    println!(
        "Asynchronous programming is a big topic with many nuances, but these fundamentals provide a strong starting point."
    );
}
