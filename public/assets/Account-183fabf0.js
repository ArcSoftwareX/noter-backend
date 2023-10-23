import{c as v,M as N,N as x,r as o,j as e,O as b,Q as D,U as y,V as k,W as C,X as w,E as A,Y as F,F as S,a as I,b as O,D as E}from"./index-7c3f564b.js";import{B as m,b as L}from"./Button-3d8b745e.js";import{A as T,b as U,a as P}from"./Avatar-824f412f.js";const d=v("ImagePlus",[["path",{d:"M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7",key:"31hg93"}],["line",{x1:"16",x2:"22",y1:"5",y2:"5",key:"ez7e4s"}],["line",{x1:"19",x2:"19",y1:"2",y2:"8",key:"1gkr8c"}],["circle",{cx:"9",cy:"9",r:"2",key:"af1f0g"}],["path",{d:"m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21",key:"1xmnt7"}]]),R=async s=>{const t=new FormData;t.append("file",s);const{data:r}=await N.put(`${x}/avatar`,t,{withCredentials:!0});return r};function _({avatar_id:s}){const[t,r]=o.useState(null),[n,l]=o.useState(!1),[u,c]=o.useState(!1),i=o.useRef(null),g=()=>{t&&(l(!0),R(t).then(a=>{l(!1),c(!1),console.log(a)}))},f=()=>{i.current&&i.current.click()},h=a=>{!a.target||!a.target.files||r(a.target.files[0])},p=a=>a.preventDefault(),j=a=>{a.preventDefault(),console.log(a.dataTransfer.files[0]),r(a.dataTransfer.files[0])};return e.jsxs(e.Fragment,{children:[e.jsxs(b,{open:u,onOpenChange:c,children:[e.jsx(D,{children:e.jsxs(T,{className:"mr-4",children:[s&&!n?e.jsx(U,{src:`${x}/avatar`}):null,e.jsx(P,{children:e.jsx(d,{className:"h-4 w-4"})})]})}),e.jsxs(y,{className:"sm:max-w-md",children:[e.jsx(k,{children:e.jsx(C,{children:"Change avatar"})}),e.jsx("div",{className:"py-4 cursor-pointer",children:e.jsxs("div",{onDragStart:a=>a.dataTransfer.dropEffect="copy",onDrop:j,onDragOver:p,onClick:f,className:"h-40 w-full flex items-center justify-center flex-col border-2 border-border border-dashed rounded-lg relative",children:[e.jsx(d,{className:"h-6 w-6 mb-4 text-muted-foreground"}),e.jsx("p",{className:"text-xs text-muted-foreground font-semibold max-w-[50%] text-center balanced",children:"Drag and drop your image here or click to open file picker."}),e.jsx("p",{className:"absolute bottom-5 text-sm text-muted-foreground",children:t?t.name:null})]})}),e.jsx(w,{children:e.jsxs(m,{onClick:g,disabled:n||!t,children:[n?e.jsx(A,{className:"h-4 w-4 mr-2"}):null,"Change"]})})]})]}),F.createPortal(e.jsx("input",{onChange:h,className:"hidden",type:"file",accept:"image/png, image/jpeg",ref:i}),document.body)]})}function H(){const{user:s,signOut:t,isLoading:r}=S(),{cloud:{hideCloudNotes:n}}=I(),l=O();return!s&&!r?e.jsxs("div",{className:"flex items-center justify-center flex-col",children:[e.jsx("h1",{className:"text-xl font-bold mb-4",children:"You are not signed in"}),e.jsx(E,{to:"/auth/sign-in?redirect_url=%2Faccount",className:L({variant:"link"}),children:"Sign in"})]}):e.jsx("div",{className:"h-screen w-screen",children:e.jsxs("div",{className:"border border-border m-5 p-5 rounded-lg flex items-start",children:[e.jsx(_,{avatar_id:(s==null?void 0:s.avatar_id)??null}),e.jsxs("div",{children:[e.jsx("h1",{className:"text-xl font-bold",children:"Welcome!"}),e.jsx("p",{className:"text-sm text-muted-foreground",children:s==null?void 0:s.email})]}),e.jsx("span",{className:"flex-1"}),e.jsx(m,{onClick:()=>t().then(()=>{n(),l("/")}),children:"Sign out"})]})})}export{H as default};
