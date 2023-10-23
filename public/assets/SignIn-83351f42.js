import{b as f,a0 as F,a as S,F as w,j as s,E as N,D as v,a1 as I}from"./index-3f76b7c9.js";import{u as y,t as E,H as k,F as D,a as i,b as o,c as l,d as m,I as c,e as d,z as e}from"./Input-fd74bc13.js";import{B as P,b as U}from"./Button-89fabcac.js";const B=e.object({email:e.string().email({message:"Invalid email"}),password:e.string().min(8).max(72)});function z(){const n=y({resolver:E(B),defaultValues:{email:"",password:""}}),u=f(),[h]=F(),{cloud:{initNotes:x}}=S(),{initUser:j}=w(),{control:r,handleSubmit:p,formState:{isSubmitting:t}}=n,b=async({email:a,password:g})=>{await I(a,g).then(()=>{x(""),j().then(()=>u(h.get("redirect_url")??"/"))})};return s.jsxs(s.Fragment,{children:[s.jsxs("div",{children:[s.jsx(k,{className:"h-10 w-10 mb-6"}),s.jsx("h2",{className:"text-2xl font-semibold mb-10",children:"Welcome back!"}),s.jsx(D,{...n,children:s.jsxs("form",{onSubmit:p(b),className:"w-80 space-y-8",children:[s.jsx(i,{control:r,name:"email",render:({field:a})=>s.jsxs(o,{children:[s.jsx(l,{children:"Email"}),s.jsx(m,{children:s.jsx(c,{placeholder:"Email",...a})}),s.jsx(d,{children:"No spam. Used only for auth."})]})}),s.jsx(i,{control:r,name:"password",render:({field:a})=>s.jsxs(o,{children:[s.jsx(l,{children:"Password"}),s.jsx(m,{children:s.jsx(c,{placeholder:"Password",type:"password",...a})}),s.jsx(d,{children:"Must be 8 - 72 characters long. Super secret."})]})}),s.jsxs(P,{type:"submit",disabled:t,children:[t?s.jsx(N,{className:"h-4 w-4 mr-2"}):null,"Sign in"]})]})})]}),s.jsx(v,{to:"/auth/sign-up",className:U({variant:"link",className:"absolute bottom-10"}),children:"Don't have an account? Sign up."})]})}export{z as default};
