extern crate libc;
use libc::{c_int, c_char};

#[link(name = "lept")]
extern {
    pub fn pixRead(filename: *const libc::c_char) -> *mut PIX;
}

#[link(name = "tesseract")]
extern {
    pub fn TessVersion() -> *const libc::c_char;
    pub fn TessDeleteText(text: *const libc::c_char);
    pub fn TessDeleteTextArray(arr: *const *const libc::c_char);
    pub fn TessDeleteIntArray(arr: *const libc::c_int);
    pub fn TessDeleteBlockList(block_list: *mut BLOCK_LIST);
    pub fn TessTextRendererCreate(outputbase: *const libc::c_char) -> *mut TessResultRenderer;
    pub fn TessHOcrRendererCreate(outputbase: *const libc::c_char) -> *mut TessResultRenderer;
    pub fn TessHOcrRendererCreate2(outputbase: *const libc::c_char,
                                   font_info: libc::c_int) -> *mut TessResultRenderer;
    pub fn TessPDFRendererCreate(outputbase: *const libc::c_char,
                                 datadir: *const libc::c_char) -> *mut TessResultRenderer;
    pub fn TessUnlvRendererCreate(outputbase: *const libc::c_char) -> *mut TessResultRenderer;
    pub fn TessBoxTextRendererCreate(outputbase: *const libc::c_char) -> *mut TessResultRenderer;
    pub fn TessDeleteResultRenderer(renderer: *mut TessResultRenderer);
    pub fn TessResultRendererInsert(renderer: *mut TessResultRenderer,
                                    next: *mut TessResultRenderer);
    pub fn TessResultRendererNext(renderer: *mut TessResultRenderer) -> *mut TessResultRenderer;
    pub fn TessResultRendererBeginDocument(renderer: *mut TessResultRenderer,
                                           title: *const libc::c_char) -> libc::c_int;
    pub fn TessResultRendererAddImage(renderer: *mut TessResultRenderer,
                                      api: *mut TessBaseAPI) -> libc::c_int;
    pub fn TessResultRendererEndDocument(renderer: *mut TessResultRenderer) -> libc::c_int;
    pub fn TessResultRendererExtention(renderer: *mut TessResultRenderer) -> *const libc::c_char;
    pub fn TessResultRendererTitle(renderer: *mut TessResultRenderer) -> *const libc::c_char;
    pub fn TessResultRendererImageNum(renderer: *mut TessResultRenderer) -> libc::c_int;
    pub fn TessBaseAPICreate() -> *mut TessBaseAPI;
    pub fn TessBaseAPIDelete(handle: *mut TessBaseAPI);
    pub fn TessBaseAPIGetOpenCLDevice(handle: *mut TessBaseAPI,
                                      device: *const *const libc::c_void) -> libc::size_t;
    pub fn TessBaseAPISetInputName(handle: *mut TessBaseAPI,
                                   name: *const libc::c_char);
    pub fn TessBaseAPIGetInputName(handle: *mut TessBaseAPI) -> *const libc::c_char;
    pub fn TessBaseAPISetInputImage(handle: *mut TessBaseAPI,
                                    pix: *const PIX);
    pub fn TessBaseAPIGetInputImage(handle: *mut TessBaseAPI) -> *const PIX;
    pub fn TessBaseAPIGetSourceYResolution(handle: *mut TessBaseAPI) -> libc::c_int;
    pub fn TessBaseAPIGetDatapath(handle: *mut TessBaseAPI) -> *const libc::c_char;
    pub fn TessBaseAPISetOutputName(handle: *mut TessBaseAPI,
                                    name: *const libc::c_char);
    pub fn TessBaseAPISetVariable(handle: *mut TessBaseAPI,
                                  name: *const libc::c_char,
                                  value: *const libc::c_char) -> libc::c_int;
    pub fn TessBaseAPISetDebugVariable(handle: *mut TessBaseAPI,
                                       name: *const libc::c_char,
                                       value: *const libc::c_char) -> libc::c_int;
    pub fn TessBaseAPIGetIntVariable(handle: *const TessBaseAPI,
                                     name: *const libc::c_char,
                                     value: *const libc::c_int) -> libc::c_int;
    pub fn TessBaseAPIGetBoolVariable(handle: *const TessBaseAPI,
                                      name: *const libc::c_char,
                                      value: *const libc::c_int) -> libc::c_int;
    pub fn TessBaseAPIGetDoubleVariable(handle: *const TessBaseAPI,
                                        name: *const libc::c_char,
                                        value: *const libc::c_double) -> libc::c_int;
    pub fn TessBaseAPIGetStringVariable(handle: *const TessBaseAPI,
                                        name: *const libc::c_char) -> *const libc::c_char;
    pub fn TessBaseAPIPrintVariablesToFile(handle: *const TessBaseAPI,
                                           filename: *const libc::c_char) -> libc::c_int;
    pub fn TessBaseAPIInit1(handle: *mut TessBaseAPI,
                            datapath: *const libc::c_char,
                            language: *const libc::c_char,
                            oem: TessOcrEngineMode,
                            configs: *const *const libc::c_char,
                            configs_size: libc::c_int) -> libc::c_int;
    pub fn TessBaseAPIInit2(handle: *mut TessBaseAPI,
                            datapath: *const libc::c_char,
                            language: *const libc::c_char,
                            oem: TessOcrEngineMode) -> libc::c_int;
    pub fn TessBaseAPIInit3(handle: *mut TessBaseAPI,
                            datapath: *const libc::c_char,
                            language: *const libc::c_char) -> libc::c_int;
    pub fn TessBaseAPIInit4(handle: *mut TessBaseAPI,
                            datapath: *const libc::c_char,
                            language: *const libc::c_char,
                            mode: TessOcrEngineMode,
                            configs: *const *const libc::c_char,
                            configs_size: libc::c_int,
                            vars_vec: *const *const libc::c_char,
                            vars_values: *const *const libc::c_char,
                            vars_vec_size: libc::size_t,
                            set_only_non_debug_params: libc::c_int) -> libc::c_int;
    pub fn TessBaseAPIGetInitLanguagesAsString(handle: *const TessBaseAPI) -> *const libc::c_char;
    pub fn TessBaseAPIGetLoadedLanguagesAsVector(handle: *const TessBaseAPI) -> *const *const libc::c_char;
    pub fn TessBaseAPIGetAvailableLanguagesAsVector(handle: *const TessBaseAPI) -> *const *const libc::c_char;
    pub fn TessBaseAPIInitLangMod(handle: *mut TessBaseAPI,
                                  datapath: *const libc::c_char,
                                  language: *const libc::c_char) -> libc::c_int;
    pub fn TessBaseAPIInitForAnalysePage(handle: *mut TessBaseAPI);
    pub fn TessBaseAPIReadConfigFile(handle: *mut TessBaseAPI,
                                     filename: *const libc::c_char);
    pub fn TessBaseAPIReadDebugConfigFile(handle: *mut TessBaseAPI,
                                          filename: *const libc::c_char);
    pub fn TessBaseAPISetPageSegMode(handle: *mut TessBaseAPI,
                                     mode: TessPageSegMode);
    pub fn TessBaseAPIGetPageSegMode(handle: *const TessBaseAPI) -> TessPageSegMode;
    pub fn TessBaseAPIRect(handle: *mut TessBaseAPI,
                           imagedata: *const libc::c_uchar,
                           bytes_per_pixel: libc::c_int,
                           bytes_per_line: libc::c_int,
                           left: libc::c_int,
                           top: libc::c_int,
                           width: libc::c_int,
                           height: libc::c_int) -> *const libc::c_char;
    pub fn TessBaseAPIClearAdaptiveClassifier(handle: *mut TessBaseAPI);
    pub fn TessBaseAPISetImage(handle: *mut TessBaseAPI,
                               imagedata: *const libc::c_uchar,
                               width: libc::c_int,
                               height: libc::c_int,
                               bytes_per_pixel: libc::c_int,
                               bytes_per_line: libc::c_int);
    pub fn TessBaseAPISetImage2(handle: *mut TessBaseAPI, pix: *const PIX);
    pub fn TessBaseAPISetSourceResolution(handle: *mut TessBaseAPI,
                                          ppi: libc::c_int);
    pub fn TessBaseAPISetRectangle(handle: *mut TessBaseAPI,
                                   left: libc::c_int,
                                   top: libc::c_int,
                                   width: libc::c_int,
                                   height: libc::c_int);
    pub fn TessBaseAPISetThresholder(handle: *mut TessBaseAPI,
                                     thresholder: *mut TessImageThresholder);
    pub fn TessBaseAPIGetThresholdedImage(handle: *mut TessBaseAPI) -> *const PIX;
    pub fn TessBaseAPIGetRegions(handle: *mut TessBaseAPI,
                                 pixa: *const *const Pixa) -> *const Boxa;
    pub fn TessBaseAPIGetTextlines(handle: *mut TessBaseAPI,
                                   pixa: *const *const Pixa,
                                   blockids: *const *const libc::c_int) -> *const Boxa;
    pub fn TessBaseAPIGetTextlines1(handle: *mut TessBaseAPI,
                                    raw_image: libc::c_int,
                                    raw_padding: libc::c_int,
                                    pixa: *const *const Pixa,
                                    blockids: *const *const libc::c_int,
                                    paraids: *const *const libc::c_int) -> *const Boxa;
    pub fn TessBaseAPIGetStrips(handle: *mut TessBaseAPI,
                                pixa: *const *const Pixa,
                                blockids: *const *const libc::c_int) -> *const Boxa;
    pub fn TessBaseAPIGetWords(handle: *mut TessBaseAPI,
                               pixa: *const *const Pixa) -> *const Boxa;
    pub fn TessBaseAPIGetConnectedComponents(handle: *mut TessBaseAPI,
                                             cc: *const *const Pixa) -> *const Boxa;
    pub fn TessBaseAPIGetComponentImages(handle: *mut TessBaseAPI,
                                         level: TessPageIteratorLevel,
                                         text_only: libc::c_int,
                                         pixa: *const *const Pixa,
                                         blockids: *const *const libc::c_int) -> *const Boxa;
    pub fn TessBaseAPIGetComponentImages1(handle: *mut TessBaseAPI,
                                          level: TessPageIteratorLevel,
                                          text_only: libc::c_int,
                                          raw_image: libc::c_int,
                                          raw_padding: libc::c_int,
                                          pixa: *const *const Pixa,
                                          blockids: *const *const libc::c_int,
                                          paraids: *const *const libc::c_int) -> *const Boxa;
    pub fn TessBaseAPIGetThresholdedImageScaleFactor(handle: *const TessBaseAPI) -> libc::c_int;
    pub fn TessBaseAPIDumpPGM(handle: *mut TessBaseAPI,
                              filename: *const libc::c_char);
    pub fn TessBaseAPIAnalyseLayout(handle: *mut TessBaseAPI) -> *mut TessPageIterator;
    pub fn TessBaseAPIRecognize(handle: *mut TessBaseAPI,
                                monitor: *const ETEXT_DESC) -> libc::c_int;
    pub fn TessBaseAPIRecognizeForChopTest(handle: *mut TessBaseAPI,
                                           monitor: *const ETEXT_DESC) -> libc::c_int;
    pub fn TessBaseAPIProcessPages(handle: *mut TessBaseAPI,
                                   filename: *const libc::c_char,
                                   retry_config: *const libc::c_char,
                                   timeout_millisec: libc::c_int,
                                   renderer: *mut TessResultRenderer) -> libc::c_int;
    pub fn TessBaseAPIProcessPage(handle: *mut TessBaseAPI,
                                  pix: *const PIX,
                                  page_index: libc::c_int,
                                  filename: *const libc::c_char,
                                  retry_config: *const libc::c_char,
                                  timeout_millisec: libc::c_int,
                                  renderer: *mut TessResultRenderer) -> libc::c_int;
    pub fn TessBaseAPIGetIterator(handle: *mut TessBaseAPI) -> *mut TessResultIterator;
    pub fn TessBaseAPIGetMutableIterator(handle: *mut TessBaseAPI) -> *mut TessMutableIterator;
    pub fn TessBaseAPIGetUTF8Text(handle: *mut TessBaseAPI) -> *const libc::c_char;
    pub fn TessBaseAPIGetHOCRText(handle: *mut TessBaseAPI,
                                  page_number: libc::c_int) -> *const libc::c_char;
    pub fn TessBaseAPIGetBoxText(handle: *mut TessBaseAPI,
                                 page_number: libc::c_int) -> *const libc::c_char;
    pub fn TessBaseAPIGetUNLVText(handle: *mut TessBaseAPI) -> *const libc::c_char;
    pub fn TessBaseAPIMeanTextConf(handle: *mut TessBaseAPI) -> libc::c_int;
    pub fn TessBaseAPIAllWordConfidences(handle: *mut TessBaseAPI) -> *const libc::c_int;
    pub fn TessBaseAPIAdaptToWordStr(handle: *mut TessBaseAPI,
                                     mode: TessPageSegMode,
                                     wordstr: *const libc::c_char) -> libc::c_int;
    pub fn TessBaseAPIClear(handle: *mut TessBaseAPI);
    pub fn TessBaseAPIEnd(handle: *mut TessBaseAPI);
    pub fn TessBaseAPIIsValidWord(handle: *mut TessBaseAPI,
                                  word: *const libc::c_char) -> libc::c_int;
    pub fn TessBaseAPIGetTextDirection(handle: *mut TessBaseAPI,
                                       out_offset: *const libc::c_int,
                                       out_slope: *const libc::c_float) -> libc::c_int;
    pub fn TessBaseAPIClearPersistentCache(handle: *mut TessBaseAPI);
    pub fn TessBaseAPIDetectOS(handle: *mut TessBaseAPI,
                               results: *const OSResults) -> libc::c_int;
    pub fn TessBaseAPIRunAdaptiveClassifier(handle: *mut TessBaseAPI,
                                            blob: *const TBLOB,
                                            num_max_matches: libc::c_int,
                                            unichar_ids: *const libc::c_int,
                                            ratings: *const libc::c_float,
                                            num_matches_returned: *const libc::c_int);
    pub fn TessBaseAPIGetUnichar(handle: *mut TessBaseAPI,
                                 unichar_id: libc::c_int) -> *const libc::c_char;
    pub fn TessBaseAPINumDawgs(handle: *const TessBaseAPI) -> libc::c_int;
    pub fn TessMakeTBLOB(pix: *const PIX) -> *const TBLOB;
    pub fn TessBaseAPIOem(handle: *const TessBaseAPI) -> TessOcrEngineMode;
    pub fn TessBaseAPISetMinOrientationMargin(handle: *mut TessBaseAPI,
                                              margin: libc::c_double);
    pub fn TessBaseGetBlockTextOrientations(handle: *mut TessBaseAPI,
                                            block_orientation: *const *const libc::c_int,
                                            vertical_writing: *const *const libc::c_int);
    pub fn TessBaseAPIFindLinesCreateBlockList(handle: *mut TessBaseAPI) -> *mut BLOCK_LIST;
    pub fn TessPageIteratorDelete(handle: *mut TessPageIterator);
    pub fn TessPageIteratorCopy(handle: *const TessPageIterator) -> *mut TessPageIterator;
    pub fn TessPageIteratorBegin(handle: *mut TessPageIterator);
    pub fn TessPageIteratorNext(handle: *mut TessPageIterator,
                                level: TessPageIteratorLevel) -> libc::c_int;
    pub fn TessPageIteratorIsAtBeginningOf(handle: *const TessPageIterator,
                                           level: TessPageIteratorLevel) -> libc::c_int;
    pub fn TessPageIteratorIsAtFinalElement(handle: *const TessPageIterator,
                                            level: TessPageIteratorLevel,
                                            element: TessPageIteratorLevel) -> libc::c_int;
    pub fn TessPageIteratorBoundingBox(handle: *const TessPageIterator,
                                       level: TessPageIteratorLevel,
                                       left: *const libc::c_int,
                                       top: *const libc::c_int,
                                       right: *const libc::c_int,
                                       bottom: *const libc::c_int) -> libc::c_int;
    pub fn TessPageIteratorBlockType(handle: *const TessPageIterator) -> TessPolyBlockType;
    pub fn TessPageIteratorGetBinaryImage(handle: *const TessPageIterator,
                                          level: TessPageIteratorLevel) -> *const PIX;
    pub fn TessPageIteratorGetImage(handle: *const TessPageIterator,
                                    level: TessPageIteratorLevel,
                                    padding: libc::c_int,
                                    original_image: *const PIX,
                                    left: *const libc::c_int,
                                    top: *const libc::c_int) -> *const PIX;
    pub fn TessPageIteratorBaseline(handle: *const TessPageIterator,
                                    level: TessPageIteratorLevel,
                                    x1: *const libc::c_int,
                                    y1: *const libc::c_int,
                                    x2: *const libc::c_int,
                                    y2: *const libc::c_int) -> libc::c_int;
    pub fn TessPageIteratorOrientation(handle: *mut TessPageIterator,
                                       orientation: *const TessOrientation,
                                       writing_direction: *const TessWritingDirection,
                                       textline_order: *const TessTextlineOrder,
                                       deskew_angle: *const libc::c_float);
    pub fn TessPageIteratorParagraphInfo(handle: *mut TessPageIterator,
                                         justification: *const TessParagraphJustification,
                                         is_list_item: *const libc::c_int,
                                         is_crown: *const libc::c_int,
                                         first_line_indent: *const libc::c_int);
    pub fn TessResultIteratorDelete(handle: *mut TessResultIterator);
    pub fn TessResultIteratorCopy(handle: *const TessResultIterator) -> *mut TessResultIterator;
    pub fn TessResultIteratorGetPageIterator(handle: *mut TessResultIterator) -> *mut TessPageIterator;
    pub fn TessResultIteratorGetPageIteratorConst(handle: *const TessResultIterator) -> *const TessPageIterator;
    pub fn TessResultIteratorGetChoiceIterator(handle: *const TessResultIterator) -> *mut TessChoiceIterator;
    pub fn TessResultIteratorNext(handle: *mut TessResultIterator,
                                  level: TessPageIteratorLevel) -> libc::c_int;
    pub fn TessResultIteratorGetUTF8Text(handle: *const TessResultIterator,
                                         level: TessPageIteratorLevel) -> *const libc::c_char;
    pub fn TessResultIteratorConfidence(handle: *const TessResultIterator,
                                        level: TessPageIteratorLevel) -> libc::c_float;
    pub fn TessResultIteratorWordRecognitionLanguage(handle: *const TessResultIterator) -> *const libc::c_char;
    pub fn TessResultIteratorWordFontAttributes(handle: *const TessResultIterator,
                                                is_bold: *const libc::c_int,
                                                is_italic: *const libc::c_int,
                                                is_underlined: *const libc::c_int,
                                                is_monospace: *const libc::c_int,
                                                is_serif: *const libc::c_int,
                                                is_smallcaps: *const libc::c_int,
                                                pointsize: *const libc::c_int,
                                                font_id: *const libc::c_int) -> *const libc::c_char;
    pub fn TessResultIteratorWordIsFromDictionary(handle: *const TessResultIterator) -> libc::c_int;
    pub fn TessResultIteratorWordIsNumeric(handle: *const TessResultIterator) -> libc::c_int;
    pub fn TessResultIteratorSymbolIsSuperscript(handle: *const TessResultIterator) -> libc::c_int;
    pub fn TessResultIteratorSymbolIsSubscript(handle: *const TessResultIterator) -> libc::c_int;
    pub fn TessResultIteratorSymbolIsDropcap(handle: *const TessResultIterator) -> libc::c_int;
    pub fn TessChoiceIteratorDelete(handle: *mut TessChoiceIterator);
    pub fn TessChoiceIteratorNext(handle: *mut TessChoiceIterator) -> libc::c_int;
    pub fn TessChoiceIteratorGetUTF8Text(handle: *const TessChoiceIterator) -> *const libc::c_char;
    pub fn TessChoiceIteratorConfidence(handle: *const TessChoiceIterator) -> libc::c_float;
}



#[repr(C)]
pub struct TessBaseAPI;

#[repr(C)]
pub struct TessResultIterator;

#[repr(C)]
pub struct TessChoiceIterator;

#[repr(C)]
pub struct TessPageIterator;

#[repr(C)]
pub struct TessMutableIterator;

#[repr(C)]
pub struct BLOCK_LIST;

#[repr(C)]
pub struct TBLOB;

#[repr(C)]
pub struct ETEXT_DESC;

#[repr(C)]
pub struct TessResultRenderer;

#[repr(C)]
pub struct TessImageThresholder;

#[repr(C)]
pub struct OSResults;

#[repr(C)]
pub struct PIX {
    w: libc::uint32_t, 
    h: libc::uint32_t,
    d: libc::uint32_t,
    wpl: libc::uint32_t,
    refcount: libc::uint32_t,
    xres: libc::int32_t,
    yres: libc::int32_t,
    informat: libc::int32_t,
    text: *const libc::c_char,
    colormap: *const PixColormap,
    data: *const libc::uint32_t
}

#[repr(C)]
pub struct Pixa {
    n: libc::int32_t,
    nalloc: libc::int32_t,
    refcount: libc::uint32_t,
    pix: *const *const PIX,
    boxa: *const Boxa
}

#[repr(C)]
pub struct Box {
    x: libc::int32_t,
    y: libc::int32_t,
    w: libc::int32_t,
    h: libc::int32_t,
    refcount: libc::uint32_t

}

#[repr(C)]
pub struct Boxa {
    n: libc::int32_t,
    nalloc: libc::int32_t,
    refcount: libc::uint32_t,
    box_: *const *const Box
}


#[repr(C)]
pub struct PixColormap {
    array: *const libc::c_void,
    depth: libc::int32_t,
    nalloc: libc::int32_t,
    n: libc::int32_t
}



#[allow(dead_code)]
#[repr(C)]
pub enum TessOcrEngineMode     { OEM_TESSERACT_ONLY, OEM_CUBE_ONLY, OEM_TESSERACT_CUBE_COMBINED, OEM_DEFAULT }

#[allow(dead_code)]
#[repr(C)]
pub enum TessPageSegMode       { PSM_OSD_ONLY, PSM_AUTO_OSD, PSM_AUTO_ONLY, PSM_AUTO, PSM_SINGLE_COLUMN, PSM_SINGLE_BLOCK_VERT_TEXT,
                                 PSM_SINGLE_BLOCK, PSM_SINGLE_LINE, PSM_SINGLE_WORD, PSM_CIRCLE_WORD, PSM_SINGLE_CHAR, PSM_SPARSE_TEXT,
                                 PSM_SPARSE_TEXT_OSD, PSM_COUNT }

#[allow(dead_code)]
#[repr(C)]
pub enum TessPageIteratorLevel { RIL_BLOCK, RIL_PARA, RIL_TEXTLINE, RIL_WORD, RIL_SYMBOL}

#[allow(dead_code)]
#[repr(C)]
pub enum TessPolyBlockType     { PT_UNKNOWN, PT_FLOWING_TEXT, PT_HEADING_TEXT, PT_PULLOUT_TEXT, PT_EQUATION, PT_INLINE_EQUATION,
                                 PT_TABLE, PT_VERTICAL_TEXT, PT_CAPTION_TEXT, PT_FLOWING_IMAGE, PT_HEADING_IMAGE,
                                 PT_PULLOUT_IMAGE, PT_HORZ_LINE, PT_VERT_LINE, PT_NOISE, PT_COUNT }

#[allow(dead_code)]
#[repr(C)]
pub enum TessOrientation       { ORIENTATION_PAGE_UP, ORIENTATION_PAGE_RIGHT, ORIENTATION_PAGE_DOWN, ORIENTATION_PAGE_LEFT }

#[allow(dead_code)]
#[repr(C)]
pub enum TessParagraphJustification { JUSTIFICATION_UNKNOWN, JUSTIFICATION_LEFT, JUSTIFICATION_CENTER, JUSTIFICATION_RIGHT }

#[allow(dead_code)]
#[repr(C)]
pub enum TessWritingDirection  { WRITING_DIRECTION_LEFT_TO_RIGHT, WRITING_DIRECTION_RIGHT_TO_LEFT, WRITING_DIRECTION_TOP_TO_BOTTOM }

#[allow(dead_code)]
#[repr(C)]
pub enum TessTextlineOrder     { TEXTLINE_ORDER_LEFT_TO_RIGHT, TEXTLINE_ORDER_RIGHT_TO_LEFT, TEXTLINE_ORDER_TOP_TO_BOTTOM }


