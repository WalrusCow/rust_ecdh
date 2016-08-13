extern crate num;

mod field;
mod elliptic;

#[cfg(test)]
mod tests {
    use field;
    use num::BigUint;

    #[test]
    fn it_works() {
        let i = BigUint::from_bytes_be(&[
            0xA9, 0xFB, 0x57, 0xDB, 0xA1, 0xEE, 0xA9, 0xBC,
            0x3E, 0x66, 0x0A, 0x90, 0x9D, 0x83, 0x8D, 0x72,
            0x6E, 0x3B, 0xF6, 0x23, 0xD5, 0x26, 0x20, 0x28,
            0x20, 0x13, 0x48, 0x1D, 0x1F, 0x6E, 0x53, 0x77,
        ]);
        let gf = field::GF::new(&i);
        println!("{:x}", i);
        let x = gf.el(i);
        println!("{}", x);
    }
}
