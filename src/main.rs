use serde::Deserialize;
use warp::Filter;

#[derive(Deserialize)]
struct GreetQuery {
    name: Option<String>,
}

#[tokio::main]
async fn main() {
    let index = warp::path::end()
        .and(warp::query::<GreetQuery>())
        .map(|query: GreetQuery| {
            let greeting = match query.name {
                Some(name) => format!("Hello, {}", name),
                None => "Welcome".to_string(),
            };

            let html = format!(r#"
<!DOCTYPE html>
<html>
<head>
  <meta charset=\"utf-8\"> 
  <title>AI Capstone Rust</title>
  <style>body{{font-family:sans-serif; background:#0b1220; color:#e6eef8; text-align:center; padding:3rem}} button{{padding:0.6rem 1rem;border-radius:6px;border:none;cursor:pointer}}</style>
</head>
<body>
  <h1>{}</h1>
  <p>This is a minimal Warp-based Rust example for the AI capstone.</p>
  <button onclick=\"fetch('/hello').then(r=>r.json()).then(j=>alert(j.message))\">Say Hello</button>
</body>
</html>
"#, greeting);

            warp::reply::html(html)
        });

    let hello = warp::path("hello")
        .and(warp::path::end())
        .map(|| warp::reply::json(&serde_json::json!({"message": "Hello from ai_capstone_rust"})));

    let routes = index.or(hello);

    let addr = ([127, 0, 0, 1], 3000);
    println!("Server running at http://{}:{}", addr.0[0], addr.1);

    warp::serve(routes).run(addr).await;
}
