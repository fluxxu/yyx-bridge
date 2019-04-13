import json
import module.guild.GuildLogic as GuildLogic


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
    data.append([map_guild_member(m)
                 for m in GuildLogic.myGuildDetail['members']])
    return data


def map_guild_member(m):
    return [
        str(m.get('id')),
        m.get("duty"),
        m.get("donate_times") or 0,
        int(m.get("last_login_time") or 0),
        int(m.get("join_time") or 0),
        int(m.get("offline_time") or 0),
        m.get("weekly_feats") or 0,
        m.get("history_donate") or 0,
        m.get("nickname") or '',
        m.get("dg_times") or 0,
        m.get("name"),
        m.get("level"),
        m.get("receive_times") or 0,
        m.get("total_feats") or 0,
        m.get("pvp_score") or 0,
    ]


f = open(r'\\.\pipe\b62340b3-9f87-4f38-b844-7b8d1598b64b', 'wb+', buffering=0)
try:
    if GuildLogic.myGuildDetail == None:
        raise Exception('Guild data not available.')

    data = map_guild(GuildLogic.myGuildDetail)
    f.write(json.dumps(data, ensure_ascii=False).encode('utf8'))
except Exception as e:
    f.write(json.dumps({
        'error': str(e)
    }, ensure_ascii=False).encode('utf8'))
f.close()
