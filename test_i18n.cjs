"use strict";
// Tests the actual i18n.js using a simulated browser DOM, without external dependencies.
const assert=require("node:assert/strict");
const vm=require("node:vm");
const fs=require("node:fs");
const path=require("node:path");
const source=fs.readFileSync(path.join(__dirname,"src","i18n.js"),"utf8");
const languages=["ro","en","de","ja","zh-CN","zh-TW","es","it","fr","pl","nb","sv","pt","ko","fi"];
const cases=[
  ["ro-RO","ro"],["en-US","en"],["de-DE","de"],["ja-JP","ja"],
  ["zh-Hans-CN","zh-CN"],["zh-CN","zh-CN"],["zh-SG","zh-CN"],
  ["zh-Hant-TW","zh-TW"],["zh-TW","zh-TW"],["zh-HK","zh-TW"],
  ["es-ES","es"],["it-IT","it"],["fr-FR","fr"],["pl-PL","pl"],
  ["no-NO","nb"],["nb-NO","nb"],["sv-SE","sv"],["pt-PT","pt"],["pt-BR","pt"],["ko-KR","ko"],["fi-FI","fi"],
  ["ru-RU","ro"],["uk-UA","ro"]
];
function simulate(browserLanguage,stored){
  const byId=new Map(),bySelector=new Map(),saved=[];
  const text=value=>({nodeType:3,textContent:value||""});
  function createNode(tag){
    const node={tagName:tag.toUpperCase(),nodeType:1,childNodes:[],children:[],dataset:{},
      listeners:{},textContent:"",value:"",
      appendChild(child){this.childNodes.push(child);this.children.push(child);return child;},
      setAttribute(k,v){this[k]=v;},
      addEventListener(k,f){this.listeners[k]=f;}
    };
    Object.defineProperty(node,"id",{get(){return this._id;},set(v){this._id=v;byId.set(v,this);}});
    return node;
  }
  const status=createNode("pre");status.id="state";status.textContent="Se încarcă...";
  const stock=createNode("input");stock.id="stock";
  const dest=createNode("input");dest.id="dest";
  const document={
    readyState:"complete",documentElement:{lang:""},title:"",
    body:{firstChild:null,insertBefore(node){this.firstChild=node;}},
    createElement:createNode,createTextNode:text,
    getElementById(id){return byId.get(id)||null;},
    querySelector(selector){
      if(selector.startsWith("#"))return byId.get(selector.slice(1))||null;
      if(!bySelector.has(selector)){
        const e=createNode("label");
        e.childNodes=selector.includes("label")?
          [{nodeType:1,immutable:true},text("old text")]:
          [text("old text")];
        bySelector.set(selector,e);
      }
      return bySelector.get(selector);
    }
  };
  const storage={getItem(){return stored||null;},setItem(k,v){saved.push([k,v]);}};
  const sandbox={document,localStorage:storage,navigator:{language:browserLanguage},
    Node:{TEXT_NODE:3},window:{},console};
  vm.runInNewContext(source,sandbox,{timeout:3000,filename:"i18n.js"});
  const lang=byId.get("language");
  assert.ok(lang,"Language selector must exist");
  assert.deepEqual(lang.children.map(o=>o.value),languages,"Only the requested 15 locales");
  assert.ok(!lang.children.some(o=>o.value==="ru"||o.value==="uk"));
  assert.ok(byId.get("language-control"));
  assert.ok(byId.get("language-label"));
  assert.ok(stock.placeholder.length>0&&dest.placeholder.length>0);
  assert.ok(document.querySelector("h1").childNodes[0].textContent.length>0);
  assert.equal(document.querySelector("section:nth-of-type(3) fieldset label:nth-of-type(1)").childNodes[0].immutable,true,
    "Feature checkbox replaced by translated text");
  for(const key of ["busy","loading","warning","footer","buildButton"]){
    assert.ok(!String(sandbox.window.z8t(key)).includes("undefined"),"Missing translation "+key);
    assert.ok(String(sandbox.window.z8t(key)).trim().length>2);
  }
  return {document,lang,sandbox,saved};
}
for(const [browser,expected] of cases){
  const r=simulate(browser);
  assert.equal(r.document.documentElement.lang,expected,"Auto locale "+browser);
  assert.equal(r.lang.value,expected,"Dropdown "+browser);
  console.log("PASS locale",browser,"=>",expected);
}
for(const code of languages){
  const r=simulate("ro-RO",code);
  assert.equal(r.document.documentElement.lang,code,"Saved locale "+code);
  assert.equal(r.lang.value,code);
  assert.ok(String(r.document.querySelector("h1").childNodes[0].textContent).includes("Nikon Z8"),"Title brand missing "+code);
  assert.ok(r.document.querySelector("body > p.warning").childNodes[0].textContent.length>15,
    "Safety warning missing "+code);
  assert.ok(r.document.querySelector("section:nth-of-type(3) p small").childNodes[0].textContent.length>15);
  r.lang.value="en";
  r.lang.listeners.change();
  assert.equal(r.document.documentElement.lang,"en","Switch failed "+code);
  assert.equal(r.lang.value,"en");
  assert.ok(r.saved.some(v=>v[0]==="z8-builder-language"&&v[1]==="en"));
  console.log("PASS translation & switch",code);
}
console.log("NIKON_Z8_I18N_PASS",languages.length,"locales",cases.length,"browser cases");
