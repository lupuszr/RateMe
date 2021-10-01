import { Pane, SelectMenu, Text } from 'evergreen-ui';
import React, { useEffect } from 'react';
import { Button } from 'rebass';
import { Employee } from '../models/Employee';
import { listEmployees } from '../providers/EmployeeProvider';

export function ScoreUser() {
    
    const [selectedEmployee, setSelectedEmployee] = React.useState<string | undefined>(undefined)
    const [employees, setEmployees] = React.useState<Array<Employee>>([])
    const [filter, setFilter] = React.useState('')
    const eff1 = useEffect(() => {
        listEmployees().then(setEmployees);
    }, [])

    const eff2 = useEffect(() => {
        console.log("jebo ", selectedEmployee);
    }, [selectedEmployee])

    return (
      <Pane>
        <Pane marginBottom={8}>
          <Text>Filter value: {filter}</Text>
        </Pane>
        <SelectMenu
          title="Select name"
          onFilterChange={(filter) => setFilter(filter)}
          options={employees.map((employee) => ({ label: employee.last_name + " " + employee.first_name, value: employee.last_name + " " + employee.first_name}))}
          selected={selectedEmployee}
          onSelect={(item) => setSelectedEmployee(item.value + '')}
        >
          <Button>{selectedEmployee || 'Select user'}</Button>
        </SelectMenu>
      </Pane>
    )
  }