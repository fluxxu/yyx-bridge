import json
import sys
import Globals

f = open(r'\\.\pipe\b62340b3-9f87-4f38-b844-7b8d1598b64b', 'wb+', buffering=0)
try:
    if Globals.player1 == None:
        raise Exception('Player data not available.')

    def map_equip(id, equip):
        return {
            'id': id,
            'quality': equip.getEquipInit('quality'),
            'equipId': equip.equipId,
            'suitId': equip.suitId,
            'strongLevel': equip.strongLevel,
            'born': equip.born,
            'lock': equip.lock,
            'garbage': equip.garbage,
            'strengthenedBaseAttrValue': equip.strengthenedBaseAttrValue,
            'baseAttr': equip.baseAttrDict,
            'attr': equip.attrDict,
            'randomAttr': equip.randomAttrDict,
        }
    list = [map_equip(id, e) for id, e in Globals.player1.inventory.items()]
    f.write(json.dumps(list, ensure_ascii=False).encode('utf8'))
except Exception as e:
    f.write(json.dumps({
        'error': str(e)
    }, ensure_ascii=False).encode('utf8'))
f.close()
