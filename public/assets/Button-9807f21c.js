import{a3 as g,r as N,j as V,A as j,e as w}from"./index-a7950236.js";const f=n=>typeof n=="boolean"?"".concat(n):n===0?"0":n,b=g,C=(n,r)=>e=>{var i;if((r==null?void 0:r.variants)==null)return b(n,e==null?void 0:e.class,e==null?void 0:e.className);const{variants:l,defaultVariants:a}=r,u=Object.keys(l).map(t=>{const s=e==null?void 0:e[t],d=a==null?void 0:a[t];if(s===null)return null;const o=f(s)||f(d);return l[t][o]}),v=e&&Object.entries(e).reduce((t,s)=>{let[d,o]=s;return o===void 0||(t[d]=o),t},{}),x=r==null||(i=r.compoundVariants)===null||i===void 0?void 0:i.reduce((t,s)=>{let{class:d,className:o,...y}=s;return Object.entries(y).every(h=>{let[m,c]=h;return Array.isArray(c)?c.includes({...a,...v}[m]):{...a,...v}[m]===c})?[...t,d,o]:t},[]);return b(n,u,x,e==null?void 0:e.class,e==null?void 0:e.className)},k=C("inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50",{variants:{variant:{default:"bg-primary text-primary-foreground shadow hover:bg-primary/90",destructive:"bg-destructive text-destructive-foreground shadow-sm hover:bg-destructive/90",outline:"border border-input bg-transparent shadow-sm hover:bg-accent hover:text-accent-foreground",secondary:"bg-secondary text-secondary-foreground shadow-sm hover:bg-secondary/80",ghost:"hover:bg-accent hover:text-accent-foreground",link:"text-primary underline-offset-4 hover:underline"},size:{default:"h-9 px-4 py-2",sm:"h-8 rounded-md px-3 text-xs",lg:"h-10 rounded-md px-8",icon:"h-9 w-9"}},defaultVariants:{variant:"default",size:"default"}}),O=N.forwardRef(({className:n,variant:r,size:e,asChild:i=!1,...l},a)=>{const u=i?w:"button";return V.jsx(u,{className:j(k({variant:r,size:e,className:n})),ref:a,...l})});O.displayName="Button";export{O as B,k as b,C as c};
