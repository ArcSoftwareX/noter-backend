import{u as l,a,j as e,L as m,C as u,b as i,r as x}from"./index-a7950236.js";import{B as d}from"./Button-9807f21c.js";function f(){const{open:t}=l(),{create:s}=a(),o=[{name:"Create a note",shortcut:"Q",func:s},{name:"Command palette",shortcut:"P",func:t},{name:"Launch tutorial",shortcut:"H",func:null}];return e.jsxs("div",{className:"h-full w-full flex items-center justify-center flex-col",children:[e.jsx(m,{className:"h-36 w-36 mb-6"}),e.jsx("h1",{className:"text-xl text-primary/90 font-semibold",children:"Welcome to Noter"}),e.jsx("p",{className:"text-sm text-muted-foreground mb-6",children:"It looks like you are new. Try this out."}),o.map(({func:r,name:c,shortcut:n})=>e.jsx(h,{func:r,name:c,shortcut:n},n))]})}const h=({func:t,name:s,shortcut:o})=>e.jsxs(d,{disabled:!t,onClick:t,variant:"ghost",className:"flex justify-between w-64 text-muted-foreground outline-none",children:[s,e.jsxs("kbd",{className:"flex h-5 select-none items-center gap-1 rounded border bg-muted px-1.5 font-mono text-sm font-medium opacity-100 sm:flex",children:[e.jsx(u,{className:"h-3.5 w-3.5"}),o]})]});function N(){const t=i(),{hasNotes:s,notes:o}=a();return x.useEffect(()=>{s()&&t("/notes",{replace:!0})},[s,t,o]),e.jsx("div",{className:"w-screen h-full",children:e.jsx(f,{})})}export{N as default};
