import json
import Globals
from DynamicConfigData import DATA_HERO, DATA_EQUIP_ATTR, DATA_EQUIP_INIT, DATA_EQUIP_RANDOM_ATTR, DATA_STORY
import com.utils.helpers as helpers
import com.const as CONST

f = open(r'\\.\pipe\b62340b3-9f87-4f38-b844-7b8d1598b64b', 'wb+', buffering=0)
try:
    player = Globals.player1
    if player == None:
        raise Exception('Player data not available.')

    heroTypeList = [
        CONST.HeroType.SS_MONSTER, CONST.HeroType.SS_GHOST, CONST.HeroType.SS_ELF]

    equip_attr_types = [
        'maxHpAdditionVal', 'defenseAdditionVal', 'attackAdditionVal', 'maxHpAdditionRate',
        'defenseAdditionRate', 'attackAdditionRate', 'speedAdditionVal', 'critRateAdditionVal', 'critPowerAdditionVal',
        'debuffEnhance', 'debuffResist']
    equip_attr_type_map = {value: key for (
        key, value) in enumerate(equip_attr_types)}

    def map_equip(id, equip):
        initData = DATA_EQUIP_INIT.data.get(equip.equipId)
        attrs = []
        tmpList = equip.getRandAttrDict()
        for key, value in tmpList.items():
            attrs.append([equip_attr_type_map[key], value])
        base_attr = [equip_attr_type_map[equip.baseAttrName],
                     equip.strengthenedBaseAttrValue]

        single_attrs = DATA_EQUIP_RANDOM_ATTR.data.get(
            equip.single_attr, {}).get('attrs') if equip.single_attr else []

        single_attrs = [[equip_attr_type_map[
            attr[0]], attr[1]] for attr in single_attrs]

        return [
            id,
            # 'name': equip.name,
            equip.suitId,
            equip.getEquipInit('quality'),
            equip.getPos(),
            equip.equipId,
            equip.strongLevel,
            equip.born,
            bool(equip.lock),
            bool(equip.garbage),
            # 'strengthened_base_attr_value': equip.strengthenedBaseAttrValue,
            # 'base_attr': equip.baseAttrDict,
            # 'attr': equip.attrDict,
            # 'random_attr': equip.randomAttrDict,
            # 'init_data': initData,
            # 'rand_attr_rates': equip.randAttrRates,
            attrs,
            base_attr,
            attrs,
            [],
            single_attrs
        ]

    def map_hero(id, hero):
        attr_calc = hero.getUnAwakeBattleAttr(
        ) if hero.awake == 0 else hero.getAwakeBattleAttr()
        return [
            id,
            # 'uid': hero.uid,
            hero.heroId,
            hero._equips,
            hero._level,
            hero.exp,
            # hero._name,
            hero.nickName,
            hero.born,
            bool(hero.lock),
            hero.rarity,
            hero.skillList,
            hero.awake,
            hero.star,
            [
                [
                    attr_calc.baseMaxHp,
                    attr_calc.maxHpAdditionVal,
                    attr_calc.maxHpAdditionRate,
                    attr_calc.maxHp,
                ],
                [
                    attr_calc.baseSpeed,
                    attr_calc.speedAdditionVal,
                    attr_calc.speedAdditionRate,
                    attr_calc.speed,
                ],
                [
                    attr_calc.baseCritPower,
                    attr_calc.critPowerAdditionVal,
                    attr_calc.critPowerAdditionRate,
                    attr_calc.critPower,
                ],
                [
                    attr_calc.baseCritRate,
                    attr_calc.critRateAdditionVal,
                    attr_calc.critRateAdditionRate,
                    attr_calc.critRate,
                ],
                [
                    attr_calc.baseDefense,
                    attr_calc.defenseAdditionVal,
                    attr_calc.defenseAdditionRate,
                    attr_calc.defense,
                ],
                [
                    attr_calc.baseAttack,
                    attr_calc.attackAdditionVal,
                    attr_calc.attackAdditionRate,
                    attr_calc.attack,
                ],
                attr_calc.debuffEnhance,
                attr_calc.debuffResist
            ]
        ]

    def get_item_presets():
        preset_items = helpers.getUserConfig('equipDrawer', [])
        preset_names = helpers.getUserConfig('equipDrawerName', [])
        presets = []
        for i, items in enumerate(preset_items):
            presets.append([preset_names[i], items])
        return presets

    def get_hero_shards():
        def map(id, data):
            book = data['book']
            return [
                id,  # hero_id
                player.currency.get(book[1], 0),  # shard_count
                player.currency.get(book[0], 0),  # book_count
                book[2]  # book_max_shard_count
            ]
        return [map(id, data) for id, data in DATA_HERO.data.items() if data['type'] in heroTypeList]

    def map_realm_card(id, card):
        return [
            id,
            card.itemid,
            card.totalTime,
            card.produceValue,
        ]

    def get_story_tasks():
        items = []
        for id, _ in DATA_HERO.data.iteritems():
            if id < 200 or id > 600:
                continue
            storyData = DATA_STORY.data.get(id)
            if storyData != None:
                ids = storyData.get('activityId')
                if ids != None:
                    for id in ids:
                        items.append([
                            id, Globals.jobMgr.getJobProg(id)
                        ])
        return items

    data = [
        [player.short_id, player.server_id, player.name, player.level],
        map(lambda v: int(v), [
            player.currency.get(CONST.CurrencyType.COIN, 0),  # COIN
            player.currency.get(CONST.CurrencyType.GOLD, 0),  # GOUYU
            player.currency.get(
                CONST.CurrencyType.STRENGTH, 0),  # STRENGTH
            player.currency.get(900273, 0),  # YINGBING
            player.currency.get(900012, 0),  # RONGYU
            player.currency.get(900016, 0),  # XUNZHANG
            player.currency.get(900090, 0),  # GONGXUN
            player.currency.get(900215, 0),  # YLJZS
            player.currency.get(900000, 0),  # HUNYU
            player.currency.get(900023, 0),  # PIFU
            player.currency.get(900024, 0),  # TUPO
            player.currency.get(490002, 0),  # BAIPIAO broken_amulet
            player.currency.get(490001, 0),  # LANPIAO mystery_amulet
            player.currency.get(490004, 0),  # XIANSHI ar_amulet
            player.currency.get(900178, 0),  # YUZHA ofuda
            player.currency.get(900188, 0),  # JINYUZHA gold_ofuda
            player.currency.get(900216, 0),  # 八岐大蛇鳞片 scale
            player.currency.get(900217, 0),  # 大蛇的逆鳞 reverse_scale
            player.currency.get(900218, 0),  # 逢魔之魂 demon_soul
            player.currency.get(900041, 0),  # 痴念之卷 foolery_pass
            player.currency.get(906058, 0)  # SP皮肤券
        ]),
        [map_hero(id, i) for id, i in player.heroes.items()
         if DATA_HERO.data.get(i.heroId).get('type') in heroTypeList],
        [map_equip(id, e) for id, e in player.inventory.items()],
        get_item_presets(),
        get_hero_shards(),
        [map_realm_card(id, data)
            for id, data in Globals.player1.myJiejieCardDataDict.items()],
        get_story_tasks()
    ]

    f.write(json.dumps(data, ensure_ascii=False).encode('utf8'))
except Exception as e:
    f.write(json.dumps({
        'error': str(e)
    }, ensure_ascii=False).encode('utf8'))
f.close()
