import json
import Globals
from DynamicConfigData import DATA_HERO, DATA_EQUIP_ATTR, DATA_EQUIP_INIT, DATA_EQUIP_RANDOM_ATTR
import com.utils.helpers as helpers
import com.const as CONST

f = open(r'\\.\pipe\b62340b3-9f87-4f38-b844-7b8d1598b64b', 'wb+', buffering=0)
try:
    if Globals.player1 == None:
        raise Exception('Player data not available.')

    def map_equip(id, equip):
        initData = DATA_EQUIP_INIT.data.get(equip.equipId)
        return {
            'id': id,
            'name': equip.name,
            'quality': equip.getEquipInit('quality'),
            'equip_id': equip.equipId,
            'suit_id': equip.suitId,
            'strong_level': equip.strongLevel,
            'born': equip.born,
            'lock': equip.lock,
            'garbage': equip.garbage,
            'strengthened_base_attr_value': equip.strengthenedBaseAttrValue,
            'base_attr': equip.baseAttrDict,
            'attr': equip.attrDict,
            'random_attr': equip.randomAttrDict,
            'init_data': initData,
            'rand_attr_rates': equip.randAttrRates
        }

    def map_hero(id, hero):
        return {
            'id': id,
            'uid': hero.uid,
            'hero_id': hero.heroId,
            'equips': hero._equips,
            'level': hero._level,
            'exp': hero.exp,
            'exp_rate': hero._expRate,
            'name': hero._name,
            'nick_name': hero.nickName,
            'born': hero.born,
            'lock': hero.lock,
            'rarity': hero.rarity,
            'skill_list': hero.skillList
        }

    def get_item_presets():
        return {
            'drawers': helpers.getUserConfig('equipDrawer', []),
            'names': helpers.getUserConfig('equipDrawerName', []),
            # hero uid list
            'tops': helpers.getUserConfig('equipDrawerTop', [])
        }

    def get_hero_fragments():
        heroTypeList = [
            CONST.HeroType.SS_MONSTER, CONST.HeroType.SS_GHOST, CONST.HeroType.SS_ELF]

        def map(id, data):
            book = data['book']
            return {
                'hero_id': id,
                'fragment_count': Globals.player1.currency.get(book[1], 0),
                'book_count': Globals.player1.currency.get(book[0], 0),
                'book_fragment_count': book[2]
            }
        return [map(id, data) for id, data in DATA_HERO.data.items() if data['type'] in heroTypeList]

    data = {
        "items":  [map_equip(id, e) for id, e in Globals.player1.inventory.items()],
        "item_presets": get_item_presets(),
        "heroes":  [map_hero(id, e) for id, e in Globals.player1.heroes.items()],
        "hero_fragments": get_hero_fragments(),
        "currency": Globals.player1.currency
    }
    f.write(json.dumps(data, ensure_ascii=False).encode('utf8'))
except Exception as e:
    f.write(json.dumps({
        'error': str(e)
    }, ensure_ascii=False).encode('utf8'))
f.close()
