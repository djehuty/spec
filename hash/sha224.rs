use tester::*;

#[path="../tester.rs"]
mod tester;

mod hash{
  extern mod sha224;
}

describe!("sha256", {
  test!("hash_string", {
    should!("should hash an empty string", {
      let hash = hash::sha224::hash_string("").to_string();
      must!(hash eq ~"d14a028c2a3a2bc9476102bb288234c415a2b01f828ea62ac5b3e42f");
    })

    should!("should hash a string", {
      let hash = hash::sha224::hash_string("The quick brown fox jumps over the lazy dog").to_string();
      must!(hash eq ~"730e109bd7a8a32b1cb9d9a09aa2325d2430587ddbc0c38bad911525");
    })
  })

  test!("hash", {
    should!("should hash an empty buffer", {
      let hash = hash::sha224::hash([]).to_string();
      must!(hash eq ~"d14a028c2a3a2bc9476102bb288234c415a2b01f828ea62ac5b3e42f");
    })
  })
})
