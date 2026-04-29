use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Deserialize)]
struct MoodQuery {
        mood: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
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

// Persistence for user-submitted moods. Stored as an array in `data/custom_moods.json`.
fn custom_moods_path() -> std::path::PathBuf {
        let mut p = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        p.push("data");
        p.push("custom_moods.json");
        p
}

fn load_custom_moods() -> Vec<MoodResponse> {
        let path = custom_moods_path();
        if !path.exists() {
                return vec![];
        }
        let s = std::fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&s).unwrap_or_else(|_| vec![])
}

fn save_custom_mood(m: &MoodResponse) -> std::io::Result<()> {
        let path = custom_moods_path();
        if let Some(dir) = path.parent() {
                std::fs::create_dir_all(dir)?;
        }
        let mut v = load_custom_moods();
        v.push(m.clone());
        let out = serde_json::to_string_pretty(&v)?;
        std::fs::write(path, out)?;
        Ok(())
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
                <svg style="position:fixed;inset:0;pointer-events:none;opacity:.06" preserveAspectRatio="xMidYMid slice" xmlns="http://www.w3.org/2000/svg">
                        <defs>
                                <linearGradient id="g1" x1="0%" x2="100%" y1="0%" y2="100%">
                                        <stop offset="0%" stop-color="#6ee7b7" />
                                        <stop offset="100%" stop-color="#60a5fa" />
                                </linearGradient>
                        </defs>
                        <rect width="100%" height="100%" fill="url(#g1)" transform="rotate(-12 0 0)" />
                </svg>
                <div class="app">
        <h1>Mood Mosaic</h1>
        <p class="lead">Pick a mood and get a personalized micro-experience.</p>
                <div class="grid">
                        <div class="mood" data-mood="happy">😊 Happy</div>
                        <div class="mood" data-mood="excited">🤩 Excited</div>
                        <div class="mood" data-mood="curious">🤔 Curious</div>
                        <div class="mood" data-mood="thoughtful">🧐 Thoughtful</div>
                        <div class="mood" data-mood="stressed">😣 Stressed</div>
                        <div class="mood" data-mood="grateful">🙏 Grateful</div>
                        <div class="mood" data-mood="calm">😌 Calm</div>
                </div>
                <div id="result"></div>

                <hr style="margin-top:2rem;opacity:.08" />
                <h3 style="text-align:center">Submit a custom mood</h3>
                <form id="customForm" style="max-width:640px;margin:0 auto;display:grid;gap:.6rem;margin-top:.6rem">
                        <input name="mood" placeholder="Mood key (e.g. 'serene')" required style="padding:.6rem;border-radius:8px;border:none;background:#071826;color:#fff" />
                        <input name="color" placeholder="Accent color (hex)" style="padding:.6rem;border-radius:8px;border:none;background:#071826;color:#fff" />
                        <input name="message" placeholder="Short message" required style="padding:.6rem;border-radius:8px;border:none;background:#071826;color:#fff" />
                        <input name="tip" placeholder="Quick tip (optional)" style="padding:.6rem;border-radius:8px;border:none;background:#071826;color:#fff" />
                        <div style="text-align:center"><button type="submit" style="padding:.6rem 1rem;border-radius:10px;border:none;background:#60a5fa;color:#071029;cursor:pointer">Save Custom Mood</button></div>
                </form>
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

                // Custom mood submission
                async function submitCustom(e){
                        e.preventDefault();
                        const form = document.getElementById('customForm');
                        const data = {
                                mood: form.mood.value,
                                message: form.message.value,
                                color: form.color.value || '#8b5cf6',
                                tip: form.tip.value || ''
                        };
                        const res = await fetch('/api/custom', {method:'POST',headers:{'content-type':'application/json'},body:JSON.stringify(data)});
                        if(res.ok){
                                alert('Saved custom mood!');
                                form.reset();
                        } else {
                                alert('Failed to save');
                        }
                }

                // Wire form
                document.addEventListener('DOMContentLoaded', ()=>{
                        const f = document.getElementById('customForm');
                        if(f) f.addEventListener('submit', submitCustom);
                });

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

                // Custom moods endpoints: POST to add, GET to list
                let api_custom_post = warp::path!("api" / "custom")
                        .and(warp::post())
                        .and(warp::body::json())
                        .map(|incoming: MoodResponse| {
                                match save_custom_mood(&incoming) {
                                        Ok(_) => warp::reply::with_status(warp::reply::json(&serde_json::json!({"ok":true})), warp::http::StatusCode::CREATED),
                                        Err(e) => warp::reply::with_status(warp::reply::json(&serde_json::json!({"ok":false, "error": format!("{}", e)})), warp::http::StatusCode::INTERNAL_SERVER_ERROR),
                                }
                        });

                let api_custom_get = warp::path!("api" / "custom")
                        .and(warp::get())
                        .map(|| {
                                let list = load_custom_moods();
                                warp::reply::json(&list)
                        });

        let routes = index.or(api_mood).or(api_surprise).or(api_custom_post).or(api_custom_get);

        let addr = ([127, 0, 0, 1], 3000);
        println!("Mood Mosaic running at http://127.0.0.1:{}", addr.1);

        warp::serve(routes).run(addr).await;
}

#[cfg(test)]
mod tests {
        use super::*;
        use serde_json::to_value;

        #[test]
        fn test_mood_data_known() {
                let m = mood_data_for("happy");
                assert_eq!(m.mood, "happy");
                assert!(m.message.contains("shining") || m.tip.len() > 0);

                let c = mood_data_for("curious");
                assert_eq!(c.mood, "curious");

                let s = mood_data_for("stressed");
                assert_eq!(s.mood, "stressed");
        }

        #[test]
        fn test_mood_data_default() {
                let d = mood_data_for("unknown-mood");
                assert_eq!(d.mood, "calm");
        }

        #[test]
        fn test_mood_response_serializes() {
                let resp = mood_data_for("happy");
                let v = to_value(&resp).expect("serialize");
                assert!(v.get("message").is_some());
                assert!(v.get("tip").is_some());
        }
}
