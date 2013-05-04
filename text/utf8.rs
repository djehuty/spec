use tester::*;

#[path="../tester.rs"]
mod tester;

mod text {
  extern mod unicode;
  extern mod utf8;
}

// ASCII equiv
static utf8:  &'static [u8]  = &[
  'h' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8];

static utf16: &'static [u16] = &[
  'h' as u16, 'e' as u16, 'l' as u16, 'l' as u16, 'o' as u16];

static utf32: &'static [u32] = &[
  'h' as u32, 'e' as u32, 'l' as u32, 'l' as u32, 'o' as u32];

// Multibyte code point
static utf8_2:  &'static [u8]  = &[
  'h' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8, 0xc5, 0x9b,
  'w' as u8, 'o' as u8, 'r' as u8, 'l' as u8, 'd' as u8];

static utf16_2: &'static [u16] = &[
  'h' as u16, 'e' as u16, 'l' as u16, 'l' as u16, 'o' as u16,
  0x15b, 'w' as u16, 'o' as u16, 'r' as u16, 'l' as u16, 'd' as u16];

static utf32_2: &'static [u32] = &[
  'h' as u32, 'e' as u32, 'l' as u32, 'l' as u32, 'o' as u32,
  0x15b, 'w' as u32, 'o' as u32, 'r' as u32, 'l' as u32, 'd' as u32];

// Combining mark
static utf8_3:  &'static [u8]  = &[
  'h' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8, 0xcd, 0xa4,
  'w' as u8, 'o' as u8, 'r' as u8, 'l' as u8, 'd' as u8];

static utf16_3: &'static [u16] = &[
  'h' as u16, 'e' as u16, 'l' as u16, 'l' as u16, 'o' as u16,
  0x364, 'w' as u16, 'o' as u16, 'r' as u16, 'l' as u16, 'd' as u16];

static utf32_3: &'static [u32] = &[
  'h' as u32, 'e' as u32, 'l' as u32, 'l' as u32, 'o' as u32,
  0x364, 'w' as u32, 'o' as u32, 'r' as u32, 'l' as u32, 'd' as u32];

describe!("utf8", {
  test!("utf16Length", {
    should!("should work for an empty string", {
      let length = text::utf8::utf16Length([]);
      must!(length eq 0);
    })

    should!("should work for an ASCII string", {
      let length = text::utf8::utf16Length(utf8);
      must!(length eq 5);
    })

    should!("should work for a UTF8 string", {
      let length = text::utf8::utf16Length(utf8_2);
      must!(length eq 11);
    })

    should!("should work with combining marks", {
      let length = text::utf8::utf16Length(utf8_3);
      must!(length eq 11);
    })
  })

  test!("utf32Length", {
    should!("should work for an empty string", {
      let length = text::utf8::utf32Length([]);
      must!(length eq 0);
    })

    should!("should work for an ASCII string", {
      let length = text::utf8::utf32Length(utf8);
      must!(length eq 5);
    })

    should!("should work for a UTF8 string", {
      let length = text::utf8::utf32Length(utf8_2);
      must!(length eq 11);
    })

    should!("should work with combining marks", {
      let length = text::utf8::utf32Length(utf8_3);
      must!(length eq 11);
    })
  })

  test!("toUtf16", {
    should!("should work for an empty string", {
      let buffer:&mut[u16] = &mut[0, 0, 0, 0, 0];
      text::utf8::toUtf16([], buffer);
      must!(buffer[0] eq 0);
    })

    should!("should work for an ASCII string", {
      let buffer:&mut[u16] = &mut[0, 0, 0, 0, 0];
      text::utf8::toUtf16(utf8, buffer);
      for uint::range(0, buffer.len()) |i| {
        must!(buffer[i] eq utf16[i]);
      }
    })

    should!("should work for a UTF8 string", {
      let buffer:&mut[u16] = &mut[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
      text::utf8::toUtf16(utf8_2, buffer);
      for uint::range(0, buffer.len()) |i| {
        must!(buffer[i] eq utf16_2[i]);
      }
    })

    should!("should work with combining marks", {
      let buffer:&mut[u16] = &mut[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
      text::utf8::toUtf16(utf8_3, buffer);
      for uint::range(0, buffer.len()) |i| {
        must!(buffer[i] eq utf16_3[i]);
      }
    })
  })

  test!("length", {
    should!("should work for an empty string", {
      let length = text::utf8::length([]);
      must!(length eq 0);
    })

    should!("should work for an ASCII string", {
      let length = text::utf8::length(utf8);
      must!(length eq 5);
    })

    should!("should work for a UTF8 string", {
      let length = text::utf8::length(utf8_2);
      must!(length eq 11);
    })

    should!("should work with combining marks", {
      let length = text::utf8::length(utf8_3);
      must!(length eq 10);
    })
  })
})
