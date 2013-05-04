use tester::*;

#[path="../tester.rs"]
mod tester;

mod hash{
  extern mod sha384;
}

describe!("sha256", {
  test!("hash_string", {
    should!("should hash an empty string", {
      let hash = hash::sha384::hash_string("").to_string();
      must!(hash eq ~"38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b");
    })

    should!("should hash a string", {
      let hash = hash::sha384::hash_string("The quick brown fox jumps over the lazy dog").to_string();
      must!(hash eq ~"ca737f1014a48f4c0b6dd43cb177b0afd9e5169367544c494011e3317dbf9a509cb1e5dc1e85a941bbee3d7f2afbc9b1");
    })
  })

  test!("hash", {
    should!("should hash an empty buffer", {
      let hash = hash::sha384::hash([]).to_string();
      must!(hash eq ~"38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b");
    })
  })
})
