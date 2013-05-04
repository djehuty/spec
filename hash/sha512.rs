use tester::*;

#[path="../tester.rs"]
mod tester;

mod hash{
  extern mod sha512;
}

describe!("sha512", {
  test!("hash_string", {
    should!("should hash an empty string", {
      let hash = hash::sha512::hash_string("").to_string();
      must!(hash eq ~"cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");
    })

    should!("should hash a string", {
      let hash = hash::sha512::hash_string("The quick brown fox jumps over the lazy dog").to_string();
      must!(hash eq ~"07e547d9586f6a73f73fbac0435ed76951218fb7d0c8d788a309d785436bbb642e93a252a954f23912547d1e8a3b5ed6e1bfd7097821233fa0538f3db854fee6");
    })
  })

  test!("hash", {
    should!("should hash an empty buffer", {
      let hash = hash::sha512::hash([]).to_string();
      must!(hash eq ~"cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");
    })
  })
})
