use tester::*;

#[path="../tester.rs"]
mod tester;

mod chrono {
  extern mod month;
  extern mod weekday;
  extern mod date;
}

describe!("Date", {
  test!("isLeapYear", {
    should!("should detect non leap years", {
      let date = chrono::date::Date { month: chrono::month::April, day: 27, year: 2013 };
      must!(date.isLeapYear() eq false)
    })

    should!("should report a year divisible by 4 as a leap year", {
      let date = chrono::date::Date { month: chrono::month::April, day: 27, year: 2004 };
      must!(date.isLeapYear() eq true)
    })

    should!("should not consider dates divisible by 100", {
      let date = chrono::date::Date { month: chrono::month::April, day: 27, year: 1900 };
      must!(date.isLeapYear() eq false)
    })

    should!("should consider dates divisible by 100 when divisible by 400", {
      let date = chrono::date::Date { month: chrono::month::April, day: 27, year: 2000 };
      must!(date.isLeapYear() eq true)
    })
  })

  test!("dayOfWeek", {
    should!("should return a Sunday", {
      let date = chrono::date::Date { month: chrono::month::April, day: 28, year: 2013 };
      must!(date.dayOfWeek() as u64 eq chrono::weekday::Sunday as u64)
    })

    should!("should return a Monday", {
      let date = chrono::date::Date { month: chrono::month::April, day: 29, year: 2013 };
      must!(date.dayOfWeek() as u64 eq chrono::weekday::Monday as u64)
    })

    should!("should return a Tuesday", {
      let date = chrono::date::Date { month: chrono::month::April, day: 30, year: 2013 };
      must!(date.dayOfWeek() as u64 eq chrono::weekday::Tuesday as u64)
    })

    should!("should return a Wednesday", {
      let date = chrono::date::Date { month: chrono::month::May, day: 1, year: 2013 };
      must!(date.dayOfWeek() as u64 eq chrono::weekday::Wednesday as u64)
    })

    should!("should return a Thursday", {
      let date = chrono::date::Date { month: chrono::month::May, day: 2, year: 2013 };
      must!(date.dayOfWeek() as u64 eq chrono::weekday::Thursday as u64)
    })

    should!("should return a Friday", {
      let date = chrono::date::Date { month: chrono::month::May, day: 3, year: 2013 };
      must!(date.dayOfWeek() as u64 eq chrono::weekday::Friday as u64)
    })

    should!("should return a Saturday", {
      let date = chrono::date::Date { month: chrono::month::May, day: 4, year: 2013 };
      must!(date.dayOfWeek() as u64 eq chrono::weekday::Saturday as u64)
    })
  })
})
