//! Cross-platform, fully native Rust builder for pinned Nikon Z8 v0.5.1.
use serde::{Deserialize,Serialize};
use serde_json::{Value,json};
use sha1::Sha1;
use sha2::{Digest,Sha256};
use std::{fs::{self,OpenOptions},io::Write,path::{Path,PathBuf}};

pub const PACKAGE:usize=99_051_117;
const BODY:usize=PACKAGE-32;
const EG:usize=581_844;
const EG_LEN:usize=81_807_711;
const ZERO:usize=0xB10000;
const EMBED:usize=0x3FF1F84;
const K8:[u8;8]=[0x6c,0x12,0x03,0x7e,0x6f,0xce,0xc1,0x9a];
const OUTER:[u8;12]=[0x58,0x66,0xcd,0x12,0x42,0x69,0xc2,0x7e,0x62,0x1d,0xa2,0x26];
pub const INPUT_SHA:&str="d67413c2ede6b9ce8887175f20700bd9fbad2a973a4820968c45503d15d80d96";
const INPUT_EG_SHA:&str="f85e0138e2f5e660ece8560de62e0d12cb795e0e14983e50f20b61dce906c46c";
pub const ALL_SHA:&str="73a151f7ce1f033a5eaadb44cfcf5e37c74bc250addc7841061a3a606fc4bab5";
const SOURCE_COMMIT:&str="25fefa1e17f321ee285bea77b3c54427f9912e4b";
pub const RECIPE_SET_SHA:&str="ac5536b557dec974c742ba70e3ef50de5892a7a2ff66296bc2bea3c4b7b6dbed";
const RELEASE_MANIFEST_SHA:&str="46a3fd1e87fecf1758856cb368f40112d439f29ba8c37ebc973bbcaa4be33a4c";
pub const FEATURES:[&str;4]=["video-recording","log-colour","still-photo-tools","raw-precapture"];
const MODS:[(usize,usize);5]=[(208,524290),(524498,57346),(EG,EG_LEN),(82389555,838664),(83228219,15822850)];
pub type Result<T>=std::result::Result<T,String>;
fn check(ok:bool,msg:&str)->Result<()> {if ok {Ok(())}else{Err(msg.to_owned())}}
pub fn sha(bytes:&[u8])->String{format!("{:x}",Sha256::digest(bytes))}
const fn make_crc_table()->[u16;256]{
    let mut table=[0u16;256];let mut i=0usize;
    while i<256 {
        let mut crc=(i as u16)<<8;let mut bit=0;
        while bit<8 {
            crc=if crc&0x8000!=0{(crc<<1)^0x1021}else{crc<<1};
            bit+=1;
        }
        table[i]=crc;i+=1;
    }
    table
}
const CRC16_TABLE:[u16;256]=make_crc_table();
fn crc16(buf:&[u8])->u16 {
    let mut s=0u16;
    for &x in buf {s=(s<<8)^CRC16_TABLE[usize::from(((s>>8)as u8)^x)];}
    s
}
#[derive(Clone)]
struct Tables{a:[u8;256],b:[u8;256],c:[u8;256]}
fn recover(encrypted:&[u8])->Result<Tables>{
    check(encrypted.len()==BODY,"Nikon package length mismatch")?;
    let z=encrypted[ZERO];let mut t=Tables{a:[0;256],b:[0;256],c:[0;256]};
    for i in 0..256{t.a[i]=encrypted[ZERO+i]^z;t.b[i]=encrypted[ZERO+i*256]^z;}
    for i in 0..256{let p=EMBED+i;t.c[i]=encrypted[p]^t.a[p&255]^t.b[(p>>8)&255];}
    let shift=t.c[(ZERO>>16)&255]^z;for x in &mut t.c{*x^=shift;}Ok(t)
}
fn crypt(input:&[u8],t:&Tables)->Vec<u8>{
    let mut out=input.to_vec();
    for (i,v) in out.iter_mut().enumerate().skip(32){*v^=t.a[i&255]^t.b[(i>>8)&255]^t.c[(i>>16)&255];}
    out
}
fn u32be(b:&[u8],p:usize)->usize{u32::from_be_bytes(b[p..p+4].try_into().unwrap()) as usize}
fn auth(body:&[u8])->[u8;20]{let mut h=Sha1::new();h.update(body);h.update(K8);h.finalize().into()}
fn inspect(raw:&[u8],pin:Option<&str>)->Result<(Vec<u8>,Tables)>{
    check(raw.len()==PACKAGE,"Firmware must be exactly 99,051,117 bytes")?;
    if let Some(d)=pin{check(sha(raw)==d,"Firmware SHA-256 not original C3.11")?;}
    check(raw[20..32]==OUTER,"Unsupported Nikon envelope")?;
    let tables=recover(&raw[32..])?;
    let body=crypt(&raw[32..],&tables);
    check(body[..32]==[b' ';32],"Wrong inner body header")?;
    check(body[32..48]==[0,0,0,5,0,0,0,208,0,0,0,0,0,0,0,0],"Wrong directory")?;
    check(body[body.len()-14..].iter().all(|x|*x==0),"Trailer padding differs")?;
    check(crc16(&body[..body.len()-14])==0,"Body CRC16 invalid")?;
    check(auth(&body)==raw[..20],"Nikon SHA1 wrapper invalid")?;
    for (i,(p,n)) in MODS.iter().enumerate(){
        let off=48+32*i;
        check(u32be(&body,off+16)==*p&&u32be(&body,off+20)==*n,"Module extent invalid")?;
        check(body[off+24..off+32]==[0;8],"Nonzero module reserved bytes")?;
        check(crc16(&body[*p..*p+*n])==0,"Module CRC16 invalid")?;
    }
    Ok((body,tables))
}
#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct Window{pub offset:usize,pub bytes:usize,pub blob_at:usize,pub old_sha256:String,pub new_sha256:String}
#[derive(Debug,Deserialize)]
struct Recipe{
 schema:String,mask:u8,features:Vec<String>,source_commit:String,
 original_fw_sha256:String,original_eg_sha256:String,reviewed_profile:String,
 windows:Vec<Window>,patch_bytes:usize,patch_sha256:String,count:usize
}
#[derive(Debug,Deserialize)]
struct Entry{
 mask:u8,features:Vec<String>,patch_file:String,descriptor_file:String,
 patch_sha256:String,descriptor_sha256:String,edit_count:usize,patch_bytes:usize
}
#[derive(Debug,Deserialize)]
struct Registry{
 schema:String,source_commit:String,stock_sha256:String,stock_eg_sha256:String,
 mask_count:usize,compiled_units:usize,combinations:Vec<Entry>
}
pub struct Assets{pub root:PathBuf,registry:Registry}
pub fn feature_mask(names:&[String])->Result<u8>{
    check(!names.is_empty(),"No selected feature group")?;
    let mut mask=0u8;
    for feature in names{
        let i=FEATURES.iter().position(|x|*x==feature).ok_or_else(||format!("Unknown feature group {feature}"))?;
        let bit=1u8<<i;check(mask&bit==0,"Repeated feature group")?;mask|=bit;
    }Ok(mask)
}
pub fn mask_features(mask:u8)->Vec<String>{
    FEATURES.iter().enumerate().filter_map(|(i,f)|if mask&(1u8<<i)!=0{Some((*f).to_owned())}else{None}).collect()
}
fn jsonread(path:&Path)->Result<Value>{
    serde_json::from_slice(&fs::read(path).map_err(|e|format!("{}: {e}",path.display()))?)
        .map_err(|e|format!("Malformed {}: {e}",path.display()))
}
impl Assets{
 pub fn open(root:&Path,toolkit:&Path)->Result<Self>{
    let bytes=fs::read(root.join("recipe-set.json")).map_err(|e|format!("Recipe set: {e}"))?;
    check(sha(&bytes)==RECIPE_SET_SHA,"Recipe-set SHA pin mismatch")?;
    let registry:Registry=serde_json::from_slice(&bytes).map_err(|e|e.to_string())?;
    check(registry.schema=="NIKON_Z8_RUST_NATIVE_RECIPE_SET_V051_V1"&&
        registry.source_commit==SOURCE_COMMIT&&registry.stock_sha256==INPUT_SHA&&
        registry.stock_eg_sha256==INPUT_EG_SHA&&registry.mask_count==15&&
        registry.compiled_units==20&&registry.combinations.len()==15,"Recipe provenance mismatch")?;
    for (i,e) in registry.combinations.iter().enumerate(){
        check(usize::from(e.mask)==i+1&&e.features==mask_features(e.mask),"Invalid recipe masks")?;
    }
    let manifest_raw=fs::read(toolkit.join("release-manifest.json")).map_err(|e|e.to_string())?;
    check(sha(&manifest_raw)==RELEASE_MANIFEST_SHA,"Tool release manifest is modified")?;
    let m:Value=serde_json::from_slice(&manifest_raw).map_err(|e|e.to_string())?;
    check(m["source_commit"]==SOURCE_COMMIT && m["tag"]=="v0.5.1","Not upstream v0.5.1 toolkit")?;
    let compiled=jsonread(&toolkit.join("resources/compiled.json"))?;
    let units=compiled["units"].as_object().ok_or("No compiled units in toolkit")?;
    check(units.len()==20,"Expected exactly 20 compiled units")?;
    for (name,unit) in units{
        let p=unit["resource"].as_str().ok_or("Missing compiled unit resource")?;
        check(p.starts_with("resources/")&&!p.contains(".."),"Unsafe toolkit resource")?;
        let bytes=fs::read(toolkit.join(p)).map_err(|e|format!("{name}: {e}"))?;
        check(unit["bytes"].as_u64()==Some(bytes.len() as u64)&&unit["sha256"].as_str()==Some(sha(&bytes).as_str()),
            "Compiled unit failed SHA/size verification")?;
    }
    Ok(Self{root:root.to_path_buf(),registry})
 }
 fn recipe(&self,mask:u8)->Result<(Recipe,Vec<u8>)>{
    check((1..=15).contains(&mask),"Invalid feature mask")?;
    let e=&self.registry.combinations[mask as usize-1];
    for n in [&e.patch_file,&e.descriptor_file] {
        check(!n.contains("..")&&!n.contains('/')&&!n.contains('\\'),"Invalid recipe filename")?;
    }
    let path=self.root.join(&e.descriptor_file);
    let serialized=fs::read(path).map_err(|e|e.to_string())?;
    check(sha(&serialized)==e.descriptor_sha256,"Recipe descriptor SHA mismatch")?;
    let r:Recipe=serde_json::from_slice(&serialized).map_err(|e|e.to_string())?;
    check(r.schema=="NIKON_Z8_RUST_PATCH_WINDOWS_V051_V1"&&
        r.source_commit==SOURCE_COMMIT&&r.original_fw_sha256==INPUT_SHA&&
        r.original_eg_sha256==INPUT_EG_SHA&&r.mask==mask&&r.features==e.features&&
        r.count==e.edit_count&&r.count==r.windows.len()&&r.patch_bytes==e.patch_bytes&&
        r.patch_sha256==e.patch_sha256&&!r.reviewed_profile.is_empty(),
        "Recipe descriptor is inconsistent")?;
    let blob=fs::read(self.root.join(&e.patch_file)).map_err(|e|e.to_string())?;
    check(blob.len()==e.patch_bytes&&sha(&blob)==e.patch_sha256,"Patch byte content differs")?;
    Ok((r,blob))
 }
 pub fn summary(&self)->Value{json!({"status":"READY","kind":"NATIVE_RUST_ONLY",
    "upstream":"XinmiaoZhao/NikonFirmwarePatch","source_commit":SOURCE_COMMIT,
    "recipe_masks":self.registry.mask_count,"compiled_units":self.registry.compiled_units,
    "recipe_set_sha256":RECIPE_SET_SHA,"input_required_sha256":INPUT_SHA,
    "flash_approved":false})}
}
fn public_words(version:&str)->Result<[u8;12]>{
    let v=version.as_bytes();
    check(v.len()==5&&v[2]==b'.'&&v.iter().enumerate().all(|(i,c)|i==2||c.is_ascii_digit()),
        "Version requires NN.NN")?;
    let mut arr=[0u8;12];
    let vals=[
      0x52800008u32|((u16::from_le_bytes([v[0],v[1]]) as u32)<<5),
      0x72a00008u32|((u16::from_le_bytes([v[2],v[3]]) as u32)<<5),
      0x52800009u32|((v[4] as u32)<<5)];
    for (i,w) in vals.iter().enumerate(){arr[i*4..i*4+4].copy_from_slice(&w.to_le_bytes());}
    Ok(arr)
}
fn identity(version:&str,tag:&str)->Result<String>{
    public_words(version)?;
    let t=tag.as_bytes();
    check(t.len()==2&&t[0].is_ascii_lowercase()&&t[1].is_ascii_digit(),
        "Tag must be lowercase letter+digit")?;
    Ok(format!("{version}.{tag}"))
}
fn apply(eg:&[u8],recipe:&Recipe,blob:&[u8],version:&str,tag:&str)->Result<Vec<u8>>{
    check(sha(eg)==INPUT_EG_SHA,"Not stock C3.11 EG")?;
    let mut result=eg.to_vec();
    let mut end=0usize;let mut blobend=0usize;
    for window in &recipe.windows{
        let off=window.offset;let hi=off.checked_add(window.bytes).ok_or("Window overflow")?;
        let b_hi=window.blob_at.checked_add(window.bytes).ok_or("Blob overflow")?;
        check(off>=end&&hi<=EG_LEN-2&&window.bytes>0&&window.blob_at==blobend&&b_hi<=blob.len(),
            "Overlapping/out-of-range recipe window")?;
        check(sha(&eg[off..hi])==window.old_sha256,"Patch preimage differs from reviewed stock")?;
        check(sha(&blob[window.blob_at..b_hi])==window.new_sha256,"Patch bytes differ from reviewed SHA")?;
        result[off..hi].copy_from_slice(&blob[window.blob_at..b_hi]);
        end=hi;blobend=b_hi;
    }
    check(blobend==blob.len(),"Unconsumed patch blob bytes")?;
    let id=identity(version,tag)?;
    let edits=[
       (0xEB29FC,public_words(version)?.to_vec()),
       (0x14EAA90,id.as_bytes().to_vec()),
       (EG_LEN-11,id.as_bytes().to_vec())];
    for (at,replacement) in edits{
        let stop=at.checked_add(replacement.len()).ok_or("Identity extent overflow")?;
        check(stop<=EG_LEN-2,"Identity beyond firmware EG")?;
        check(!recipe.windows.iter().any(|w|at<w.offset+w.bytes&&w.offset<stop),
            "Identity overlaps reviewed patch region")?;
        result[at..stop].copy_from_slice(&replacement);
    }
    let crc=crc16(&result[..EG_LEN-2]);
    result[EG_LEN-2..].copy_from_slice(&crc.to_be_bytes());
    check(crc16(&result)==0,"EG CRC invalid after patch")?;
    Ok(result)
}
pub fn check_input(path:&Path)->Result<Value>{
    let bytes=fs::read(path).map_err(|e|e.to_string())?;
    let (body,_)=inspect(&bytes,Some(INPUT_SHA))?;
    check(sha(&body[EG..EG+EG_LEN])==INPUT_EG_SHA,"Stock EG SHA mismatch")?;
    Ok(json!({"status":"COMPATIBLE","identity":"03.11.b0","sha256":INPUT_SHA,
        "bytes":PACKAGE,"module_crcs":"5/5","flash_approved":false}))
}
fn non_eg_equal(a:&[u8],b:&[u8])->bool{
    a[..112]==b[..112]&&a[128..EG]==b[128..EG]&&
    a[EG+EG_LEN..a.len()-16]==b[EG+EG_LEN..b.len()-16]
}
fn output_filename(version:&str)->String{format!("Z_8_{}.bin",version.replace(".",""))}
#[derive(Clone)]
pub struct BuildConfig{pub stock:PathBuf,pub output_dir:PathBuf,pub version:String,pub tag:String,
                       pub features:Vec<String>,pub recipes:PathBuf,pub toolkit:PathBuf}
pub fn build(cfg:&BuildConfig)->Result<Value>{
    let id=identity(&cfg.version,&cfg.tag)?;
    let mask=feature_mask(&cfg.features)?;
    check(!cfg.output_dir.exists(),"Output folder exists; refusing overwrite")?;
    check(cfg.output_dir!=cfg.stock,"Unsafe output path")?;
    let assets=Assets::open(&cfg.recipes,&cfg.toolkit)?;
    let stock=fs::read(&cfg.stock).map_err(|e|format!("Stock input: {e}"))?;
    let (original,tables)=inspect(&stock,Some(INPUT_SHA))?;
    let before=&original[EG..EG+EG_LEN];
    let (recipe,blob)=assets.recipe(mask)?;
    let patched=apply(before,&recipe,&blob,&cfg.version,&cfg.tag)?;
    let mut updated=original.clone();
    let name=format!("eg2070_{}{}.bi",cfg.version.replace(".",""),cfg.tag);
    check(name.len()<=16,"Internal module name too long")?;
    updated[112..128].fill(0);
    updated[112..112+name.len()].copy_from_slice(name.as_bytes());
    updated[EG..EG+EG_LEN].copy_from_slice(&patched);
    let bodycrc=crc16(&updated[..updated.len()-16]);
    let tail=updated.len()-16;updated[tail..tail+2].copy_from_slice(&bodycrc.to_be_bytes());
    check(crc16(&updated[..updated.len()-14])==0,"Repack CRC invalid")?;
    let mut result=Vec::with_capacity(PACKAGE);
    result.extend_from_slice(&auth(&updated));result.extend_from_slice(&OUTER);
    result.extend_from_slice(&crypt(&updated,&tables));
    check(result.len()==PACKAGE,"Wrong resulting package size")?;
    let (decoded,_)=inspect(&result,None)?;
    check(non_eg_equal(&original,&decoded),"Non-EG module bytes changed")?;
    check(decoded[EG..EG+EG_LEN]==patched,"EG failed roundtrip")?;
    for (i,(start,len)) in MODS.iter().enumerate(){
        if i!=2{check(original[*start..*start+*len]==decoded[*start..*start+*len],"Other module changed")?;}
    }
    let output_sha=sha(&result);
    let check_original=sha(&fs::read(&cfg.stock).map_err(|e|e.to_string())?);
    check(check_original==INPUT_SHA,"Original firmware changed during build")?;
    if mask==15&&cfg.version=="04.08"&&cfg.tag=="p0"{
        check(output_sha==ALL_SHA,"Failed bit-exact official v0.5.1 reference build")?;
    }
    fs::create_dir(&cfg.output_dir).map_err(|e|format!("New output directory: {e}"))?;
    let firmware=cfg.output_dir.join(output_filename(&cfg.version));
    let output=(||->Result<Value>{
        let mut f=OpenOptions::new().create_new(true).write(true).open(&firmware)
             .map_err(|e|e.to_string())?;
        f.write_all(&result).and_then(|_|f.sync_all()).map_err(|e|e.to_string())?;
        check(sha(&fs::read(&firmware).map_err(|e|e.to_string())?)==output_sha,
            "Readback SHA-256 mismatch")?;
        let manifest=json!({
            "schema":"NIKON_Z8_RUST_NATIVE_BUILDER_V051_V1",
            "status":"BUILT_AND_LOCALLY_VERIFIED","input_identity":"03.11.b0",
            "output_identity":id,"input_sha256":INPUT_SHA,
            "output_sha256":output_sha,"output_eg_sha256":sha(&patched),
            "output_bytes":PACKAGE,"output_filename":output_filename(&cfg.version),
            "features":mask_features(mask),"feature_mask":mask,
            "recipe_set_sha256":RECIPE_SET_SHA,"upstream_commit":SOURCE_COMMIT,
            "patch_window_count":recipe.count,"bit_exact_release_reference":mask==15&&id=="04.08.p0",
            "repack_body_crc16":true,"five_module_crc16":true,
            "independent_rust_roundtrip":true,"flash_approved":false,
            "notes":"Does not prove device-level operation; output is experimental"
        });
        fs::write(cfg.output_dir.join("SHA256SUMS.txt"),
            format!("{}  {}\n",output_sha,output_filename(&cfg.version)))
            .map_err(|e|e.to_string())?;
        fs::write(cfg.output_dir.join("manifest.json"),
            serde_json::to_vec_pretty(&manifest).map_err(|e|e.to_string())?)
            .map_err(|e|e.to_string())?;
        let readback=verify(&cfg.stock,&firmware,&cfg.output_dir.join("manifest.json"),&assets)?;
        check(readback["status"]=="RUST_PACKAGE_VERIFIED","Independent file readback failed")?;
        Ok(manifest)
    })();
    if output.is_err(){
        // Fail-closed: rename incomplete binary before returning error.
        if firmware.exists(){let _=fs::rename(&firmware,firmware.with_extension("bin.unverified"));}
    }
    output
}
pub fn verify(stock:&Path,candidate:&Path,config:&Path,assets:&Assets)->Result<Value>{
    let manifest=jsonread(config)?;
    let version=manifest["output_identity"].as_str().ok_or("Missing identity")?;
    let (v,t)=version.split_once('.').and_then(|(a,b)|b.rsplit_once('.').map(|(c,d)|(format!("{a}.{c}"),d.to_owned())))
        .ok_or("Bad output version")?;
    let names:Vec<String>=serde_json::from_value(manifest["features"].clone()).map_err(|e|e.to_string())?;
    let mask=feature_mask(&names)?;
    check(manifest["feature_mask"]==mask&&manifest["recipe_set_sha256"]==RECIPE_SET_SHA&&
        manifest["upstream_commit"]==SOURCE_COMMIT,"Output manifest provenance differs")?;
    check(identity(&v,&t)?==version,"Incorrect output identity")?;
    let src=fs::read(stock).map_err(|e|e.to_string())?;
    let dst=fs::read(candidate).map_err(|e|e.to_string())?;
    let (before,_)=inspect(&src,Some(INPUT_SHA))?;
    let (after,_)=inspect(&dst,None)?;
    check(sha(&dst)==manifest["output_sha256"].as_str().ok_or("Missing SHA")?,"Output SHA mismatch")?;
    check(non_eg_equal(&before,&after),"Non-EG bytes differ")?;
    check(after[EG+0x14EAA90..EG+0x14EAA98]==version.as_bytes()[..],
        "Output version mismatch")?;
    let (recipe,blob)=assets.recipe(mask)?;
    check(recipe.count==manifest["patch_window_count"].as_u64().unwrap_or(0) as usize,
        "Review window count does not match manifest")?;
    let predicted=apply(&before[EG..EG+EG_LEN],&recipe,&blob,&v,&t)?;
    check(after[EG..EG+EG_LEN]==predicted,
        "Firmware EG differs from reviewed 15-mask recipe at one or more bytes")?;
    let name=format!("eg2070_{}{}.bi",v.replace(".",""),t);
    let mut directory=[0u8;16];directory[..name.len()].copy_from_slice(name.as_bytes());
    check(after[112..128]==directory,"Firmware EG directory entry differs")?;
    check(mask!=15||version!="04.08.p0"||sha(&dst)==ALL_SHA,
        "Canonical all-features image SHA mismatch")?;
    Ok(json!({"status":"RUST_PACKAGE_VERIFIED","sha256":sha(&dst),
        "mask":mask,"identity":version,"manifest_matched":true,
        "module_crc16":"5/5","flash_approved":false}))
}
#[cfg(test)]
mod tests{
use super::*;
#[test]fn crc_vector(){assert_eq!(crc16(b"123456789"),0x31c3);}
#[test]fn no_empty_mask(){assert!(feature_mask(&[]).is_err());}
#[test]fn reject_version(){assert!(identity("4.8","p0").is_err());assert!(identity("04.08","P0").is_err());}
#[test]fn xor_reversible(){let t=Tables{a:[5;256],b:[23;256],c:[91;256]};
let d=vec![9u8;70_000];assert_eq!(crypt(&crypt(&d,&t),&t),d);}
}
