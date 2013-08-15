use tester::*;

use std::str;

#[path="../tester.rs"]
mod tester;

mod io {
  extern mod file;
  extern mod stream;
  extern mod console;
}

describe!("File", {
  setup!({
    match io::file::create(".tmp/io_file_foo", io::stream::Write) {
      ~io::stream::Writer(f) => f.write(['a' as u8, 'b' as u8, 'c' as u8, 'd' as u8]),
      _ => {}
    };

    match io::file::create(".tmp/io_file_bar", io::stream::Write) {
      _ => {}
    };
  })

  test!("open", {
    should!("open the file for reading", {
      let reader = io::file::open(".tmp/io_file_foo", io::stream::Read);

      let result =
        match reader {
          ~io::stream::Reader(_) => ~"read",
          _ => ~"nope"
        };

      must!(result eq ~"read");
    })

    should!("open the file for writing", {
      let writer = io::file::open(".tmp/io_file_bar", io::stream::Write);

      let result =
        match writer {
          ~io::stream::Writer(_) => ~"write",
          _ => ~"nope"
        };

      must!(result eq ~"write");
    })
  })

  test!("read", {
    should!("read the given number of bytes", {
      let reader = io::file::open(".tmp/io_file_foo", io::stream::Read);

      let result =
        match reader {
          ~io::stream::Reader(r) => str::from_bytes(r.read(2)),
          _ => ~"nope"
        };

      must!(result eq ~"ab");
    })

    should!("read the given number of bytes after a seek", {
      let reader = io::file::open(".tmp/io_file_foo", io::stream::Read);

      let result =
        match reader {
          ~io::stream::Reader(r) => {
            r.seek(2);
            str::from_bytes(r.read(2))
          }
          _ => ~"nope"
        };

      must!(result eq ~"cd");
    })

    should!("read from the last position", {
      let reader = io::file::open(".tmp/io_file_foo", io::stream::Read);

      let result =
        match reader {
          ~io::stream::Reader(r) => {
            r.read(2);
            str::from_bytes(r.read(2))
          }
          _ => ~"nope"
        };

      must!(result eq ~"cd");
    })
  })

  test!("write", {
    should!("write the given bytes", {
      {
        let reader = io::file::open(".tmp/io_file_bar", io::stream::Write);

        match reader {
          ~io::stream::Writer(r) => {r.write(['x' as u8, 'z' as u8]); ~"ok"},
          _ => ~"nope"
        };
      }

      let result1 = match io::file::open(".tmp/io_file_bar", io::stream::Read) {
        ~io::stream::Reader(r) => str::from_bytes(r.read(2)),
        _ => ~"nope"
      };

      must!(result1 eq ~"xz");

      {
        let reader = io::file::open(".tmp/io_file_bar", io::stream::Write);

        match reader {
          ~io::stream::Writer(r) => {r.write(['z' as u8, 'x' as u8]); ~"ok"},
          _ => ~"nope"
        };
      }

      let result2 = match io::file::open(".tmp/io_file_bar", io::stream::Read) {
        ~io::stream::Reader(r) => str::from_bytes(r.read(2)),
        _ => ~"nope"
      };

      must!(result2 eq ~"zx");
    })
  })

  test!("seek", {
    should!("seek forward", {
      let reader = io::file::open(".tmp/io_file_foo", io::stream::Read);

      let result =
        match reader {
          ~io::stream::Reader(r) => {
            r.seek(2);
            str::from_bytes(r.read(2))
          }
          _ => ~"nope"
        };

      must!(result eq ~"cd");
    })

    should!("seek backward", {
      let reader = io::file::open(".tmp/io_file_foo", io::stream::Read);

      let result =
        match reader {
          ~io::stream::Reader(r) => {
            r.seek(3);
            r.seek(-1);
            str::from_bytes(r.read(2))
          }
          _ => ~"nope"
        };

      must!(result eq ~"cd");
    })
  })
})
