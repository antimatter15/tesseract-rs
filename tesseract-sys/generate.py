import re

tymap = {
    'const char*': '*const libc::c_char',
    'char*': '*const libc::c_char',
    'char**': '*const *const libc::c_char',
    'int*': '*const libc::c_int',
    'BOOL': 'libc::c_int',
    'const BOOL': 'libc::c_int',
    'BOOL*': '*const libc::c_int',
    'BOOL**': '*const *const libc::c_int',
    'int': 'libc::c_int',
    'const int': 'libc::c_int',
    'void': 'libc::c_void',
    'void**': '*const *const libc::c_void',
    'struct Pix*': '*const PIX',
    'double*': '*const libc::c_double',
    'float*': '*const libc::c_float',
    'float': 'libc::c_float',
    'double': 'libc::c_double',
    'FILE*': None,
    'STRING*': None,
    'const STRING*': None,
    'size_t': 'libc::size_t',
    'const unsigned char*': '*const libc::c_uchar',
    'struct Pixa**': '*const *const Pixa',
    'struct Boxa*': '*const Boxa',
    'int**': '*const *const libc::c_int',
    'TessDictFunc': None,
    'TessProbabilityInContextFunc': None,
    'TessFillLatticeFunc': None,
    'INT_FEATURE_STRUCT*': None,
    'ROW*': None,
    'const TessDawg*': None,
    'TessTruthCallback*': None,
    'TessCubeRecoContext*': None,
    'Pix*': '*const PIX'
}

for x in [ 'OSResults', 'TBLOB', 'ETEXT_DESC', "TessOcrEngineMode", "TessPageSegMode", "TessPageIteratorLevel", "TessPolyBlockType", "TessOrientation", "TessParagraphJustification", "TessWritingDirection", "TessTextlineOrder" ]:
    tymap[x] = x
    tymap['const '+x] = x
    tymap[x+'*'] = '*const ' + x
    
for x in ['BLOCK_LIST', 'TessChoiceIterator', 'TessResultRenderer', 'TessBaseAPI', 'TessImageThresholder', 'TessPageIterator',  'TessResultIterator', 'TessMutableIterator']:
    tymap[x + '*'] = '*mut ' + x
    tymap['const '+x+'*'] = '*const ' + x

def rustify(stuff):
    
    things = stuff.replace("*", "").strip().split(" ")
    merp = " ".join(things[0:-1]) + ("*" * stuff.count("*"))
    if tymap[merp] == None:
        return None
    return things[-1] + ": " + tymap[merp]

for x in re.finditer(r"(?s)TESS_API\s+(?P<type>.*?)TESS_CALL\s+(?P<name>.*?)\((?P<args>.*?)\);", open("capi.h").read()):
    ty = x.group("type").strip()
    name = x.group("name").strip()
    args = [rustify(k)  for k in x.group("args").strip().split(",") if k != ""]
    if any([k == None for k in args]):
        continue
    hdr = "pub fn " + name + "("
    fd = hdr + ", ".join(args) + ")"
    if len(fd) > 70:
        fd = hdr + (",\n" + " "  * len(hdr)).join(args) + ")"
    if ty == "void":
        fd += ";"
    elif tymap[ty] == None:
        continue
    else:
        fd += " -> " + tymap[ty] + ";"
    print fd
    