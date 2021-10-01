-- Your SQL goes here
CREATE TABLE EmployeeSkill (
  skill_id integer NOT NULL,
  employee_id integer NOT NULL,
  history json,
  CONSTRAINT PK_Employee_Skill_employeeid_skillid PRIMARY KEY (employee_id, skill_id),
  CONSTRAINT FK_Employee_employeeid FOREIGN KEY (employee_id) REFERENCES Employee,
  CONSTRAINT FK_Skill_skillid FOREIGN KEY (skill_id) REFERENCES Skill
)