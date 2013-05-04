use tester::*;

#[path="../tester.rs"]
mod tester;

mod hash{
  extern mod sha256;
}

describe!("sha256", {
  test!("hash_string", {
    should!("should hash an empty string", {
      let hash = hash::sha256::hash_string("").to_string();
      must!(hash eq ~"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    })

    should!("should hash a string", {
      let hash = hash::sha256::hash_string("The quick brown fox jumps over the lazy dog").to_string();
      must!(hash eq ~"d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592");
    })
  })

  test!("hash", {
    should!("should hash an empty buffer", {
      let hash = hash::sha256::hash([]).to_string();
      must!(hash eq ~"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    })
  })
})
