import json
import Globals
from DynamicConfigData import DATA_HERO, DATA_EQUIP_ATTR, DATA_EQUIP_INIT, DATA_EQUIP_RANDOM_ATTR, DATA_SKILL
import com.utils.helpers as helpers
import com.const as CONST

# f = open(r'\\.\pipe\b62340b3-9f87-4f38-b844-7b8d1598b64b', 'wb+', buffering=0)
f = open(r'c:\Data\SkillLevels.json', 'w')
try:
    skills = []
    for id, data in DATA_HERO.data.items():
        for id in data.get('skill'):
            if id != 0:
                skills.append([id, DATA_SKILL.getSkillTotalLevel(
                    id, False), DATA_SKILL.getSkillTotalLevel(id, True)])
        if data.get('awakeSkill'):
          id = data.get('awakeSkill')
          skills.append([id, DATA_SKILL.getSkillTotalLevel(
                    id, False), DATA_SKILL.getSkillTotalLevel(id, True)])

    f.write(json.dumps(skills, ensure_ascii=False).encode('utf8'))
except Exception as e:
    f.write(json.dumps({
        'error': str(e)
    }, ensure_ascii=False).encode('utf8'))
f.close()
