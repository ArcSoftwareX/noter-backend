import{$ as w,r as t,h as $,_ as l,g as S,Z as g,j as i,A as u}from"./index-e84aa537.js";const m="Avatar",[_,P]=w(m),[L,p]=_(m),h=t.forwardRef((e,r)=>{const{__scopeAvatar:a,...c}=e,[o,s]=t.useState("idle");return t.createElement(L,{scope:a,imageLoadingStatus:o,onImageLoadingStatusChange:s},t.createElement($.span,l({},c,{ref:r})))}),N="AvatarImage",E=t.forwardRef((e,r)=>{const{__scopeAvatar:a,src:c,onLoadingStatusChange:o=()=>{},...s}=e,n=p(N,a),d=C(c),f=S(b=>{o(b),n.onImageLoadingStatusChange(b)});return g(()=>{d!=="idle"&&f(d)},[d,f]),d==="loaded"?t.createElement($.img,l({},s,{ref:r,src:c})):null}),R="AvatarFallback",y=t.forwardRef((e,r)=>{const{__scopeAvatar:a,delayMs:c,...o}=e,s=p(R,a),[n,d]=t.useState(c===void 0);return t.useEffect(()=>{if(c!==void 0){const f=window.setTimeout(()=>d(!0),c);return()=>window.clearTimeout(f)}},[c]),n&&s.imageLoadingStatus!=="loaded"?t.createElement($.span,l({},o,{ref:r})):null});function C(e){const[r,a]=t.useState("idle");return g(()=>{if(!e){a("error");return}let c=!0;const o=new window.Image,s=n=>()=>{c&&a(n)};return a("loading"),o.onload=s("loaded"),o.onerror=s("error"),o.src=e,()=>{c=!1}},[e]),r}const v=h,A=E,x=y,I=t.forwardRef(({className:e,...r},a)=>i.jsx(v,{ref:a,className:u("relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full",e),...r}));I.displayName=v.displayName;const M=t.forwardRef(({className:e,...r},a)=>i.jsx(A,{ref:a,className:u("aspect-square h-full w-full",e),...r}));M.displayName=A.displayName;const j=t.forwardRef(({className:e,...r},a)=>i.jsx(x,{ref:a,className:u("flex h-full w-full items-center justify-center rounded-full bg-muted",e),...r}));j.displayName=x.displayName;export{I as A,j as a,M as b};
