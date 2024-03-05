# KIOTO 🎌

Experimental asynchronous runtime inspired by `tokio-rs`, built for learning purpose

## Research

### Task

In asynchronous runtime, the first citizen unit is so-called `Task` which can be a stackfule coroutine which contains the `Future`. A task is runnable when it can make progress, and is no longer runnable (or idle) when it is blocked on an external resource.

Tasks are independent in that any number of runnable tasks can execute concurrently.

### Scheduler

Scheduler is responsible for scheduling the order of execution of each `Task`. The scheduler architecture consists a `queue`.

> We are talking about the user-level scheduler instead of the OS kernel-level schedule. Reference: [Round Robin Scheduling Algorithm](https://en.wikipedia.org/wiki/Round-robin_scheduling)

#### One queue, many processors

> MPSC: Multiple producers, Single consumer. In this case, the producer will be the task sender in each processor and the consumer is the task receiver in the queue.

One single global queue is initialized in the `main thread` while each processor spawns its own `thread` to run the processor. `Task` are processed concurrently by each `processor` and push to the tail of the global queue.

- Design is simple: The implementation is relatively simple. An off-the-shelf queue can be paired with the processor loop sketched above.
- When tasks execute for a long period of time, queue contention is reduced. However, Rust's asynchronous tasks are expected to take very little time executing when popped from the run queue. In this scenario, the overhead from contending on the queue becomes significant.

#### Many processors, each with their own run queue

Use multiple single-threaded schedulers. Each processor gets its own run queue and tasks are pinned to a specific processor. This avoids the problem of synchronization entirely. As Rust's task model requires the ability to queue a task from any thread, there still needs to be a thread-safe way to inject tasks into the scheduler.

Unless the workload is entirely uniform, some processors will become `idle` while other processors are `under load`, resulting in `resource underutilization`.

#### Work stealing scheduler

The work-stealing scheduler builds upon the sharded scheduler model and addresses the underutilization problem. Each processor maintains its own run queue. Tasks that become runnable are pushed onto the current processor's run queue and processors drain their local run queue. However, when a processor becomes idle, it checks sibling processor run queues and attempts to steal from them. A processor will go to sleep only once it fails to find work from sibling run queues.

- Work-stealing is the algorithm of choice for general purpose schedulers.

## Terminology

@/**Handle**: In computer programming, a handle is an abstract reference to a resource that is used when application software references blocks of memory or objects that are managed by another system like a database or an operating system. Example of common handles are: `network socket`, `file descriptor`, `database connections`, `process identifiers (pIDs)`. We can call it is like a pointer to the entity.

@/**JoinHandle**: Returns on new thread spawned, `JoinHandle::join` blocks until the corresponding thread is done executing (task is finished ✅). It asks the OS to block the main thread (the one calling `join()`) until the joined thread is done, and collect its status.

Before calling join(), you only know that you have handed the thread to the OS. You don't know later in the code if it has already been started, is running, has finished, was killed by the OS, or has panic()'ed, etc.

## References

- [`Rust` | Advanced Concepts | Actor Model by Actix](https://actix.rs/docs/actix/actor/): Actor basically has their own execution context, communicates with each other through messaging channel
- [`Rust` | Rust Forum - Pin use in `Future::poll()`](https://users.rust-lang.org/t/pin-use-in-futures-poll/80264/7): Discussion thread about `Future::pollPint(<&mut self>, _)` on Rust Forum, the discussion explains the `Pin` trait in a very deep level of low-level knowledge.
- [`Rust` | "Pin, Unpin and why Rust needs them" by Cloudflare](https://blog.cloudflare.com/pin-and-unpin-in-rust/)
- [`Rust` | "Understanding pinning in Rust futures" on Hackernoon](https://hackernoon.com/pin-safety-understanding-pinning-in-rust-futures): This resource is very easy to understand, it gives a clear cut about `Future` and `Pinning`

- [`Rust` | `std::pin::Pin`: Pin Projection](https://doc.rust-lang.org/std/pin/index.html#projections-and-structural-pinning): Learn more about [pin_project](https://docs.rs/pin-project/latest/pin_project/) crate for safe pin projection
- [`Rust` | "How Tokio schedule tasks?" on Rust Magazine](https://rustmagazine.org/issue-4/how-tokio-schedule-tasks/)
- [`Rust` | Advanced Concepts | Scheduling Internals](https://tontinton.com/posts/scheduling-internals/)
- [`Rust` | About Tokio scheduler internal](https://tokio.rs/blog/2019-10-scheduler): Understanding how Tokio scheduler works under the hood
