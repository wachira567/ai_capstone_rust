use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Deserialize)]
struct MoodQuery {
        mood: Option<String>,
}

#[derive(Serialize)]
struct MoodResponse {
        mood: String,
        message: String,
        color: String,
        tip: String,
}

fn mood_data_for(mood: &str) -> MoodResponse {
        match mood {
                "happy" => MoodResponse {
                        mood: "happy".into(),
                        message: "You're shining today! Keep that glow.".into(),
                        color: "#f59e0b".into(),
                        tip: "Share your joy: tell someone you appreciate them.".into(),
                },
                "curious" => MoodResponse {
                        mood: "curious".into(),
                        message: "Curiosity is the engine of learning.".into(),
                        color: "#60a5fa".into(),
                        tip: "Try a 5-minute deep dive into a random topic.".into(),
                },
                "stressed" => MoodResponse {
                        mood: "stressed".into(),
                        message: "Breathe — small steps move mountains.".into(),
                        color: "#ef4444".into(),
                        tip: "Try the 4-4-4 breathing: inhale 4s, hold 4s, exhale 4s.".into(),
                },
                _ => MoodResponse {
                        mood: "calm".into(),
                        message: "A quiet moment can reset your day.".into(),
                        color: "#34d399".into(),
                        tip: "Stand, stretch, and look away from screens for a minute.".into(),
                },
        }
}

#[tokio::main]
async fn main() {
        // Root HTML — small interactive frontend
        let index = warp::path::end().map(|| {
                let html = r#"<!doctype html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width,initial-scale=1">
    <title>Mood Mosaic</title>
    <style>
        :root{--bg:#071029;--card:#071a2b;--muted:#c7d2fe}
        body{margin:0;min-height:100vh;background:linear-gradient(135deg,#071029 0%,#0b2540 100%);font-family:Inter,system-ui,Arial;color:var(--muted);display:flex;align-items:center;justify-content:center}
        .app{max-width:900px;padding:2rem}
        h1{color:#fff;text-align:center;font-size:2.2rem;margin-bottom:0.3rem}
        p.lead{text-align:center;color:#9fb0d6}
        .grid{display:flex;gap:1rem;justify-content:center;margin-top:1.5rem}
        .mood{background:var(--card);padding:1rem 1.2rem;border-radius:12px;cursor:pointer;min-width:120px;text-align:center;box-shadow:0 6px 18px rgba(2,6,23,0.6);transition:transform .18s ease}
        .mood:hover{transform:translateY(-6px)}
        #result{margin-top:1.4rem;padding:1rem;border-radius:10px;background:rgba(255,255,255,0.03);text-align:center}
        button.share{margin-top:1rem;padding:.5rem 1rem;border-radius:8px;border:none;background:#fff;color:#071029;cursor:pointer}
    </style>
    <script src="https://cdn.jsdelivr.net/npm/canvas-confetti@1.6.0/dist/confetti.browser.min.js"></script>
</head>
<body>
    <div class="app">
        <h1>Mood Mosaic</h1>
        <p class="lead">Pick a mood and get a personalized micro-experience.</p>
        <div class="grid">
            <div class="mood" data-mood="happy">😊 Happy</div>
            <div class="mood" data-mood="curious">🤔 Curious</div>
            <div class="mood" data-mood="stressed">😣 Stressed</div>
            <div class="mood" data-mood="calm">😌 Calm</div>
        </div>
        <div id="result"></div>
    </div>

    <script>
        async function pick(mood){
            const res = await fetch('/api/mood?mood='+encodeURIComponent(mood));
            const j = await res.json();
            const r = document.getElementById('result');
            r.innerHTML = `<h2 style="color:${j.color};margin:0 0 .6rem 0">${j.message}</h2><div style="color:#9fb0d6">Tip: ${j.tip}</div><button class='share' onclick="navigator.clipboard.writeText('I feel '+j.mood+' — '+j.message)">Copy Mood</button>`;
            // visual flourish
            if(mood==='happy' || mood==='curious'){
                confetti({particleCount:80,spread:70});
            } else if(mood==='stressed'){
                confetti({particleCount:40,spread:30,colors:['#ef4444','#f97316']});
            } else {
                confetti({particleCount:30,spread:40,shapes:['circle']});
            }
        }

        document.querySelectorAll('.mood').forEach(el=>el.addEventListener('click',()=>pick(el.dataset.mood)));

        // easter egg: press 'm' to get a surprise
        window.addEventListener('keydown', async (e)=>{
            if(e.key==='m'){
                const res = await fetch('/api/surprise');
                const j = await res.json();
                alert(j.title+'\n\n'+j.body);
            }
        });
    </script>
</body>
</html>"#;

                warp::reply::html(html)
        });

        // API: /api/mood?mood=happy
        let api_mood = warp::path!("api" / "mood")
                .and(warp::query::<MoodQuery>())
                .map(|q: MoodQuery| {
                        let selected = q.mood.unwrap_or_else(|| "calm".to_string());
                        let resp = mood_data_for(&selected);
                        warp::reply::json(&resp)
                });

        // Surprise endpoint
        let api_surprise = warp::path!("api" / "surprise").map(|| {
                let choices = vec![
                        ("Secret Mosaic", "You found the secret mood mosaic — keep exploring!"),
                        ("Tiny Challenge", "Do 10 jumping jacks and celebrate small wins."),
                        ("Curiosity Boost", "Read the first paragraph of a random Wikipedia article."),
                ];
                let mut rng = rand::thread_rng();
                let (title, body) = choices.choose(&mut rng).unwrap();
                warp::reply::json(&serde_json::json!({"title": title, "body": body}))
        });

        let routes = index.or(api_mood).or(api_surprise);

        let addr = ([127, 0, 0, 1], 3000);
        println!("Mood Mosaic running at http://127.0.0.1:{}", addr.1);

        warp::serve(routes).run(addr).await;
}
