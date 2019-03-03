import json
import Globals
from DynamicConfigData import DATA_HERO, DATA_EQUIP_ATTR, DATA_EQUIP_INIT, DATA_EQUIP_RANDOM_ATTR
import com.utils.helpers as helpers

f = open(r'\\.\pipe\b62340b3-9f87-4f38-b844-7b8d1598b64b', 'wb+', buffering=0)
try:
    from com.const import AttrDesc
    import types

    if getattr(AttrDesc, "__format", None) == None:
        AttrDesc.__format = AttrDesc.format

    def format(self, value):
        return '%.4f' % value

    AttrDesc.format = types.MethodType(format, AttrDesc)

    data = repr(AttrDesc.format)
    f.write(json.dumps(data, ensure_ascii=False).encode('utf8'))
except Exception as e:
    f.write(json.dumps({
        'error': str(e)
    }, ensure_ascii=False).encode('utf8'))
f.close()
