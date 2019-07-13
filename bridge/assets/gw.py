import json
import Globals
from DynamicConfigData import DATA_HERO, DATA_EQUIP_ATTR, DATA_EQUIP_INIT, DATA_EQUIP_RANDOM_ATTR, DATA_SKILL
import com.utils.helpers as helpers
import com.const as CONST
import module.utils.TipUtil as TipUtil
import module.UiUtil as UiUtil
import com.data.cdata.GhostWalkFair as GhostWalkFairData
from scenemembers.GhostWalkScene import GhostWalkScene, GhostWalkFairPanel
from DynamicConfigData import DATA_HERO

# def ThrowBeanCD(self, isCD):
#     if isCD:
#         self.IsThrowingBean = True
#         Globals.delayExec(800.0 / self.ThrowBeanSpeedFactor, lambda : self.ThrowBeanCD(False))
#     else:
#         self.IsThrowingBean = False

# GhostWalkScene.ThrowBeanCD = ThrowBeanCD

if getattr(GhostWalkScene, "__FireBean", None) == None:
    GhostWalkScene.__FireBean = GhostWalkScene.FireBean


def FireBean(self, offsetX, offsetY):
    Globals.currGameScene.FairModelSpeedFactor = 0
    Globals.currGameScene.ThrowBeanSpeedFactor = 10.0
    return self.__FireBean(offsetX, offsetY)


GhostWalkScene.FireBean = FireBean

Globals.currGameScene.ThrowBeanSpeedFactor = 10.0

if Globals.currGameScene.FairModelSpeedFactor > 0:
    Globals.currGameScene.FairModelSpeedFactor = 0
else:
    Globals.currGameScene.FairModelSpeedFactor = 5

if getattr(Globals.currGameScene, "__ok", None) == None:
    Globals.currGameScene.__ok = True
    f = open(r'\\.\pipe\b62340b3-9f87-4f38-b844-7b8d1598b64b', 'wb+', buffering=0)
    try:
        # f.write(str(Globals.currentScene.get_models()))
        # import C_file
        # path = 'icon/head/j_buzhihuo.png'
        # buf = C_file.get_res_file(path, '')
        # fobj = open(r'c:\Data\\j_buzhihuo.png', 'wb')
        # fobj.write(buf)
        # fobj.close()
        # data = path
        # f.write(json.dumps(data, ensure_ascii=False).encode('utf8'))
        heros = []
        for item in Globals.currGameScene.FairGhostInfoList:
            r = DATA_HERO.data[item[0]]['rarity']
            name = DATA_HERO.data[item[0]]['name']
            if r >= 4:
                heros.append('!!!!!!!!!!' + name + '!!!!!!!!!!')
            elif item[0] >= 320 and item[0] < 400:
                heros.append('**********' + name + '**********')
            else:
                heros.append(name)
        # f.write(json.dumps(heros, ensure_ascii=False).encode('utf8'))
        f.write('\n'.join(heros))
    except Exception as e:
        f.write(json.dumps({
            'error': str(e)
        }, ensure_ascii=False).encode('utf8'))
    f.close()
