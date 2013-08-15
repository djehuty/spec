use tester::*;
use std::str;

#[path="../tester.rs"]
mod tester;

mod io {
  extern mod console;
  extern mod stream;
  extern mod file;
}

mod drawing {
  extern mod color;
}

fn setup() -> ~[int] {
  unsafe {
    let dupped = ::std::libc::funcs::posix88::unistd::dup(::std::libc::consts::os::posix88::STDOUT_FILENO);

    let temp_out =
      do "tmp".to_c_str().with_ref |pathbuf| {
        do "w+b".to_c_str().with_ref |modebuf| {
          ::std::libc::fopen(pathbuf, modebuf)
        }
      };

    ::std::libc::funcs::posix88::unistd::dup2(::std::libc::funcs::posix88::stdio::fileno(temp_out),
                                              ::std::libc::consts::os::posix88::STDOUT_FILENO);

    ~[dupped as int, temp_out as int]
  }
}

fn teardown(fd:~[int]) {
  unsafe {
    ::std::libc::funcs::posix88::unistd::dup2(fd[0] as ::std::libc::c_int,
                                              ::std::libc::consts::os::posix88::STDOUT_FILENO);
    ::std::libc::funcs::posix88::unistd::close(fd[0] as ::std::libc::c_int);

    ::std::libc::fclose(fd[1] as *::std::libc::FILE);
  }
}

describe!("console", {
  test!("print", {
    should!("write out the given string", {
      let fd = setup();
      io::console::print("ok");
      teardown(fd);

      let result =
        match io::file::open("tmp", io::stream::Read) {
          ~io::stream::Reader(r) => { let bytes = r.read(5); str::from_bytes([bytes[0], bytes[4]]) },
          _ => ~"bad"
        };

      must!(result eq ~"ok");
    })
  })

  test!("println", {
    should!("end the string with a newline", {
      let fd = setup();
      io::console::println("ok");
      teardown(fd);

      let result =
        match io::file::open("tmp", io::stream::Read) {
          ~io::stream::Reader(r) => { let bytes = r.read(9); str::from_bytes([bytes[0], bytes[4], bytes[8]]) },
          _ => ~"bad"
        };

      must!(result eq ~"ok\n");
    })
  })
})
