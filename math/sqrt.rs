use tester::*;

#[path="../tester.rs"]
mod tester;

mod math {
  extern mod sqrt;
}

describe!("sqrt", {
  test!("sqrt_f64", {
    should!("should work for 0.0", {
      let result = math::sqrt::sqrt_f64(0.0);
      must!(result eq 0.0);
    })

    should!("should work for some whole square", {
      let result = math::sqrt::sqrt_f64(4.0);
      must!(result eq 2.0);
    })
  })
})
