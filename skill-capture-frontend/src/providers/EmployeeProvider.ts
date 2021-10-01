import { mkEmployee } from './../models/Employee';
export const listEmployees = () => {
    const res = Promise.resolve([
        { first_name: "Goran",
          last_name: "Blazin",
            title: "Rust Guru",
            created_at: "2021-10-01T10:30:52.953Z",
            updated_at: "2021-10-01T10:30:52.953Z"
        },
        { first_name: "Vasil",
          last_name: "Manilov",
            title: "The production guy",
            created_at: "2021-10-01T10:30:52.953Z",
            updated_at: "2021-10-01T10:30:52.953Z"
        },
        { first_name: "Viktor",
          last_name: "Pele",
            title: "The type guy",
            created_at: "2021-10-01T10:30:52.953Z",
            updated_at: "2021-10-01T10:30:52.953Z"
        },

    ])
    

    return res.then(xs => xs.map(x => mkEmployee(x.first_name, x.last_name, x.title, x.created_at, x.updated_at)))

}