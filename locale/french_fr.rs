use tester::*;

#[path="../tester.rs"]
mod tester;

mod locale {
  extern mod french_fr;
}

mod chrono {
  extern mod month;
  extern mod weekday;
}

describe!("french_fr", {
  test!("language", {
    should!("return fran\u00e7ais", {
      let result = locale::french_fr::language();
      must!(result eq ~"fran\u00e7ais")
    })
  })

  test!("country", {
    should!("return france", {
      let result = locale::french_fr::country();
      must!(result eq ~"france")
    })
  })

  test!("code", {
    should!("return fraFR", {
      let result = locale::french_fr::code();
      must!(result eq ~"fraFR")
    })
  })

  test!("timeShort", {
    should!("parse the given time", {
      let result = locale::french_fr::timeShort(11, 43);
      must!(result eq ~"11 h 43")
    })
  })

  test!("float", {
    should!("parse the given float", {
      let result = locale::french_fr::floatingPoint(123.456);
      must!(result eq ~"123.456")
    })
  })

  test!("timeLong", {
    should!("parse the given time", {
      let result = locale::french_fr::timeLong(15, 23, 59);
      must!(result eq ~"15:23:59")
    })
  })

  test!("dateShort", {
    should!("parse the given date", {
      let result = locale::french_fr::dateShort(chrono::month::April, 4);
      must!(result eq ~"avril 4")
    })
  })

  test!("dateLong", {
    should!("parse the given date", {
      let result = locale::french_fr::dateLong(chrono::month::April, 4, 2013);
      must!(result eq ~"avril 4 2013")
    })
  })

  test!("dateTimeShort", {
    should!("parse the given time", {
      let result = locale::french_fr::dateTimeShort(chrono::month::April, 4, 2013, 15, 23);
      must!(result eq ~"15 h 23 on avril 4 2013")
    })

    should!("parse the given date", {
      let result = locale::french_fr::dateTimeShort(chrono::month::April, 4, 2013, 11, 43);
      must!(result eq ~"11 h 43 on avril 4 2013")
    })
  })

  test!("month", {
    should!("parse january", {
      let result = locale::french_fr::month(chrono::month::January);
      must!(result eq ~"janvier")
    })

    should!("parse february", {
      let result = locale::french_fr::month(chrono::month::February);
      must!(result eq ~"f\u00e9vier")
    })

    should!("parse march", {
      let result = locale::french_fr::month(chrono::month::March);
      must!(result eq ~"mars")
    })

    should!("parse april", {
      let result = locale::french_fr::month(chrono::month::April);
      must!(result eq ~"avril")
    })

    should!("parse may", {
      let result = locale::french_fr::month(chrono::month::May);
      must!(result eq ~"mai")
    })

    should!("parse june", {
      let result = locale::french_fr::month(chrono::month::June);
      must!(result eq ~"juin")
    })

    should!("parse july", {
      let result = locale::french_fr::month(chrono::month::July);
      must!(result eq ~"juillet")
    })

    should!("parse august", {
      let result = locale::french_fr::month(chrono::month::August);
      must!(result eq ~"ao\u00fbt")
    })

    should!("parse september", {
      let result = locale::french_fr::month(chrono::month::September);
      must!(result eq ~"septembre")
    })

    should!("parse october", {
      let result = locale::french_fr::month(chrono::month::October);
      must!(result eq ~"octobre")
    })

    should!("parse november", {
      let result = locale::french_fr::month(chrono::month::November);
      must!(result eq ~"novembre")
    })

    should!("parse december", {
      let result = locale::french_fr::month(chrono::month::December);
      must!(result eq ~"d\u00e9cember")
    })
  })

  test!("weekday", {
    should!("parse sunday", {
      let result = locale::french_fr::weekday(chrono::weekday::Sunday);
      must!(result eq ~"dimanche")
    })

    should!("parse monday", {
      let result = locale::french_fr::weekday(chrono::weekday::Monday);
      must!(result eq ~"lundi")
    })

    should!("parse tuesday", {
      let result = locale::french_fr::weekday(chrono::weekday::Tuesday);
      must!(result eq ~"mardi")
    })

    should!("parse wednesday", {
      let result = locale::french_fr::weekday(chrono::weekday::Wednesday);
      must!(result eq ~"mercredi")
    })

    should!("parse thursday", {
      let result = locale::french_fr::weekday(chrono::weekday::Thursday);
      must!(result eq ~"jeudi")
    })

    should!("parse friday", {
      let result = locale::french_fr::weekday(chrono::weekday::Friday);
      must!(result eq ~"vendredi")
    })

    should!("parse saturday", {
      let result = locale::french_fr::weekday(chrono::weekday::Saturday);
      must!(result eq ~"samedi")
    })
  })
})
