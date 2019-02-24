import json
import Globals
from DynamicConfigData import DATA_HERO, DATA_EQUIP_ATTR, DATA_EQUIP_INIT, DATA_EQUIP_RANDOM_ATTR
import com.utils.helpers as helpers
import com.const as CONST

f = open(r'\\.\pipe\b62340b3-9f87-4f38-b844-7b8d1598b64b', 'wb+', buffering=0)
try:
    if Globals.player1 == None:
        raise Exception('Player data not available.')

    equip_attr_types = [
        'maxHpAdditionVal', 'defenseAdditionVal', 'attackAdditionVal', 'maxHpAdditionRate',
        'defenseAdditionRate', 'attackAdditionRate', 'speedAdditionVal', 'critRateAdditionVal', 'critPowerAdditionVal',
        'debuffEnhance', 'debuffResist']
    equip_attr_type_map = {value: key for (
        key, value) in enumerate(equip_attr_types)}

    def map_equip(id, equip):
        initData = DATA_EQUIP_INIT.data.get(equip.equipId)
        randData = DATA_EQUIP_ATTR.data.get(initData['rand_id'])
        attrs = []
        random_attrs = []
        base_attr = [equip_attr_type_map[equip.baseAttrName],
                     equip.strengthenedBaseAttrValue]
        for item in randData['attr_list']:
            attrname = item[0]
            attrvalue = item[1]
            if attrname in equip.randAttrRates:
                attrs.append([equip_attr_type_map[attrname],
                              attrvalue * equip.randAttrRates[attrname]])
                random_attrs.append(
                    [equip_attr_type_map[attrname], attrvalue * equip.randAttrRates[attrname]])

        return [
            id,
            # 'name': equip.name,
            equip.suitId,
            equip.getEquipInit('quality'),
            equip.getPos(),
            equip.equipId,
            equip.strongLevel,
            equip.born,
            equip.lock,
            equip.garbage,
            # 'strengthened_base_attr_value': equip.strengthenedBaseAttrValue,
            # 'base_attr': equip.baseAttrDict,
            # 'attr': equip.attrDict,
            # 'random_attr': equip.randomAttrDict,
            # 'init_data': initData,
            # 'rand_attr_rates': equip.randAttrRates,
            attrs,
            base_attr,
            random_attrs,
            [[equip_attr_type_map[attrname], rate]
                for (attrname, rate) in equip.randAttrRates.items()],
        ]

    def map_hero(id, hero):
        return [
            id,
            # 'uid': hero.uid,
            hero.heroId,
            hero._equips,
            hero._level,
            hero.exp,
            hero._expRate,
            # hero._name,
            hero.nickName,
            hero.born,
            hero.lock,
            hero.rarity,
            hero.skillList
        ]

    def get_item_presets():
        preset_items = helpers.getUserConfig('equipDrawer', [])
        preset_names = helpers.getUserConfig('equipDrawerName', [])
        presets = []
        for i, items in enumerate(preset_items):
            presets.append([preset_names[i], items])
        return presets

    def get_hero_fragments():
        heroTypeList = [
            CONST.HeroType.SS_MONSTER, CONST.HeroType.SS_GHOST, CONST.HeroType.SS_ELF]

        def map(id, data):
            book = data['book']
            return [
                id,  # hero_id
                Globals.player1.currency.get(book[1], 0),  # fragment_count
                Globals.player1.currency.get(book[0], 0),  # book_count
                book[2]  # book_fragment_count
            ]
        return [map(id, data) for id, data in DATA_HERO.data.items() if data['type'] in heroTypeList]

    data = [
        [map_hero(id, e) for id, e in Globals.player1.heroes.items()],
        [map_equip(id, e) for id, e in Globals.player1.inventory.items()],
        get_item_presets(),
        get_hero_fragments(),
        # [
        #     Globals.player1.currency.get(CONST.CurrencyType.COIN),  # COIN
        #     Globals.player1.currency.get(CONST.CurrencyType.GOLD),  # GOUYU
        #     Globals.player1.currency.get(
        #         CONST.CurrencyType.STRENGTH),  # STRENGTH
        #     Globals.player1.currency.get(CONST.CurrencyType.RONGYU),
        #     Globals.player1.currency.get(CONST.CurrencyType.PVPJIFEN),
        # ]
    ]

    f.write(json.dumps(data, ensure_ascii=False).encode('utf8'))
except Exception as e:
    f.write(json.dumps({
        'error': str(e)
    }, ensure_ascii=False).encode('utf8'))
f.close()
