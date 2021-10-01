import { Employee } from "./Employee";
import { Skill } from "./Skills";


type Score = 1 | 2 | 3 | 4 | 5;

export type SkillRateHistoryItem = {
    score: Score;
    whoRated: Employee;
    createdAt: String;
};
export type RatedSkill = {
    history: Array<SkillRateHistoryItem>;
    employee: Employee;
    skill: Skill;
}

type mkRatedSkillT = (history: Array<SkillRateHistoryItem>, employee: Employee, skill: Skill) => RatedSkill;
export const mkRatedSkill: mkRatedSkillT = (history, employee, skill) => ({
    history,
    skill,
    employee
});

export type ComputedRatedSkill = {
    history: Array<SkillRateHistoryItem>;
    employee: Employee;
    skill: Skill;
    agreggatedScore: number;
}

export const computeRatedSkill = (ratedSkill: RatedSkill): ComputedRatedSkill => {
    const agreggatedScore = ratedSkill.history.reduce((acc, curr) => (acc + curr.score), 0);
    return {
        ...ratedSkill,
        agreggatedScore
    }
}

export const extractLastScore = (ratedSkill: RatedSkill, whoRated: Employee): SkillRateHistoryItem | undefined  => {
    return ratedSkill.history.reverse().find(item => item.whoRated === whoRated);
}

// -- extract by year