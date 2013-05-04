use tester::*;

#[path="../tester.rs"]
mod tester;

mod hash{
  extern mod sha1;
}

describe!("sha1", {
  test!("hash_string", {
    should!("should hash an empty string", {
      let hash = hash::sha1::hash_string("").to_string();
      must!(hash eq ~"da39a3ee5e6b4b0d3255bfef95601890afd80709");
    })

    should!("should hash a string", {
      let hash = hash::sha1::hash_string("The quick brown fox jumps over the lazy dog").to_string();
      must!(hash eq ~"2fd4e1c67a2d28fced849ee1bb76e7391b93eb12");
    })
  })

  test!("hash", {
    should!("should hash an empty buffer", {
      let hash = hash::sha1::hash([]).to_string();
      must!(hash eq ~"da39a3ee5e6b4b0d3255bfef95601890afd80709");
    })
  })
})
