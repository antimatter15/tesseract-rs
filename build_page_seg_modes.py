import sys
from itertools import islice

class PageSegMode:
    def __init__(self, name, comment):
        self.name = name
        self.comment = comment
    
    def name_as_enum(self):
        return ''.join(n.capitalize() for n in self.name.split("_"))
    
    def name_as_variable(self):
        return 'tesseract_sys::TessPageSegMode_' + self.name

name = None
page_seg_modes = []
i = 0;

for line in islice(sys.stdin, 1, None):
    if i == 0:
        name = line.rstrip('\n')
    elif i == 1:
        comment = line.rstrip('\n')
        page_seg_mode = PageSegMode(name, comment)
        page_seg_modes.append(page_seg_mode)
    i = (i + 1) % 3

print("// ⚠️ This file is generated")
print("// ⚠️ Regenerate with `make src/page_seg_mode.rs`")
print()
print("use tesseract_sys::TessPageSegMode;")
print()
print("/// Enum representing different PageSegMode options accepted by Tesseract")
print("#[derive(Debug, Clone, Copy, PartialEq, Eq)]")
print("pub enum PageSegMode {")

for page_seg_mode in page_seg_modes:
    print(f"    /// {page_seg_mode.comment}")
    print(f"    {page_seg_mode.name_as_enum()},",)

print("}")
print()
print("impl PageSegMode {")
print("    /// Get the page-seg-mode's value as used by Tesseract")
print("    pub fn as_tess_page_seg_mode(&self) -> TessPageSegMode {")
print("        match self {")

for page_seg_mode in page_seg_modes:
    print(f"            PageSegMode::{page_seg_mode.name_as_enum()} => {page_seg_mode.name_as_variable()},")

print("        }")
print("    }")
print("}")