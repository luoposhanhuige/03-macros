// You're working with asynchronous Rust and implementing a macro my_ready! to help with polling futures.
// Future: Represents an asynchronous computation that can be polled to determine if it is complete.
// Pin: Ensures that the data behind a pointer cannot be moved, which is critical for futures that rely on stable memory addresses.
// Context and Poll: Used for manually polling futures. Context provides a Waker to wake the task, and Poll indicates whether the future is ready (Poll::Ready) or still pending (Poll::Pending).

use std::{
    future::Future,        // trait for asynchronous computations.
    pin::Pin,              // prevents data from being moved in memory (important for futures).
    task::{Context, Poll}, // used during manual polling of futures.
};

// 1, Entry Point
#[tokio::main] // starts an async runtime. This macro sets up the Tokio runtime for asynchronous execution.
async fn main() {
    // If you wanted to manually poll the MyFut future, you could call the poll_fut function, which internally uses the my_ready! macro.
    // For example:
    // let mut cx = Context::from_waker(futures::task::noop_waker_ref());
    // let ret = poll_fut(&mut cx);
    // match ret {
    //     Poll::Ready(value) => println!("Future is ready with value: {}", value),
    //     Poll::Pending => println!("Future is still pending"),
    // }
    // A Waker is an object that allows a future to signal the async runtime that it is ready to be polled again. It is part of the std::task module and is used in conjunction with the Context.
    // When a future cannot make progress (e.g., waiting for I/O), it returns Poll::Pending and stores the Waker provided by the Context.
    // Once the future becomes ready (e.g., data is available), it calls waker.wake() or waker.wake_by_ref() to notify the runtime.
    // A Context is a structure that provides access to the Waker for the current task. It is passed to the poll method of a future and is used to manage the task's lifecycle.
    // Provides a Waker: The Context gives the future access to the Waker for the current task.
    // Task-Specific: Each task has its own Context, ensuring that the Waker is tied to the correct task.
    // Without Waker, the runtime would need to poll all tasks continuously, wasting CPU cycles.
    // With Waker, the runtime only polls tasks that explicitly signal readiness, improving efficiency.

    // How Waker and Context Work Together
    // Polling a Future:

    // The runtime calls the poll method of a future, passing a Context that contains a Waker.
    // Future is Not Ready:

    // If the future cannot make progress, it stores the Waker from the Context and returns Poll::Pending.
    // Future Becomes Ready:

    // When the future becomes ready (e.g., I/O completes), it calls waker.wake() to notify the runtime.
    // Runtime Reschedules the Task:

    // The runtime reschedules the task for polling, and the future is polled again.

    // fut is a Nested Future
    // The fut variable is an instance of your custom future MyFut, which also implements the Future trait.
    // This makes fut a nested future inside the async fn main() future.
    let fut = MyFut::new(42); // returns a future. Create an instance of MyFut with the value 42.

    // Async Runtime (Tokio) Handles Polling Automatically:
    // In the main function, the fut.await expression is used, which delegates the polling of the MyFut future to the async runtime (Tokio).
    // The runtime automatically handles the polling process, so manual polling via poll_fut is unnecessary.

    println!("Final result: {}", fut.await); // fut.await runs your custom future (MyFut).
                                             // The .await keyword:
                                             // Suspends Execution: It pauses the current async function until the future (fut) is ready.
                                             // Polls the Future: It repeatedly polls the future to check if it is complete (Poll::Ready) or still pending (Poll::Pending).
                                             // Resumes Execution: Once the future is ready, .await retrieves the value (Poll::Ready(v)) and resumes execution of the async function.

    // When fut.await is called:

    // First Poll:

    // The poll method of MyFut is called.
    // Since polled is false, the future sets polled = true, wakes the task, and returns Poll::Pending.
    // The async runtime (Tokio) suspends the current task and schedules it to be polled again.
    // Second Poll:

    // The poll method is called again.
    // Now polled is true, so the future returns Poll::Ready(v) (where v = 42).
    // Retrieve the Value:

    // The .await expression retrieves the value 42 from Poll::Ready(42) and resumes execution of the async function.
}
// Execution Flow (in main)
// MyFut::new(42) returns a future.
// First poll:
// polled is false, so sets it true and returns Pending.
// Tokio wakes the task again.
// Second poll:
// polled is true, returns Ready(42).
// Prints: Final result: 42.

// 2, Optional Manual Polling
// This is how you'd poll MyFut manually.
// Wraps the polling call in your macro my_ready!.
// If poll returns Poll::Pending, the macro short-circuits and returns early.
// the poll_fut function is not explicitly called anywhere.
// It is defined as an optional helper function (marked with #[allow(unused)]), but it is not invoked in the main function or elsewhere in the code.

// The poll_fut function is designed for manual polling of the MyFut future.
// It could be called in a context where you want to manually poll the future instead of using .await.
#[allow(unused)]
fn poll_fut(cx: &mut Context<'_>) -> Poll<usize> {
    let mut fut = MyFut::new(42);
    let fut = Pin::new(&mut fut);
    my_ready!(fut.poll(cx))
}

// 3, Custom Future Implementation
// This struct will return its value only after being polled twice.
struct MyFut {
    polled: bool,
    v: usize,
}

// Constructor
impl MyFut {
    fn new(v: usize) -> Self {
        Self { polled: false, v }
    }
}

// 4, Future Implementation
// How it behaves:
// On first poll:
// Sets polled = true.
// Wakes the task to tell the executor: "poll me again soon."
// Returns Poll::Pending.
// On second poll:
// Now polled == true, so it returns Poll::Ready(v).
impl Future for MyFut {
    type Output = usize;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.polled {
            Poll::Ready(self.v)
        } else {
            self.polled = true;
            // wake up the waker
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// 5, Macro Definition
// This macro simplifies the polling of futures.
// It checks if the future is ready.
// If it is ready, it returns Poll::Ready with the value.
// If it is not ready, it returns Poll::Pending, allowing the caller to handle it appropriately.
// Usage: my_ready!(fut.poll(cx))
// This macro is useful for writing cleaner async code, especially when manually polling futures.
// This mimics the behavior of the try_ready! or ready! macro in older async versions—it's for writing manual poll implementations cleanly.
#[macro_export]
macro_rules! my_ready {
    ($expr:expr) => {
        match $expr {
            std::task::Poll::Ready(v) => std::task::Poll::Ready(v),
            std::task::Poll::Pending => return std::task::Poll::Pending,
        }
    };
}
