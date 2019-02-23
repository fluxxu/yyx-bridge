import json
import Globals
from DynamicConfigData import DATA_HERO, DATA_EQUIP_ATTR, DATA_EQUIP_INIT, DATA_EQUIP_RANDOM_ATTR
import com.utils.helpers as helpers

f = open(r'\\.\pipe\b62340b3-9f87-4f38-b844-7b8d1598b64b', 'wb+', buffering=0)
try:
    def map(key, value):
        book = value['book']
        return {
            'type': value['type'],
            'book': value['book'],
            'b1': Globals.player1.currency.get(book[0], 0),
            'b2': Globals.player1.currency.get(book[1], 0)
        }
    data = [map(k, v) for k, v in DATA_HERO.data.items()]
    f.write(json.dumps(data, ensure_ascii=False).encode('utf8'))
except Exception as e:
    f.write(json.dumps({
        'error': str(e)
    }, ensure_ascii=False).encode('utf8'))
f.close()
