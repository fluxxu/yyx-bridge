import json
import Globals
import module.guild.GuildLogic as GuildLogic
import com.const as CONST
import com.const as const
import com.utils.helpers as helpers


def get_task_records():
    panel = Globals.uiMgr.getUI('GuildTaskRecordPanel')
    if panel == None:
        return {}
    records = panel.records
    if records == None:
        return {}
    return {
        str(v.get('id')): v for v in records
    }


def map_guild(g):
    keys = [
        "_id",
        "creator_id",
        "short_id",
        "active_rank_pos",
        "pvp_rank_pos",
        "active_member_count",
        "funds",
        "guild_badge",
        "create_time",
        "guild_lv",
        "pvp_score",
    ]
    data = [
        str(g.get(k)) if k == '_id' or k == 'creator_id' else g.get(k) for k in keys
    ]
    task_records = get_task_records()
    members = []
    for m in GuildLogic.myGuildDetail['members']:
        idstr = str(m.get('id'))
        record = task_records.get(idstr)
        if record != None:
            m['_task_week_finished_times'] = int(
                record.get('week_finished_times') or 0)
            m['_task_day_finished_times'] = int(
                record.get('day_finished_times') or 0)
        else:
            m['_task_week_finished_times'] = -1
            m['_task_day_finished_times'] = -1

        mapped = map_guild_member(m)
        members.append(mapped)
    data.append(members)
    return data


def map_guild_member(m):
    keys = [
        "id",
        "duty",
        "donate_times",
        "last_login_time",
        "join_time",
        "offline_time",
        "weekly_feats",
        "history_donate",
        "nickname",
        "dg_times",
        "name",
        "level",
        "receive_times",
        "total_feats",
        "pvp_score",
        # "trial_emblem_score",
        # "season_pvp_score",
        # "buff_cd",
        # "honors",
        # "push_switch",
        # "head_id",
        # "yys_skin",
        # "def_records_all",
        # "icon",
        # "record_dt",
        # "yysid",
        "_task_day_finished_times",
        "_task_week_finished_times"
    ]
    return [
        m.get(k) if k != 'id' else str(m.get(k)) for k in keys
    ]


# f = open(r'\\.\pipe\b62340b3-9f87-4f38-b844-7b8d1598b64b', 'wb+', buffering=0)
f = open(r'c:\Data\records.json', 'wb+', buffering=0)
try:
    if GuildLogic.myGuildDetail == None:
        raise Exception('Guild data not available.')

    data = map_guild(GuildLogic.myGuildDetail)
    # data = str(get_task_records())
    f.write(json.dumps(data, ensure_ascii=False).encode('utf8'))
except Exception as e:
    f.write(json.dumps({
        'error': str(e)
    }, ensure_ascii=False).encode('utf8'))
f.close()
