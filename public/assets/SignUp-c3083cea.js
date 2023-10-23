import{b as N,a0 as F,a as S,F as w,j as s,E as v,D as y,a2 as I}from"./index-eddd97d0.js";import{u as U,t as E,H,F as P,a as r,b as t,c as i,d as o,I as l,e as m,z as a}from"./Input-2b42254b.js";import{B as k,b as A}from"./Button-b11fd42b.js";const B=a.object({email:a.string().email({message:"Invalid email"}),password:a.string().min(8).max(72),name:a.string().min(2).max(32).optional()});function z(){const c=U({resolver:E(B),defaultValues:{email:"",password:"",name:""}}),h=N(),[x]=F(),{cloud:{initNotes:u}}=S(),{initUser:j}=w(),{control:n,handleSubmit:p,formState:{isSubmitting:d}}=c,b=async({email:e,password:g,name:f})=>{await I(e,f??null,g).then(()=>{u(""),j().then(()=>{h(x.get("redirect_url")??"/")})})};return s.jsxs(s.Fragment,{children:[s.jsxs("div",{children:[s.jsx(H,{className:"h-10 w-10 mb-6"}),s.jsx("h2",{className:"text-2xl font-semibold mb-10",children:"Hello there!"}),s.jsx(P,{...c,children:s.jsxs("form",{onSubmit:p(b),className:"w-80 space-y-8",children:[s.jsx(r,{control:n,name:"email",render:({field:e})=>s.jsxs(t,{children:[s.jsx(i,{children:"Email"}),s.jsx(o,{children:s.jsx(l,{placeholder:"Email",...e})}),s.jsx(m,{children:"No spam. Used only for auth."})]})}),s.jsx(r,{control:n,name:"name",render:({field:e})=>s.jsxs(t,{children:[s.jsx(i,{children:"Name"}),s.jsx(o,{children:s.jsx(l,{placeholder:"Name",type:"text",...e})}),s.jsx(m,{children:"Optional."})]})}),s.jsx(r,{control:n,name:"password",render:({field:e})=>s.jsxs(t,{children:[s.jsx(i,{children:"Password"}),s.jsx(o,{children:s.jsx(l,{placeholder:"Password",type:"password",...e})}),s.jsx(m,{children:"Must be 8 - 72 characters long. Super secret."})]})}),s.jsxs(k,{type:"submit",disabled:d,children:[d?s.jsx(v,{className:"h-4 w-4 mr-2"}):null,"Sign up"]})]})})]}),s.jsx(y,{to:"/auth/sign-in",className:A({variant:"link",className:"absolute bottom-10"}),children:"Already have an account? Sign in."})]})}export{z as default};
