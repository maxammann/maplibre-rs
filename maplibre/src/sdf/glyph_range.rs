use crate::sdf::util::hash;
use std::ops::Range;

pub type GlyphRange = Range<u16>;

const GLYPHS_PER_GLYPH_RANGE: u32 = 256;
const GLYPH_RANGES_PER_FONT_STACK: u32 = 256;
// 256 - 126 ranges skipped w/ i18n::allowsFixedWidthGlyphGeneration
const NON_IDEOGRAPH_GLYPH_RANGES_PER_FONT_STACK: u32 = 130;

fn hash_glyphrange(range: &GlyphRange) -> u64 {
    hash(&[range.start, range.end])
}