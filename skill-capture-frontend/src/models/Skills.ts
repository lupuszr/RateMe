export type Skill = {
    name: string;
    category: string;
}

type mkSkillT = (name: string, category: string) => Skill;
export const mkSkill: mkSkillT = (name: string, category: string) => ({
    name,
    category
});