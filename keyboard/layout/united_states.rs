use tester::*;

#[path="../../tester.rs"]
mod tester;

mod keyboard {
  extern mod key;
  mod layout {
    extern mod united_states;
  }
}

fn press_key(keycode: u32) -> char {
  let key = keyboard::key::Key { code: keycode,
                                 leftAlt: false,
                                 rightAlt: false,
                                 leftControl: false,
                                 rightControl: false,
                                 leftShift: false,
                                 rightShift: false,
                                 capsLock: false };

  keyboard::layout::united_states::translate(key, 0) as char
}

fn press_shift_key(keycode: u32) -> char {
  let key = keyboard::key::Key { code: keycode,
                                 leftAlt: false,
                                 rightAlt: false,
                                 leftControl: false,
                                 rightControl: false,
                                 leftShift: true,
                                 rightShift: false,
                                 capsLock: false };

  keyboard::layout::united_states::translate(key, 0) as char
}

describe!("keyboard::layout", {
  test!("united_states", {
    should!("A key types 'a'", {
      must!(press_key(97) eq 'a');
    })

    should!("B key types 'b'", {
      must!(press_key(133) eq 'b');
    })

    should!("C key types 'c'", {
      must!(press_key(131) eq 'c');
    })

    should!("D key types 'd'", {
      must!(press_key(99) eq 'd');
    })

    should!("E key types 'e'", {
      must!(press_key(67) eq 'e');
    })

    should!("F key types 'f'", {
      must!(press_key(100) eq 'f');
    })

    should!("G key types 'g'", {
      must!(press_key(101) eq 'g');
    })

    should!("H key types 'h'", {
      must!(press_key(102) eq 'h');
    })

    should!("I key types 'i'", {
      must!(press_key(72) eq 'i');
    })

    should!("J key types 'j'", {
      must!(press_key(103) eq 'j');
    })

    should!("K key types 'k'", {
      must!(press_key(104) eq 'k');
    })

    should!("L key types 'l'", {
      must!(press_key(105) eq 'l');
    })

    should!("M key types 'm'", {
      must!(press_key(135) eq 'm');
    })

    should!("N key types 'n'", {
      must!(press_key(134) eq 'n');
    })

    should!("O key types 'o'", {
      must!(press_key(73) eq 'o');
    })

    should!("P key types 'p'", {
      must!(press_key(74) eq 'p');
    })

    should!("Q key types 'q'", {
      must!(press_key(65) eq 'q');
    })

    should!("R key types 'r'", {
      must!(press_key(68) eq 'r');
    })

    should!("S key types 's'", {
      must!(press_key(98) eq 's');
    })

    should!("T key types 't'", {
      must!(press_key(69) eq 't');
    })

    should!("U key types 'u'", {
      must!(press_key(71) eq 'u');
    })

    should!("V key types 'v'", {
      must!(press_key(132) eq 'v');
    })

    should!("W key types 'w'", {
      must!(press_key(66) eq 'w');
    })

    should!("X key types 'x'", {
      must!(press_key(130) eq 'x');
    })

    should!("Y key types 'y'", {
      must!(press_key(70) eq 'y');
    })

    should!("Z key types 'z'", {
      must!(press_key(129) eq 'z');
    })

    should!("0 key types '0'", {
      must!(press_key(42) eq '0');
    })

    should!("1 key types '1'", {
      must!(press_key(33) eq '1');
    })

    should!("2 key types '2'", {
      must!(press_key(34) eq '2');
    })

    should!("3 key types '3'", {
      must!(press_key(35) eq '3');
    })

    should!("4 key types '4'", {
      must!(press_key(36) eq '4');
    })

    should!("5 key types '5'", {
      must!(press_key(37) eq '5');
    })

    should!("6 key types '6'", {
      must!(press_key(38) eq '6');
    })

    should!("7 key types '7'", {
      must!(press_key(39) eq '7');
    })

    should!("8 key types '8'", {
      must!(press_key(40) eq '8');
    })

    should!("9 key types '9'", {
      must!(press_key(41) eq '9');
    })

    should!("Semicolon key types ';'", {
      must!(press_key(106) eq ';');
    })

    should!("Apostrophe key types '", {
      must!(press_key(107) eq '\'');
    })

    should!("Comma key types ','", {
      must!(press_key(136) eq ',');
    })

    should!("Period key types '.'", {
      must!(press_key(137) eq '.');
    })

    should!("Foreslash key types '/'", {
      must!(press_key(138) eq '/');
    })

    should!("Backslash key types '\\'", {
      must!(press_key(77) eq '\\');
    })

    should!("Left bracket key types '['", {
      must!(press_key(75) eq '[');
    })

    should!("Right bracket key types ']'", {
      must!(press_key(76) eq ']');
    })

    should!("Single quote key types '`'", {
      must!(press_key(32) eq '`');
    })

    should!("Minus key types '-'", {
      must!(press_key(43) eq '-');
    })

    should!("Equals key types '='", {
      must!(press_key(44) eq '=');
    })

    should!("Space key types ' '", {
      must!(press_key(163) eq ' ');
    })

    should!("Shift + A key types 'A'", {
      must!(press_shift_key(97) eq 'A');
    })

    should!("Shift + B key types 'B'", {
      must!(press_shift_key(133) eq 'B');
    })

    should!("Shift + C key types 'C'", {
      must!(press_shift_key(131) eq 'C');
    })

    should!("Shift + D key types 'D'", {
      must!(press_shift_key(99) eq 'D');
    })

    should!("Shift + E key types 'E'", {
      must!(press_shift_key(67) eq 'E');
    })

    should!("Shift + F key types 'F'", {
      must!(press_shift_key(100) eq 'F');
    })

    should!("Shift + G key types 'G'", {
      must!(press_shift_key(101) eq 'G');
    })

    should!("Shift + H key types 'H'", {
      must!(press_shift_key(102) eq 'H');
    })

    should!("Shift + I key types 'I'", {
      must!(press_shift_key(72) eq 'I');
    })

    should!("Shift + J key types 'J'", {
      must!(press_shift_key(103) eq 'J');
    })

    should!("Shift + K key types 'K'", {
      must!(press_shift_key(104) eq 'K');
    })

    should!("Shift + L key types 'L'", {
      must!(press_shift_key(105) eq 'L');
    })

    should!("Shift + M key types 'M'", {
      must!(press_shift_key(135) eq 'M');
    })

    should!("Shift + N key types 'N'", {
      must!(press_shift_key(134) eq 'N');
    })

    should!("Shift + O key types 'O'", {
      must!(press_shift_key(73) eq 'O');
    })

    should!("Shift + P key types 'P'", {
      must!(press_shift_key(74) eq 'P');
    })

    should!("Shift + Q key types 'Q'", {
      must!(press_shift_key(65) eq 'Q');
    })

    should!("Shift + R key types 'R'", {
      must!(press_shift_key(68) eq 'R');
    })

    should!("Shift + S key types 'S'", {
      must!(press_shift_key(98) eq 'S');
    })

    should!("Shift + T key types 'T'", {
      must!(press_shift_key(69) eq 'T');
    })

    should!("Shift + U key types 'U'", {
      must!(press_shift_key(71) eq 'U');
    })

    should!("Shift + V key types 'V'", {
      must!(press_shift_key(132) eq 'V');
    })

    should!("Shift + W key types 'W'", {
      must!(press_shift_key(66) eq 'W');
    })

    should!("Shift + X key types 'X'", {
      must!(press_shift_key(130) eq 'X');
    })

    should!("Shift + Y key types 'Y'", {
      must!(press_shift_key(70) eq 'Y');
    })

    should!("Shift + Z key types 'Z'", {
      must!(press_shift_key(129) eq 'Z');
    })

    should!("Shift + 0 key types ')'", {
      must!(press_shift_key(42) eq ')');
    })

    should!("Shift + 1 key types '!'", {
      must!(press_shift_key(33) eq '!');
    })

    should!("Shift + 2 key types '@'", {
      must!(press_shift_key(34) eq '@');
    })

    should!("Shift + 3 key types '#'", {
      must!(press_shift_key(35) eq '#');
    })

    should!("Shift + 4 key types '$'", {
      must!(press_shift_key(36) eq '$');
    })

    should!("Shift + 5 key types '%'", {
      must!(press_shift_key(37) eq '%');
    })

    should!("Shift + 6 key types '^'", {
      must!(press_shift_key(38) eq '^');
    })

    should!("Shift + 7 key types '&'", {
      must!(press_shift_key(39) eq '&');
    })

    should!("Shift + 8 key types '*'", {
      must!(press_shift_key(40) eq '*');
    })

    should!("Shift + 9 key types '('", {
      must!(press_shift_key(41) eq '(');
    })

    should!("Shift + Semicolon key types ':'", {
      must!(press_shift_key(106) eq ':');
    })

    should!("Shift + Apostrophe key types '\"'", {
      must!(press_shift_key(107) eq '"');
    })

    should!("Shift + Comma key types '<'", {
      must!(press_shift_key(136) eq '<');
    })

    should!("Shift + Period key types '>'", {
      must!(press_shift_key(137) eq '>');
    })

    should!("Shift + Foreslash key types '?'", {
      must!(press_shift_key(138) eq '?');
    })

    should!("Shift + Backslash key types '|'", {
      must!(press_shift_key(77) eq '|');
    })

    should!("Shift + Left bracket key types '{'", {
      must!(press_shift_key(75) eq '{');
    })

    should!("Shift + Right bracket key types '}'", {
      must!(press_shift_key(76) eq '}');
    })

    should!("Shift + Single quote key types '~'", {
      must!(press_shift_key(32) eq '~');
    })

    should!("Shift + Minus key types '_'", {
      must!(press_shift_key(43) eq '_');
    })

    should!("Shift + Equals key types '+'", {
      must!(press_shift_key(44) eq '+');
    })

    should!("Shift + Space key types ' '", {
      must!(press_shift_key(163) eq ' ');
    })
  })
})
