import struct, sys, os, glob, json

def tags(b):
    n = struct.unpack('>I', b[128:132])[0]
    out = {}
    for i in range(n):
        off = 132 + i*12
        sig, o, s = struct.unpack('>4sII', b[off:off+12])
        out[sig.decode('latin1')] = (o, s)
    return out

def text(b, o, s):
    t = b[o:o+4]
    d = b[o:o+s]
    if t == b'text':      # v2 textType
        return d[8:].split(b'\x00')[0].decode('latin1')
    if t == b'desc':      # v2 textDescriptionType
        ln = struct.unpack('>I', d[8:12])[0]
        return d[12:12+ln].split(b'\x00')[0].decode('latin1')
    if t == b'mluc':      # v4 multiLocalizedUnicodeType
        cnt, rs = struct.unpack('>II', d[8:16])
        if cnt == 0: return ''
        _, _, ln, off = struct.unpack('>2s2sII', d[16:28])
        return d[off:off+ln].decode('utf-16-be')
    return '<%s>' % t.decode('latin1', 'replace')

for p in sorted(sys.argv[1:]):
    b = open(p, 'rb').read()
    ver = struct.unpack('>I', b[8:12])[0]
    cls = b[12:16].decode('latin1'); space = b[16:20].decode('latin1'); pcs = b[20:24].decode('latin1')
    creator = b[48:52].decode('latin1')
    try: tg = tags(b)
    except Exception as e:
        print('%-42s PARSE-FAIL %s' % (os.path.basename(p), e)); continue
    def g(k):
        return text(b, *tg[k]).strip() if k in tg else ''
    print(json.dumps({
        'file': os.path.basename(p), 'size': len(b),
        'version': '0x%08X' % ver, 'class': cls, 'space': space, 'pcs': pcs,
        'platform': b[40:44].decode('latin1'), 'creator': creator,
        'desc': g('desc')[:120], 'cprt': g('cprt')[:400], 'dmdd': g('dmdd')[:120],
        'ntags': len(tg), 'tags': sorted(tg.keys()),
    }))
