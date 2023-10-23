import{c as k,j as e,A as u,u as C,T as m,H as f,L as y,I as p,C as L,r as n,a as M,J as S,K as D,D as T,B as z,S as j}from"./index-ca491c9c.js";import{A as W,a as E}from"./Avatar-e8af05b1.js";import g from"./NotFound-72d390cd.js";import{b as I}from"./Button-5c727687.js";const P=k("ChevronLeft",[["path",{d:"m15 18-6-6 6-6",key:"1wnfg3"}]]),_=({className:s,children:t,...r})=>e.jsx("h4",{className:u("scroll-m-20 text-base font-semibold tracking-tight",s),...r,children:t}),U=({className:s,children:t,...r})=>e.jsx("h4",{className:u("text-sm text-muted-foreground",s),...r,children:t});function A({title:s,desc:t}){const{open:r}=C();return e.jsxs("div",{className:"fixed top-0 inset-x-0 h-16 py-2 bg-secondary/60 border-b border-b-border backdrop-blur-xl flex items-center justify-stretch px-4 z-50",children:[e.jsxs(m,{children:[e.jsx(f,{className:"mr-4",children:e.jsx(W,{onClick:r,children:e.jsx(E,{children:e.jsx(y,{className:"h-full w-full"})})})}),e.jsxs(p,{className:"flex items-center",children:["Command palette",e.jsxs("span",{className:"flex items-center ml-2 text-muted-foreground",children:[e.jsx(L,{className:"h-4"})," P"]})]})]}),e.jsxs("div",{children:[e.jsx(_,{children:s!=null&&s.length?s:"Untitled"}),t?e.jsxs(m,{children:[e.jsx(f,{children:e.jsx(U,{className:"whitespace-nowrap max-w-[calc(100vw-100px)] sm:max-w-[calc(100vw-200px)] overflow-hidden text-ellipsis",children:t})}),e.jsx(p,{className:"flex items-center",children:t})]}):null]})]})}const B=({className:s,...t})=>e.jsx("div",{className:u("w-full bg-secondary/50 animate-pulse rounded-lg",s),...t}),F=n.forwardRef(({...s},t)=>e.jsxs("svg",{xmlns:"http://www.w3.org/2000/svg",width:"1em",height:"1em",viewBox:"0 0 24 24",ref:t,...s,children:[e.jsxs("defs",{children:[e.jsx("clipPath",{id:"lineMdWatchLoop0",children:e.jsx("rect",{width:"24",height:"12"})}),e.jsx("symbol",{id:"lineMdWatchLoop1",children:e.jsx("path",{fill:"none",stroke:"#fff",strokeLinecap:"round",strokeLinejoin:"round",strokeWidth:"2",d:"M23 16.5C23 10.4249 18.0751 5.5 12 5.5C5.92487 5.5 1 10.4249 1 16.5z",clipPath:"url(#lineMdWatchLoop0)",children:e.jsx("animate",{attributeName:"d",dur:"6s",keyTimes:"0;0.07;0.93;1",repeatCount:"indefinite",values:"M23 16.5C23 11.5 18.0751 12 12 12C5.92487 12 1 11.5 1 16.5z;M23 16.5C23 10.4249 18.0751 5.5 12 5.5C5.92487 5.5 1 10.4249 1 16.5z;M23 16.5C23 10.4249 18.0751 5.5 12 5.5C5.92487 5.5 1 10.4249 1 16.5z;M23 16.5C23 11.5 18.0751 12 12 12C5.92487 12 1 11.5 1 16.5z"})})}),e.jsxs("mask",{id:"lineMdWatchLoop2",children:[e.jsx("use",{href:"#lineMdWatchLoop1"}),e.jsx("use",{href:"#lineMdWatchLoop1",transform:"rotate(180 12 12)"}),e.jsx("circle",{cx:"12",cy:"12",r:"0",fill:"#fff",children:e.jsx("animate",{attributeName:"r",dur:"6s",keyTimes:"0;0.03;0.97;1",repeatCount:"indefinite",values:"0;3;3;0"})})]})]}),e.jsx("rect",{width:"24",height:"24",fill:"#888888",mask:"url(#lineMdWatchLoop2)"})]})),R=n.forwardRef(({...s},t)=>e.jsx("svg",{xmlns:"http://www.w3.org/2000/svg",width:"1em",height:"1em",viewBox:"0 0 24 24",ref:t,...s,children:e.jsxs("g",{fill:"none",stroke:"#888888",strokeLinecap:"round",strokeWidth:"2",children:[e.jsx("path",{strokeDasharray:"2 4",strokeDashoffset:"6",d:"M12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3",children:e.jsx("animate",{attributeName:"stroke-dashoffset",dur:"0.6s",repeatCount:"indefinite",values:"6;0"})}),e.jsx("path",{strokeDasharray:"30",strokeDashoffset:"30",d:"M12 3C16.9706 3 21 7.02944 21 12C21 16.9706 16.9706 21 12 21",children:e.jsx("animate",{fill:"freeze",attributeName:"stroke-dashoffset",begin:"0.1s",dur:"0.3s",values:"30;0"})}),e.jsx("path",{strokeDasharray:"10",strokeDashoffset:"10",d:"M12 16v-7.5",children:e.jsx("animate",{fill:"freeze",attributeName:"stroke-dashoffset",begin:"0.5s",dur:"0.2s",values:"10;0"})}),e.jsx("path",{strokeDasharray:"6",strokeDashoffset:"6",d:"M12 8.5l3.5 3.5M12 8.5l-3.5 3.5",children:e.jsx("animate",{fill:"freeze",attributeName:"stroke-dashoffset",begin:"0.7s",dur:"0.2s",values:"6;0"})})]})})),H=n.lazy(()=>z(()=>import("./NoteEditor-ba56f49c.js"),["assets/NoteEditor-ba56f49c.js","assets/index-ca491c9c.js","assets/index-38881b76.css","assets/LexicalHtml-bc570db1.js","assets/Button-5c727687.js"]));function G(){const{update:s,cloud:{update:t,isLoaded:r,isUpdating:v},get:x}=M(),d=S().id,[a,c]=n.useState(d?x(d):null),[w,h]=n.useState(!1),i=D(a);if(n.useEffect(()=>{h(!0)},[a]),n.useEffect(()=>{i&&(console.log("sync note"),i.isCloud?t(i.id,i):s(i.id,i),h(!1))},[i,s,t]),n.useEffect(()=>{c(d?x(d):null)},[x,r,d]),!d)return e.jsx(g,{});if(!a&&!r)return"loading note from cloud";if(!a&&r)return e.jsx(g,{});const N=l=>{c(o=>o?{...o,title:l.target.value.length?l.target.value:null}:null)},b=l=>c(o=>o?{...o,description:l.target.value.length?l.target.value:null}:null);return e.jsxs("div",{className:"max-h-screen w-screen overflow-y-scroll scrollbar-hidden pb-10",children:[e.jsx(A,{title:a.title??"Untitled",desc:a.description}),e.jsxs("div",{className:"2xl:max-w-7xl xl:max-w-6xl lg:max-w-4xl md:max-w-3xl sm:max-w-none mx-auto px-10 pt-20",children:[e.jsxs(T,{to:"/notes",className:I({variant:"ghost",className:"text-muted-foreground pl-2"}),children:[e.jsx(P,{className:"h-5 w-5 mr-1"}),"My notes"]}),e.jsx("input",{className:"placeholder:text-muted-foreground bg-transparent text-primary w-full outline-none mt-10",placeholder:"No description",value:a.description??"",onChange:b}),e.jsx("input",{className:"text-4xl font-bold placeholder:text-muted-foreground bg-transparent text-primary w-full outline-none mb-6",placeholder:"Untitled",value:a.title??"",onChange:N}),e.jsx(n.Suspense,{fallback:e.jsx("div",{className:"w-full",children:[1,2,3,4].map(l=>e.jsx(B,{className:"h-24 mb-4"},l))}),children:e.jsx(H,{editorState:a.content,save:l=>c(o=>o?{...o,content:l}:null)})})]}),e.jsx("div",{className:"text-muted-foreground fixed left-10 bottom-10 flex items-center text-sm gap-2 px-3 py-2 rounded-lg backdrop-blur-xl",children:e.jsx(V,{state:a!=null&&a.isCloud?v?"Saving":w?"Unsaved":"Saved":null})})]})}const V=({state:s})=>s?e.jsxs(e.Fragment,{children:[s==="Saved"?e.jsx(j,{className:"h-5 w-5"}):s==="Saving"?e.jsx(R,{className:"h-5 w-5"}):e.jsx(F,{className:"w-5 h-5"})," ",s]}):e.jsxs(e.Fragment,{children:[e.jsx(j,{className:"h-5 w-5"}),"Saved locally"]});export{G as default};
