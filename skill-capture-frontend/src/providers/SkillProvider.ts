import { mkSkill } from './../models/Skills';
export const listSkills = () => {
    return fetch("http://d7de-91-143-223-13.ngrok.io/skill").then(a => a.json()).then(xs => xs.map((sk: any) => mkSkill(sk.name, sk.category)));
}
