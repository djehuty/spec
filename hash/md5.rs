use tester::*;

#[path="../tester.rs"]
mod tester;

mod hash {
  extern mod md5;
}

describe!("md5", {
  test!("hash_string", {
    should!("hash an empty string", {
      let hash = hash::md5::hash_string("").to_string();
      must!(hash eq ~"d41d8cd98f00b204e9800998ecf8427e");
    })

    should!("hash a string", {
      let hash = hash::md5::hash_string("The quick brown fox jumps over the lazy dog").to_string();
      must!(hash eq ~"9e107d9d372bb6826bd81d3542a419d6");
    })
  })

  test!("hash", {
    should!("hash an empty buffer", {
      let hash = hash::md5::hash([]).to_string();
      must!(hash eq ~"d41d8cd98f00b204e9800998ecf8427e");
    })
  })
})
