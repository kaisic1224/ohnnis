use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:1224").await.unwrap();

    loop {
        let (socket ) = listener.accept();
    }
}


// HOW TO USE SHARED STATE CONCURRENCY
//
// CLONE everything for synchronous operations
// When you are working in Async, use Arc<Mutex<T>>
// - then clone the thing that Arc points to whenever you need to access the thing inside Arc
// - Arcs = shared pointers, a way to keep track of how many objects are cloned and preserve its
//   lifetime across multiple instances
// - Mutexes = a state storage mechanism - store a piece of mutable data within a mutex, it will be
//   accessible iff it is not currently under read/write operations (from other threads) aka locked
//
//   CLOSURES AND move keyword - '||' defines a closure, any variables passed into the || will be
//   passed into the closure function. If 'move' preceeds a closure declaration, all variables used
//   in the closure will have their ownership transferred into the closure and be freed after
//   closure's lifetime is finsihed (wallahi).
//
//   CONSIDER ASKING THESE QUESTIONS:   
//   Do I need to pass around a read only copy? Use a ref. Example: Sharing a config struct.
//
//   Do I need to do the above but across multiple threads? Use an Arc.
//
//   Do I have single threaded shared data that needs to be edited? Use mut refs.
//
//   Do I need to keep something unedited in one path, and edited in another? Use a clone. Example: You have a vec of planet objects created at runtime by sourcing a DB, and a function that takes a vec of planets and uses them as state and then prints a cute interactive galaxy. You want the user to be able to reset to the default state while they play with their galaxy, so you hold the original. It's cheaper than requerying the DB.
//
//   Do I have to do the above across threads? Use an Arc<Mutex>. Example: You have the application running on Tokio. Or maybe more simply, you need to share a logger instance.
