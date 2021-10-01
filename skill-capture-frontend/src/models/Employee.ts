export type Employee = {
    first_name: string;
    last_name: string;
    title: string;
    created_at: string;
    updated_at: string;
}

type mkEmployeeT = (
    first_name: string,
    last_name: string,
    title: string,
    created_at: string,
    updated_at: string) => Employee;
export const mkEmployee: mkEmployeeT = (first_name, last_name, title, created_at, updated_at) => ({
    first_name,
    last_name,
    title,
    created_at,
    updated_at,
});