#[macro_escape];
#[allow(unused_mut)];

pub use std::uint;

pub fn report_failure(expected: ~str, actual: ~str) {
  ::std::io::print(fmt!("\n\x1b[31;1mFail\x1b[39;0m - %s vs %s", actual, expected));
}

trait TestInput {
  fn compare(&self, b: &Self) -> bool;
  fn compare_near(&self, b: &Self, tolerance: Self) -> bool;
  fn fail(&self, b: Self);
}

macro_rules! impl_test_input(
  ($t:ty) => {
    impl TestInput for $t {
      pub fn compare(&self, b: &$t) -> bool {
        *self == *b
      }

      pub fn compare_near(&self, b: &$t, _tolerance: $t) -> bool {
        *self == *b
      }

      pub fn fail(&self, b: $t) {
        report_failure(self.to_str(), b.to_str());
      }
    }
  }
)

macro_rules! impl_test_float(
  ($t:ty) => {
    impl TestInput for $t {
      pub fn compare(&self, b: &$t) -> bool {
        *self == *b
      }

      pub fn compare_near(&self, b: &$t, tolerance: $t) -> bool {
        (*self > b - tolerance) && (*self < b + tolerance)
      }

      pub fn fail(&self, b: $t) {
        report_failure(self.to_str(), b.to_str());
      }
    }
  }
)

macro_rules! impl_test_char(
  ($t:ty) => {
    impl TestInput for $t {
      pub fn compare(&self, b: &$t) -> bool {
        *self == *b
      }

      pub fn compare_near(&self, b: &$t, _tolerance: $t) -> bool {
        *self == *b
      }

      pub fn fail(&self, b: $t) {
        report_failure(fmt!("%c", *self), fmt!("%c", b))
      }
    }
  }
)

impl_test_input!(~str)
impl_test_input!(u8)
impl_test_input!(u16)
impl_test_input!(u32)
impl_test_input!(u64)
impl_test_input!(uint)
impl_test_input!(i8)
impl_test_input!(i16)
impl_test_input!(i32)
impl_test_input!(i64)
impl_test_input!(int)
impl_test_input!(bool)

impl_test_char!(char)

impl_test_float!(float)
impl_test_float!(f32)
impl_test_float!(f64)

pub fn perform_test<T:TestInput>(a:T, b:T, negate: bool) -> bool {
  if (a.compare(&b) ^ negate) {
    true
  }
  else {
    a.fail(b);
    false
  }
}

pub fn perform_near<T:TestInput>(a:T, b:T, tolerance:T, negate:bool) -> bool {
  if (a.compare_near(&b, tolerance) ^ negate) {
    true
  }
  else {
    a.fail(b);
    false
  }
}

pub struct Group {
  func: ~fn(int) -> ~[uint],
  name: &'static str
}

pub struct Test {
  func: ~fn(int) -> ~[uint],
  name: &'static str
}

macro_rules! describe(
  ($prompt:expr, $func:expr) => (
    fn main() {
      #[allow(unused_mut)];
      let module_name = $prompt;

      let mut _setup = |_:int| {};
      let mut _teardown = |_:int| {};

      let mut _actual_tests:~[Group] = ~[];

      let mut _tests:uint = 0;
      let mut _fails:uint = 0;
      let mut _successes:uint = 0;

      ::std::io::println(module_name);

      $func;

      _setup(0);

      for test in _actual_tests.iter() {
        ::std::io::print("  ");
        ::std::io::println(test.name);

        let foo: &fn(int) -> ~[uint] = test.func;
        let result = foo(0);

        _tests += result[0];
        _fails += result[1];
        _successes += result[2];
      }

      _teardown(0);

      let assertions = _successes + _fails;
      ::std::io::println(fmt!("\n%u tests %u assertions %u failures", _tests, assertions, _fails));
    }
  )
)

macro_rules! test(
  ($prompt:expr, $func:expr) => ({
    _actual_tests.push(
      Group { name: $prompt,
              func: |_| -> ~[uint] {
                let mut _tests: ~[Test] = ~[];

                let mut _total = 0;
                let mut _successes = 0;
                let mut _fails = 0;

                $func;

                for test in _tests.iter() {
                  ::std::io::print("    should ");
                  ::std::io::print(test.name);

                  let foo: &fn(int) -> ~[uint] = test.func;
                  let result = foo(0);

                  _total += result[0];
                  _fails += result[1];
                  _successes += result[2];
                }

                ~[_total, _fails, _successes]
              }
            }
    );
  })
)

macro_rules! should(
  ($prompt:expr, $func:expr) => ({
    _tests.push(
      Test { name: $prompt,
             func: |_| -> ~[uint] {
               let mut _failure = false;
               let mut _successes = 0;
               let mut _fails = 0;

               $func;

               if (!_failure) {
                 ::std::io::print(" - ");
                 ::std::io::println("\x1b[32;1mPass\x1b[39;0m");
               }
               else {
                 ::std::io::println("");
               }

               ~[_successes + _fails, _fails, _successes]
             }
           }
    );
  })
)

macro_rules! setup(
  ($func:expr) => ({
    _setup = |_:int| $func;
  })
)

macro_rules! teardown(
  ($func:expr) => ({
    _teardown = |_:int| $func;
  })
)

macro_rules! must(
  ($a:expr eq $b:expr) => (
    if (perform_test($a, $b, false)) { _successes += 1; } else { _failure = true; _fails += 1; }
  );
  ($a:expr near $b:expr) => (
    if (perform_near($a, $b, 0.00001, false)) { _successes += 1; } else { _failure = true; _fails += 1; }
  );
  ($a:expr near $b:expr within $t:expr) => (
    if (perform_near($a, $b, $t, false)) { _successes += 1; } else { _failure = true; _fails += 1; }
  );
)

macro_rules! wont(
  ($a:expr eq $b:expr) => (
    if (perform_test($a, $b, true)) { _successes += 1; } else { _failure = true; _fails += 1; }
  );
  ($a:expr near $b:expr) => (
    if (perform_near($a, $b, 0.00001, true)) { _successes += 1; } else { _failure = true; _fails += 1; }
  );
  ($a:expr near $b:expr within $t:expr) => (
    if (perform_near($a, $b, $t, true)) { _successes += 1; } else { _failure = true; _fails += 1; }
  );
)
