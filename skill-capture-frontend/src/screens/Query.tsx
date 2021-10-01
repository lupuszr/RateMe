import { Pane, Select, Text } from 'evergreen-ui';
import React, { useEffect } from 'react';
import { Skill } from '../models/Skills';
import { listSkills } from '../providers/SkillProvider';

export function Query() {
    
    const [selectedSkill, setSelectedSkill] = React.useState<string | undefined>(undefined)
    const [skills, setSkills] = React.useState<Array<Skill>>([])
    const [filter, setFilter] = React.useState('')
    const eff1 = useEffect(() => {
        listSkills().then(x => setSkills(x));
    }, [])


    return (
      <Pane>
        <Pane marginBottom={8}>
          <Text>Filter value: {filter}</Text>
        </Pane>
        <Select value={selectedSkill} onChange={event => setSelectedSkill(event.target.value)}>
        {skills.map(skill => (
            <option value={skill.name} selected>
                {skill.name}
            </option>
        ))}
        </Select>
      </Pane>
    )
  }