use warp::Filter;

const DOCKER_BANNER: &str = r#"
##         .
## ## ##        ==
## ## ## ## ##    ===
/""""""""""""""""\\___/ ===
{{                       /  ===-
\______ O           __/
 \    \         __/
  \____\_______/

Hello from Docker!
"#;

#[tokio::main]
async fn main() {
    // Define the warp filter for the root path ("/")
    let hello = warp::path::end().map(|| warp::reply::html(DOCKER_BANNER));

    // Serve the filter at address 0.0.0.0:3030
    warp::serve(hello).run(([0, 0, 0, 0], 3030)).await;
}

