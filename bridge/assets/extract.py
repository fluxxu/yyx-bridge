import json
import Globals
from DynamicConfigData import DATA_HERO, DATA_EQUIP_ATTR, DATA_EQUIP_INIT, DATA_EQUIP_RANDOM_ATTR, DATA_SKILL
import com.utils.helpers as helpers
import com.const as CONST
import module.utils.TipUtil as TipUtil
import module.UiUtil as UiUtil

f = open(r'\\.\pipe\b62340b3-9f87-4f38-b844-7b8d1598b64b', 'wb+', buffering=0)
try:
    path = UiUtil.getSkillIconPath(
        DATA_SKILL.getSkillData(5151, 1, True)['skillIcon'])
    buf = C_file.get_res_file(path, '')
    fobj = open(r'c:\Data\\text.png', 'wb')
    fobj.write(buf)
    fobj.close()
    data = 'ok'
    f.write(json.dumps(data, ensure_ascii=False).encode('utf8'))
except Exception as e:
    f.write(json.dumps({
        'error': str(e)
    }, ensure_ascii=False).encode('utf8'))
f.close()
