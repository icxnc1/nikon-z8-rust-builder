//! Fully Rust-powered local GUI. No Python process, no camera or card operations.
//! HTTP binds only to 127.0.0.1; POST requires same-origin custom request header.
use axum::{extract::State,http::{HeaderMap,StatusCode},response::Html,routing::{get,post},Json,Router};
use serde::Deserialize;
use serde_json::{Value,json};
use std::{net::SocketAddr,path::PathBuf,sync::Arc};
use crate::engine::{self,Assets,BuildConfig};

#[derive(Clone)]
struct App {recipes:PathBuf,toolkit:PathBuf}
#[derive(Deserialize)]
struct Input {path:String}
#[derive(Deserialize)]
struct Verify {stock:String,candidate:String,manifest:String}
#[derive(Deserialize)]
struct Build {input:String,output:String,version:String,tag:String,features:Vec<String>}
fn post_allowed(headers:&HeaderMap)->bool{
    headers.get("x-z8-local").and_then(|v|v.to_str().ok())==Some("yes")
}
fn checked_path(s:&str)->Result<PathBuf,String>{
    if s.is_empty()||s.contains('\0')||s.len()>4096 {return Err("Invalid path".into());}
    Ok(PathBuf::from(s))
}
async fn index()->Html<&'static str>{Html(HTML)}
async fn localization_js()->impl axum::response::IntoResponse{
    ([(axum::http::header::CONTENT_TYPE,"text/javascript; charset=utf-8")],
      include_str!("i18n.js"))
}
async fn status(State(app):State<Arc<App>>)->Json<Value>{
    Json(match Assets::open(&app.recipes,&app.toolkit){
        Ok(a)=>a.summary(),Err(e)=>json!({"status":"BLOCKED","error":e})
    })
}
fn output(res:Result<Value,String>)->(StatusCode,Json<Value>){
    match res{Ok(x)=>(StatusCode::OK,Json(x)),Err(e)=>(StatusCode::BAD_REQUEST,
        Json(json!({"status":"BLOCKED_FAIL_CLOSED","error":e,"flash_approved":false})))}
}
async fn check(HeaderGuard(h):HeaderGuard,Json(req):Json<Input>)->(StatusCode,Json<Value>){
    if !post_allowed(&h){return output(Err("Missing local request header".into()));}
    let future=tokio::task::spawn_blocking(move||engine::check_input(&checked_path(&req.path)?));
    output(future.await.unwrap_or_else(|_|Err("Check worker failed".into())))
}
async fn verify(State(app):State<Arc<App>>,HeaderGuard(h):HeaderGuard,Json(req):Json<Verify>)->(StatusCode,Json<Value>){
    if !post_allowed(&h){return output(Err("Missing local request header".into()));}
    let future=tokio::task::spawn_blocking(move||{
        let assets=Assets::open(&app.recipes,&app.toolkit)?;
        engine::verify(&checked_path(&req.stock)?,&checked_path(&req.candidate)?,
            &checked_path(&req.manifest)?,&assets)
    });
    output(future.await.unwrap_or_else(|_|Err("Verification worker failed".into())))
}
async fn build(State(app):State<Arc<App>>,HeaderGuard(h):HeaderGuard,
              Json(req):Json<Build>)->(StatusCode,Json<Value>){
    if !post_allowed(&h){return output(Err("Missing local request header".into()));}
    let cfg=BuildConfig{
        stock:match checked_path(&req.input){Ok(p)=>p,Err(e)=>return output(Err(e))},
        output_dir:match checked_path(&req.output){Ok(p)=>p,Err(e)=>return output(Err(e))},
        version:req.version,tag:req.tag,features:req.features,
        recipes:app.recipes.clone(),toolkit:app.toolkit.clone()
    };
    let future=tokio::task::spawn_blocking(move||engine::build(&cfg));
    output(future.await.unwrap_or_else(|_|Err("Build worker failed".into())))
}
use axum::{extract::FromRequestParts,http::request::Parts};
struct HeaderGuard(HeaderMap);
impl<S:Send+Sync> FromRequestParts<S> for HeaderGuard{
    type Rejection=StatusCode;
    async fn from_request_parts(parts:&mut Parts,_:&S)->Result<Self,Self::Rejection>{
        Ok(HeaderGuard(parts.headers.clone()))
    }
}
pub async fn serve(root:PathBuf,toolkit:PathBuf,port:u16)->Result<(),String>{
    let app=Router::new().route("/",get(index))
        .route("/i18n.js",get(localization_js))
        .route("/api/status",get(status))
        .route("/api/check",post(check))
        .route("/api/build",post(build))
        .route("/api/verify",post(verify))
        .with_state(Arc::new(App{recipes:root,toolkit}));
    let listener=tokio::net::TcpListener::bind(SocketAddr::from(([127,0,0,1],port)))
        .await.map_err(|e|e.to_string())?;
    println!("NIKON_Z8_RUST_GUI=http://127.0.0.1:{port}/");
    axum::serve(listener,app).await.map_err(|e|e.to_string())
}
const HTML:&str=r###"<!doctype html><html lang="ro"><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>Nikon Z8 — Rust Native Builder</title>
<style>
:root{font:15px system-ui,sans-serif;color-scheme:dark}
body{max-width:900px;margin:3rem auto;padding:0 1rem;color:#e6edf5;background:#12171f}
h1{font-size:1.9rem;letter-spacing:-.04em}h2{font-size:1.1rem}
section{border:1px solid #34404c;border-radius:14px;padding:1.1rem 1.4rem;margin:1.2rem 0;background:#1b2531}
label{display:block;margin:.7rem 0}input[type=text],input[type=number]{display:block;width:100%;box-sizing:border-box;padding:.7rem;border-radius:7px;border:1px solid #4f606d;background:#0c141e;color:white;font:inherit}
button{border:0;border-radius:8px;background:#518ae4;color:white;font:600 1rem system-ui;padding:.75rem 1.3rem;cursor:pointer}
button:disabled{opacity:.5;cursor:progress}fieldset{border:0;padding:0;display:grid;grid-template-columns:1fr 1fr}
fieldset label{display:flex;gap:.6rem;align-items:center}
small{color:#acbace}pre{white-space:pre-wrap;word-break:break-word;max-height:430px;overflow:auto;font:12px ui-monospace,monospace;padding:1rem;background:#090e15;border-radius:9px}
.warning{color:#ffbb69}#log{min-height:90px}footer{margin:1rem 0 3rem}
.language-control{display:flex;justify-content:flex-end;align-items:center;gap:.7rem;margin:0 0 1rem}
.language-control label{margin:0}.language-control select{padding:.5rem .7rem;background:#0c141e;color:white;border:1px solid #60758c;border-radius:8px;font:inherit;max-width:100%}
</style></head><body><h1>Nikon Z8 · Rust Native Builder</h1>
<p>v0.5.1 · C3.11 → Experimental · Windows / Ubuntu WSL</p>
<p class="warning">Aplicație offline: nu scrie pe carduri, nu accesează camera. Verificarea fișierului nu certifică siguranța instalării.</p>
<section><h2>1. Verificare resurse</h2><button onclick="status()">Verifică instrumentele</button><pre id="state">Se încarcă...</pre></section>
<section><h2>2. Firmware original Nikon</h2><label>Cale fișier C3.11 (SHA verificat)
<input id="stock" type="text" placeholder="/mnt/d/patch z8/Z_8_0311.bin"></label>
<button onclick="check()">Verifică originalul</button></section>
<section><h2>3. Funcții</h2>
<fieldset>
<label><input type="checkbox" name="f" value="video-recording" checked> Video recording / N-RAW 3:2</label>
<label><input type="checkbox" name="f" value="log-colour" checked> Log colour / RED / Sony</label>
<label><input type="checkbox" name="f" value="still-photo-tools" checked> SDR HEIF / Pixel Shift / Crop</label>
<label><input type="checkbox" name="f" value="raw-precapture" checked> RAW pre-capture</label>
</fieldset>
<label>Identitate versiune<input type="text" id="version" value="04.08"></label>
<label>Tag<input type="text" id="tag" value="p0"></label>
<label>Folder nou, inexistent<input type="text" id="dest" placeholder="/mnt/d/patch z8/build/nou-04.08"></label>
<button id="build" onclick="build()">Generează și verifică în Rust</button>
<p><small>Operația include integritate Nikon, 20 unități, 15 rețete, SHA, CRC, reconstruire și citire de control. Nu suprascrie un output existent.</small></p></section>
<section><h2>4. Verificare fișier existent</h2>
<label>Fișier firmware experimental<input id="candidate" type="text"></label>
<label>Manifest JSON<input id="manifest" type="text"></label>
<button onclick="verify()">Verifică independent pachetul</button></section>
<section><h2>Rezultate</h2><pre id="log"></pre></section>
<footer><small>GPL-3.0-only · Upstream XinmiaoZhao/NikonFirmwarePatch v0.5.1 · Nu este produs oficial Nikon.</small></footer>
<script src="/i18n.js"></script>
<script>
const el=x=>document.getElementById(x);
async function api(path,data){
const options=data===undefined?{}:{method:"POST",headers:{"content-type":"application/json","x-z8-local":"yes"},body:JSON.stringify(data)};
const b=el("build");b.disabled=true;el("log").textContent=window.z8t("busy");
try{const r=await fetch(path,options);const j=await r.json();el("log").textContent=JSON.stringify(j,null,2);return j}
catch(e){el("log").textContent=String(e)}
finally{b.disabled=false}
}
async function status(){let j=await api("/api/status");el("state").dataset.loading="0";el("state").textContent=JSON.stringify(j,null,2)}
function check(){api("/api/check",{path:el("stock").value})}
function build(){api("/api/build",{input:el("stock").value,output:el("dest").value,version:el("version").value,tag:el("tag").value,features:[...document.querySelectorAll('input[name=f]:checked')].map(x=>x.value)})}
function verify(){api("/api/verify",{stock:el("stock").value,candidate:el("candidate").value,manifest:el("manifest").value})}
status()
</script></body></html>"###;
