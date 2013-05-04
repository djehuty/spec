use tester::*;

#[path="../tester.rs"]
mod tester;

mod math {
  extern mod abs;
}

/* TODO:

   Add fuzzy tests.

*/

describe!("abs", {
  test!("abs_f64", {
    should!("should work for 0.0", {
      let result = math::abs::abs_f64(0.0);
      must!(result eq 0.0);
    })

    should!("should work for a positive value", {
      let result = math::abs::abs_f64(5.0);
      must!(result eq 5.0);
    })

    should!("should work for a negative value", {
      let result = math::abs::abs_f64(-5.0);
      must!(result eq 5.0);
    })
  })

  test!("abs_f32", {
    should!("should work for 0.0", {
      let result = math::abs::abs_f32(0.0);
      must!(result eq 0.0);
    })

    should!("should work for a positive value", {
      let result = math::abs::abs_f32(5.0);
      must!(result eq 5.0);
    })

    should!("should work for a negative value", {
      let result = math::abs::abs_f32(-5.0);
      must!(result eq 5.0);
    })
  })

  test!("abs_i32", {
    should!("should work for 0.0", {
      let result = math::abs::abs_i32(0);
      must!(result eq 0);
    })

    should!("should work for a positive value", {
      let result = math::abs::abs_i32(5);
      must!(result eq 5);
    })

    should!("should work for a negative value", {
      let result = math::abs::abs_i32(-5);
      must!(result eq 5);
    })
  })

  test!("abs_i64", {
    should!("should work for 0.0", {
      let result = math::abs::abs_i64(0);
      must!(result eq 0);
    })

    should!("should work for a positive value", {
      let result = math::abs::abs_i64(5);
      must!(result eq 5);
    })

    should!("should work for a negative value", {
      let result = math::abs::abs_i64(-5);
      must!(result eq 5);
    })
  })
})
