import { mkSkill } from './../models/Skills';
export const listSkills = () => {
    return fetch("http://localhost:8000/skills").then(a => a.json()).then(xs => xs.map((sk: any) => mkSkill(sk.name, sk.category)));
}
