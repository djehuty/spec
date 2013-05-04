use tester::*;

#[path="../tester.rs"]
mod tester;

mod math {
  extern mod random;
}

describe!("Random", {
  test!("next_u32", {
    should!("should not return the seed", {
      let mut random = math::random::Random { state: 12345678 };
      let result = random.next_u32();

      wont!(result eq 12345678);
    })

    should!("should not return same result twice", {
      let mut random = math::random::Random { state: 12345678 };
      let check  = random.next_u32();
      let result = random.next_u32();

      wont!(result eq check);
    })
  })

  test!("next_u64", {
    should!("should not return same result twice", {
      let mut random = math::random::Random { state: 12345678 };
      let check  = random.next_u64();
      let result = random.next_u64();

      wont!(result eq check);
    })
  })

  test!("next_f32", {
    should!("should not return same result twice", {
      let mut random = math::random::Random { state: 12345678 };
      let check  = random.next_f32();
      let result = random.next_f32();

      wont!(result eq check);
    })
  })

  test!("next_f64", {
    should!("should not return same result twice", {
      let mut random = math::random::Random { state: 12345678 };
      let check  = random.next_f64();
      let result = random.next_f64();

      wont!(result eq check);
    })
  })
})
