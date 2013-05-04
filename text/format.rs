use tester::*;

#[path="../tester.rs"]
mod tester;

mod text {
  extern mod format;
}

describe!("format", {
  test!("float", {
    should!("work on a basic decimal base", {
      let result = text::format::floatingPoint(123.5, 10, ~".");
      must!(result eq ~"123.5");
    })

    should!("work on infinity", {
      let result = text::format::floatingPoint(1.0_f64/0.0_f64, 10, ~".");
      must!(result eq ~"inf")
    })

    should!("work on negative infinity", {
      let result = text::format::floatingPoint(-1.0_f64/0.0_f64, 10, ~".");
      must!(result eq ~"-inf")
    })

    should!("work on nan", {
      let result = text::format::floatingPoint(0.0_f64/0.0_f64, 10, ~".");
      must!(result eq ~"nan")
    })

    should!("work with any point separator", {
      let result = text::format::floatingPoint(45.645, 10, ~"x#x");
      must!(result eq ~"45x#x645")
    })
  })

  test!("integer", {
    should!("work on a basic decimal base", {
      let result = text::format::integer(1234, 10);
      must!(result eq ~"1234");
    })

    should!("work on a basic octal base", {
      let result = text::format::integer(1234, 8);
      must!(result eq ~"2322");
    })

    should!("work on a basic hexidecimal base", {
      let result = text::format::integer(0x89ab, 16);
      must!(result eq ~"89ab");
    })

    should!("work on a negative decimal base", {
      let result = text::format::integer(-1234, 10);
      must!(result eq ~"-1234");
    })

    should!("work on a negative octal base", {
      let result = text::format::integer(-1234, 8);
      must!(result eq ~"-2322");
    })

    should!("work on a negative hexidecimal base", {
      let result = text::format::integer(-0x89ab, 16);
      must!(result eq ~"-89ab");
    })
  })
})
