src/page_seg_mode.rs: page-seg-modes.txt build_page_seg_modes.py
	python build_page_seg_modes.py < page-seg-modes.txt | rustfmt > $@
