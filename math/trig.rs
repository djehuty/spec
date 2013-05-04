use tester::*;

#[path="../tester.rs"]
mod tester;

mod math {
  extern mod trig;
}

// Trig approximations only need to be valid within a certain margin of error.

/* TODO:

   Add fuzzy test!s to check invariants while being wary of approximation issues:
    * sin(x)  == cos(x - pi/2) or sin(x) == cos(x + 3pi/2)
    * sin(-x) == -sin(x) and cos(-x) == cos(x)
    * sin(x +,- y) = sin(x) * cos(y) +,- cos(x) * sin(y)
    * sin(2*x) == 2 * sin(x) * cos(x)
    * cos(2*x) == cos(x)^2 - sin(x)^2

*/

describe!("trig", {
  test!("cos", {
    should!("should work for 0.0", {
      let result = math::trig::cos(0.0);
      must!(result eq 1.0);
    })

    should!("should work for 1/2 pi", {
      let PI = 3.14159265358979323846_f64;
      let result = math::trig::cos(PI / 2.0);
      must!(result near 0.0);
    })

    should!("should work for pi", {
      let PI = 3.14159265358979323846_f64;
      let result = math::trig::cos(PI);
      must!(result near -1.0);
    })

    should!("should work for 3/2 pi", {
      let PI = 3.14159265358979323846_f64;
      let result = math::trig::cos(PI * 3.0 / 2.0);
      must!(result near 0.0);
    })

    should!("should work for 2 pi", {
      let PI = 3.14159265358979323846_f64;
      let result = math::trig::cos(PI * 2.0);
      must!(result near 1.0);
    })
  })

  test!("sin", {
    should!("should work for 0.0", {
      let result = math::trig::sin(0.0);
      must!(result eq 0.0);
    })

    should!("should work for 1/2 pi", {
      let PI = 3.14159265358979323846_f64;
      let result = math::trig::sin(PI / 2.0);
      must!(result near 1.0);
    })

    should!("should work for pi", {
      let PI = 3.14159265358979323846_f64;
      let result = math::trig::sin(PI);
      must!(result near 0.0);
    })

    should!("should work for 3/2 pi", {
      let PI = 3.14159265358979323846_f64;
      let result = math::trig::sin(PI * 3.0 / 2.0);
      must!(result near -1.0);
    })

    should!("should work for 2 pi", {
      let PI = 3.14159265358979323846_f64;
      let result = math::trig::sin(PI * 2.0);
      must!(result near 0.0);
    })
  })
})
