use tester::*;

#[path="../tester.rs"]
mod tester;

mod locale {
  extern mod english_us;
}

mod chrono {
  extern mod month;
  extern mod weekday;
}

describe!("english_us", {
  test!("language", {
    should!("return English", {
      let result = locale::english_us::language();
      must!(result eq ~"English")
    })
  })

  test!("country", {
    should!("return United States of America", {
      let result = locale::english_us::country();
      must!(result eq ~"United States of America")
    })
  })

  test!("code", {
    should!("return engUS", {
      let result = locale::english_us::code();
      must!(result eq ~"engUS")
    })
  })

  test!("timeShort", {
    should!("parse the given time", {
      let result = locale::english_us::timeShort(11, 43);
      must!(result eq ~"11:43")
    })
  })

  test!("float", {
    should!("parse the given float", {
      let result = locale::english_us::floatingPoint(123.456);
      must!(result eq ~"123.456")
    })
  })

  test!("timeLong", {
    should!("parse the given time", {
      let result = locale::english_us::timeLong(15, 23, 59);
      must!(result eq ~"15:23:59")
    })
  })

  test!("dateShort", {
    should!("parse the given date", {
      let result = locale::english_us::dateShort(chrono::month::April, 4);
      must!(result eq ~"April 4th")
    })

    should!("parse the given date with the 1st", {
      let result = locale::english_us::dateShort(chrono::month::January, 1);
      must!(result eq ~"January 1st")
    })

    should!("parse the given date with the 2nd", {
      let result = locale::english_us::dateShort(chrono::month::March, 2);
      must!(result eq ~"March 2nd")
    })

    should!("parse the given date with the 3rd", {
      let result = locale::english_us::dateShort(chrono::month::May, 3);
      must!(result eq ~"May 3rd")
    })

    should!("parse the given date with the 21st", {
      let result = locale::english_us::dateShort(chrono::month::September, 21);
      must!(result eq ~"September 21st")
    })

    should!("parse the given date with the 22nd", {
      let result = locale::english_us::dateShort(chrono::month::June, 22);
      must!(result eq ~"June 22nd")
    })

    should!("parse the given date with the 23rd", {
      let result = locale::english_us::dateShort(chrono::month::July, 23);
      must!(result eq ~"July 23rd")
    })

    should!("parse the given date with the 31st", {
      let result = locale::english_us::dateShort(chrono::month::August, 31);
      must!(result eq ~"August 31st")
    })

    should!("parse the given date with the 11th", {
      let result = locale::english_us::dateShort(chrono::month::January, 11);
      must!(result eq ~"January 11th")
    })

    should!("parse the given date with the 12th", {
      let result = locale::english_us::dateShort(chrono::month::February, 12);
      must!(result eq ~"February 12th")
    })

    should!("parse the given date with the 13th", {
      let result = locale::english_us::dateShort(chrono::month::October, 13);
      must!(result eq ~"October 13th")
    })
  })

  test!("dateLong", {
    should!("parse the given date", {
      let result = locale::english_us::dateLong(chrono::month::April, 4, 2013);
      must!(result eq ~"April 4th, 2013")
    })

    should!("parse the given date with the 1st", {
      let result = locale::english_us::dateLong(chrono::month::January, 1, 2013);
      must!(result eq ~"January 1st, 2013")
    })

    should!("parse the given date with the 2nd", {
      let result = locale::english_us::dateLong(chrono::month::March, 2, 2043);
      must!(result eq ~"March 2nd, 2043")
    })

    should!("parse the given date with the 3rd", {
      let result = locale::english_us::dateLong(chrono::month::May, 3, 1231);
      must!(result eq ~"May 3rd, 1231")
    })

    should!("parse the given date with the 21st", {
      let result = locale::english_us::dateLong(chrono::month::September, 21, 4423);
      must!(result eq ~"September 21st, 4423")
    })

    should!("parse the given date with the 22nd", {
      let result = locale::english_us::dateLong(chrono::month::June, 22, 4321);
      must!(result eq ~"June 22nd, 4321")
    })

    should!("parse the given date with the 23rd", {
      let result = locale::english_us::dateLong(chrono::month::July, 23, 948);
      must!(result eq ~"July 23rd, 948")
    })

    should!("parse the given date with the 31st", {
      let result = locale::english_us::dateLong(chrono::month::August, 31, 1101);
      must!(result eq ~"August 31st, 1101")
    })

    should!("parse the given date with the 11th", {
      let result = locale::english_us::dateLong(chrono::month::January, 11, 8484);
      must!(result eq ~"January 11th, 8484")
    })

    should!("parse the given date with the 12th", {
      let result = locale::english_us::dateLong(chrono::month::February, 12, 123123);
      must!(result eq ~"February 12th, 123123")
    })

    should!("parse the given date with the 13th", {
      let result = locale::english_us::dateLong(chrono::month::October, 13, 9999);
      must!(result eq ~"October 13th, 9999")
    })
  })

  test!("dateTimeShort", {
    should!("parse the given time", {
      let result = locale::english_us::dateTimeShort(chrono::month::April, 4, 2013, 15, 23);
      must!(result eq ~"15:23 on April 4th, 2013")
    })

    should!("parse the given date", {
      let result = locale::english_us::dateTimeShort(chrono::month::April, 4, 2013, 11, 43);
      must!(result eq ~"11:43 on April 4th, 2013")
    })

    should!("parse the given date with the 1st", {
      let result = locale::english_us::dateTimeShort(chrono::month::January, 1, 2013, 11, 43);
      must!(result eq ~"11:43 on January 1st, 2013")
    })

    should!("parse the given date with the 2nd", {
      let result = locale::english_us::dateTimeShort(chrono::month::March, 2, 2043, 11, 43);
      must!(result eq ~"11:43 on March 2nd, 2043")
    })

    should!("parse the given date with the 3rd", {
      let result = locale::english_us::dateTimeShort(chrono::month::May, 3, 1231, 11, 43);
      must!(result eq ~"11:43 on May 3rd, 1231")
    })

    should!("parse the given date with the 21st", {
      let result = locale::english_us::dateTimeShort(chrono::month::September, 21, 4423, 11, 43);
      must!(result eq ~"11:43 on September 21st, 4423")
    })

    should!("parse the given date with the 22nd", {
      let result = locale::english_us::dateTimeShort(chrono::month::June, 22, 4321, 11, 43);
      must!(result eq ~"11:43 on June 22nd, 4321")
    })

    should!("parse the given date with the 23rd", {
      let result = locale::english_us::dateTimeShort(chrono::month::July, 23, 948, 11, 43);
      must!(result eq ~"11:43 on July 23rd, 948")
    })

    should!("parse the given date with the 31st", {
      let result = locale::english_us::dateTimeShort(chrono::month::August, 31, 1101, 11, 43);
      must!(result eq ~"11:43 on August 31st, 1101")
    })

    should!("parse the given date with the 11th", {
      let result = locale::english_us::dateTimeShort(chrono::month::January, 11, 8484, 11, 43);
      must!(result eq ~"11:43 on January 11th, 8484")
    })

    should!("parse the given date with the 12th", {
      let result = locale::english_us::dateTimeShort(chrono::month::February, 12, 123123, 11, 43);
      must!(result eq ~"11:43 on February 12th, 123123")
    })

    should!("parse the given date with the 13th", {
      let result = locale::english_us::dateTimeShort(chrono::month::October, 13, 9999, 11, 43);
      must!(result eq ~"11:43 on October 13th, 9999")
    })
  })

  test!("month", {
    should!("parse january", {
      let result = locale::english_us::month(chrono::month::January);
      must!(result eq ~"January")
    })

    should!("parse february", {
      let result = locale::english_us::month(chrono::month::February);
      must!(result eq ~"February")
    })

    should!("parse march", {
      let result = locale::english_us::month(chrono::month::March);
      must!(result eq ~"March")
    })

    should!("parse april", {
      let result = locale::english_us::month(chrono::month::April);
      must!(result eq ~"April")
    })

    should!("parse may", {
      let result = locale::english_us::month(chrono::month::May);
      must!(result eq ~"May")
    })

    should!("parse june", {
      let result = locale::english_us::month(chrono::month::June);
      must!(result eq ~"June")
    })

    should!("parse july", {
      let result = locale::english_us::month(chrono::month::July);
      must!(result eq ~"July")
    })

    should!("parse august", {
      let result = locale::english_us::month(chrono::month::August);
      must!(result eq ~"August")
    })

    should!("parse september", {
      let result = locale::english_us::month(chrono::month::September);
      must!(result eq ~"September")
    })

    should!("parse october", {
      let result = locale::english_us::month(chrono::month::October);
      must!(result eq ~"October")
    })

    should!("parse november", {
      let result = locale::english_us::month(chrono::month::November);
      must!(result eq ~"November")
    })

    should!("parse december", {
      let result = locale::english_us::month(chrono::month::December);
      must!(result eq ~"December")
    })
  })

  test!("weekday", {
    should!("parse sunday", {
      let result = locale::english_us::weekday(chrono::weekday::Sunday);
      must!(result eq ~"Sunday")
    })

    should!("parse monday", {
      let result = locale::english_us::weekday(chrono::weekday::Monday);
      must!(result eq ~"Monday")
    })

    should!("parse tuesday", {
      let result = locale::english_us::weekday(chrono::weekday::Tuesday);
      must!(result eq ~"Tuesday")
    })

    should!("parse wednesday", {
      let result = locale::english_us::weekday(chrono::weekday::Wednesday);
      must!(result eq ~"Wednesday")
    })

    should!("parse thursday", {
      let result = locale::english_us::weekday(chrono::weekday::Thursday);
      must!(result eq ~"Thursday")
    })

    should!("parse friday", {
      let result = locale::english_us::weekday(chrono::weekday::Friday);
      must!(result eq ~"Friday")
    })

    should!("parse saturday", {
      let result = locale::english_us::weekday(chrono::weekday::Saturday);
      must!(result eq ~"Saturday")
    })
  })
})
