mod engine;
mod web;
use engine::{Assets,BuildConfig,Result};
use std::{env,path::PathBuf,process};

fn argument(args:&[String],option:&str)->Result<String>{
    args.windows(2).find(|a|a[0]==option).map(|a|a[1].clone())
      .ok_or_else(||format!("Missing {option}"))
}
fn args_path(a:&[String],arg:&str)->Result<PathBuf>{Ok(PathBuf::from(argument(a,arg)?))}
fn output(value:&serde_json::Value)->Result<()>{
    println!("{}",serde_json::to_string_pretty(value).map_err(|e|e.to_string())?);Ok(())
}
fn run()->Result<()>{
    let args:Vec<String>=env::args().skip(1).collect();
    let action=args.first().map(String::as_str).unwrap_or("help");
    match action{
        "about"=>output(&serde_json::json!({
            "app":"Nikon Z8 Native Rust Patch Builder",
            "version":"0.1.0","engine":"Rust","upstream_version":"v0.5.1",
            "host_support":"Linux WSL/Windows",
            "firmware_input":"Original Nikon Z8 C3.11 only",
            "status":"EXPERIMENTAL_OFFLINE_ONLY","flash_approved":false
        })),
        "check"=>output(&engine::check_input(&args_path(&args,"--input")?)?),
        "inspect"=>output(&Assets::open(
            &args_path(&args,"--recipes")?,&args_path(&args,"--toolkit")?)?.summary()),
        "build"=>{
            let selected=argument(&args,"--features")?;
            let config=BuildConfig{
                stock:args_path(&args,"--input")?,output_dir:args_path(&args,"--output")?,
                version:argument(&args,"--version")?,tag:argument(&args,"--tag")?,
                features:selected.split(',').map(str::to_owned).collect(),
                recipes:args_path(&args,"--recipes")?,toolkit:args_path(&args,"--toolkit")?
            };
            output(&engine::build(&config)?)
        },
        "verify"=>{let assets=Assets::open(&args_path(&args,"--recipes")?,&args_path(&args,"--toolkit")?)?;
            output(&engine::verify(
            &args_path(&args,"--input")?,
            &args_path(&args,"--candidate")?,
            &args_path(&args,"--manifest")?,&assets)?)},
        "help"=>{println!("Nikon Z8 Rust Builder (offline, no flashing)\n\
            check --input STOCK.bin\n\
            inspect --toolkit V051_RELEASE --recipes LOCAL_NATIVE_RECIPES\n\
            build --input STOCK.bin --toolkit V051_RELEASE --recipes LOCAL_NATIVE_RECIPES \\\n                  --features video-recording,log-colour,still-photo-tools,raw-precapture \\\n                  --version 04.08 --tag p0 --output NEW_DIR\n\
            verify --input STOCK.bin --candidate OUTPUT.bin --manifest manifest.json --toolkit V051_RELEASE --recipes LOCAL_NATIVE_RECIPES
\
            gui --toolkit V051_RELEASE --recipes LOCAL_NATIVE_RECIPES --port 8768");Ok(())},
        _=>Err(format!("Unknown action {action}"))
    }
}
#[tokio::main]
async fn main(){
    let args:Vec<String>=env::args().skip(1).collect();
    let result=if args.first().map(String::as_str)==Some("gui"){
        let result=(||->Result<(PathBuf,PathBuf,u16)>{
            let port=argument(&args,"--port").unwrap_or_else(|_|"8768".to_owned())
                .parse::<u16>().map_err(|e|e.to_string())?;
            if port==0{return Err("Invalid TCP port".into());}
            Ok((args_path(&args,"--recipes")?,args_path(&args,"--toolkit")?,port))
        })();
        match result{Ok((root,toolkit,port))=>web::serve(root,toolkit,port).await,Err(e)=>Err(e)}
    }else{run()};
    if let Err(e)=result{eprintln!("RUST_BUILDER_FAIL_CLOSED {e}");process::exit(2);}
}
