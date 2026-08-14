import json
import sys
import traceback

PIPE = r'\\.\pipe\b62340b3-9f87-4f38-b844-7b8d1598b64b'
f = open(PIPE, 'wb+', buffering=0)

try:
    import Globals
    from DynamicConfigData import (
        DATA_HERO, DATA_EQUIP_ATTR, DATA_EQUIP_INIT,
        DATA_EQUIP_RANDOM_ATTR, DATA_STORY)
    import com.utils.helpers as helpers
    import com.const as CONST

    player = Globals.player1
    if player is None:
        raise Exception('Player data not available.')

    heroTypeList = [
        CONST.HeroType.SS_MONSTER, CONST.HeroType.SS_GHOST, CONST.HeroType.SS_ELF]

    equip_attr_types = [
        'maxHpAdditionVal', 'defenseAdditionVal', 'attackAdditionVal',
        'maxHpAdditionRate', 'defenseAdditionRate', 'attackAdditionRate',
        'speedAdditionVal', 'critRateAdditionVal', 'critPowerAdditionVal',
        'debuffEnhance', 'debuffResist']
    equip_attr_type_map = {value: key for (
        key, value) in enumerate(equip_attr_types)}

    def map_equip(id, equip):
        try:
            initData = DATA_EQUIP_INIT.data.get(equip.equipId)
        except Exception:
            initData = None
        attrs = []
        try:
            tmpList = equip.getRandAttrDict()
            for key, value in tmpList.items():
                attrs.append([equip_attr_type_map.get(key, -1), value])
        except Exception:
            pass
        base_attr = []
        try:
            base_attr = [equip_attr_type_map.get(equip.baseAttrName, -1),
                         equip.strengthenedBaseAttrValue]
        except Exception:
            pass

        single_attrs = []
        try:
            if equip.single_attr:
                single_attrs = DATA_EQUIP_RANDOM_ATTR.data.get(
                    equip.single_attr, {}).get('attrs') or []
                single_attrs = [[equip_attr_type_map.get(
                    attr[0], -1), attr[1]] for attr in single_attrs]
        except Exception:
            pass

        try:
            strong_level = equip.strongLevel
        except Exception:
            strong_level = 0
        try:
            suit_id = equip.suitId
        except Exception:
            suit_id = 0
        try:
            pos = int(equip.getPos())
        except Exception:
            pos = 0
        try:
            qua = int(equip.getEquipInit('quality'))
        except Exception:
            qua = 0
        try:
            born = equip.born
        except Exception:
            born = 0
        try:
            lock = bool(equip.lock)
        except Exception:
            lock = False
        try:
            garbage = bool(equip.garbage)
        except Exception:
            garbage = False

        return [
            id,
            suit_id,
            qua,
            pos,
            equip.equipId,
            strong_level,
            born,
            lock,
            garbage,
            attrs,
            base_attr,
            attrs,
            [],
            single_attrs
        ]

    def map_hero(id, hero):
        try:
            attr_calc = (hero.getUnAwakeBattleAttr()
                         if hero.awake == 0 else hero.getAwakeBattleAttr())
        except Exception:
            attr_calc = None

        def g(name, default=0):
            if attr_calc is None:
                return default
            try:
                return getattr(attr_calc, name, default)
            except Exception:
                return default

        try:
            equips = hero._equips
            if equips is None:
                equips = []
        except Exception:
            equips = []
        try:
            skill_list = [[s[0], s[1]] for s in hero.skillList]
        except Exception:
            skill_list = []

        return [
            id,
            hero.heroId,
            equips,
            hero._level,
            hero.exp,
            hero.nickName,
            hero.born,
            bool(hero.lock),
            hero.rarity,
            skill_list,
            hero.awake,
            hero.star,
            [
                [g('baseMaxHp'), g('maxHpAdditionVal'), g('maxHpAdditionRate'), g('maxHp')],
                [g('baseSpeed'), g('speedAdditionVal'), g('speedAdditionRate'), g('speed')],
                [g('baseCritPower'), g('critPowerAdditionVal'), g('critPowerAdditionRate'), g('critPower')],
                [g('baseCritRate'), g('critRateAdditionVal'), g('critRateAdditionRate'), g('critRate')],
                [g('baseDefense'), g('defenseAdditionVal'), g('defenseAdditionRate'), g('defense')],
                [g('baseAttack'), g('attackAdditionVal'), g('attackAdditionRate'), g('attack')],
                g('debuffEnhance'),
                g('debuffResist')
            ]
        ]

    def get_item_presets():
        try:
            preset_items = helpers.getUserConfig('equipDrawer', [])
            preset_names = helpers.getUserConfig('equipDrawerName', [])
            return [[preset_names[i], items]
                    for i, items in enumerate(preset_items)]
        except Exception:
            return []

    def get_hero_shards():
        def map_shard(id, data):
            try:
                book = data['book']
                return [
                    id,
                    player.currency.get(book[1], 0),  # shard_count
                    player.currency.get(book[0], 0),  # book_count
                    book[2]  # book_max_shard_count
                ]
            except Exception:
                return [id, 0, 0, 0]
        try:
            return [map_shard(id, data)
                    for id, data in DATA_HERO.data.items()
                    if data.get('type') in heroTypeList]
        except Exception:
            return []

    def map_realm_card(id, card):
        return [
            id,
            card.itemid,
            card.totalTime,
            card.produceValue,
        ]

    def get_story_tasks():
        items = []
        try:
            for id, _ in DATA_HERO.data.items():
                if id < 200 or id > 600:
                    continue
                storyData = DATA_STORY.data.get(id)
                if storyData is not None:
                    ids = storyData.get('activityId')
                    if ids is not None:
                        for aid in ids:
                            try:
                                items.append([
                                    aid, Globals.jobMgr.getJobProg(aid)
                                ])
                            except Exception:
                                pass
        except Exception:
            pass
        return items

    # currency: 21 项 (coin/jade/action_point/auto_point/honor/medal/contrib/
    #   totem_pass/s_jade/skin_token/realm_raid_pass/broken_amulet/
    #   mystery_amulet/ar_amulet/ofuda/gold_ofuda/scale/reverse_scale/
    #   demon_soul/foolery_pass/sp_skin_token)
    currency_ids = [
        CONST.CurrencyType.COIN,        # coin 金币
        CONST.CurrencyType.GOLD,        # jade 勾玉
        CONST.CurrencyType.STRENGTH,    # action_point 体力
        900273,                          # auto_point 樱饼
        900012,                          # honor 荣誉
        900016,                          # medal 勋章
        900090,                          # contrib 功勋
        900215,                          # totem_pass
        900000,                          # s_jade 魂玉
        900023,                          # skin_token 皮肤券
        900024,                          # realm_raid_pass 突破券
        490002,                          # broken_amulet 破碎符
        490001,                          # mystery_amulet 蓝票
        490004,                          # ar_amulet 现世符
        900178,                          # ofuda 御札
        900188,                          # gold_ofuda 金御札
        900216,                          # scale 八岐大蛇鳞片
        900217,                          # reverse_scale 逆鳞
        900218,                          # demon_soul 逢魔之魂
        900041,                          # foolery_pass 痴念之卷
        906058,                          # sp_skin_token SP皮肤券
    ]

    data = [
        [player.short_id, player.server_id, player.name, player.level],
        [int(player.currency.get(cid, 0)) for cid in currency_ids],
        [map_hero(id, i) for id, i in player.heroes.items()
         if DATA_HERO.data.get(i.heroId, {}).get('type') in heroTypeList],
        [map_equip(id, e) for id, e in player.inventory.items()],
        get_item_presets(),
        get_hero_shards(),
        [map_realm_card(id, data)
         for id, data in player.myJiejieCardDataDict.items()],
        get_story_tasks()
    ]

    f.write(json.dumps(data, ensure_ascii=False).encode('utf8'))
except Exception as e:
    f.write(json.dumps({
        'error': str(e),
        'tb': traceback.format_exc()
    }, ensure_ascii=False).encode('utf8'))
f.close()
