extern crate paillier;
use paillier::*;

fn main() {
    println!(" to test paillier function!");

    let (ek, dk) = Paillier::keypair().keys();


    // encrypt four values
    let c1 = Paillier::encrypt(&ek, 10);
    let c2 = Paillier::encrypt(&ek, 20);
    let c3 = Paillier::encrypt(&ek, 30);
    let c4 = Paillier::encrypt(&ek, 40);

    // add all of them together
    let c = Paillier::add(&ek,
                          &Paillier::add(&ek, &c1, &c2),
                          &Paillier::add(&ek, &c3, &c4)
    );

    // multiply the sum by 2
    let d = Paillier::mul(&ek, &c, 2);

    // decrypt final result
    let m: u64 = Paillier::decrypt(&dk, &d);

    println!("decrypted total sum is {}", m);

    println!("test data 1");

    // encrypt two values
    let c1 = Paillier::encrypt(&ek, RawPlaintext::from(BigInt::from(20)));
    let c2 = Paillier::encrypt(&ek, RawPlaintext::from(BigInt::from(30)));

    // add all of them together
    let c = Paillier::add(&ek, c1, c2);

    // multiply the sum by 2
    let d = Paillier::mul(&ek, c, RawPlaintext::from(BigInt::from(2)));

    // decrypt final result
    let m: BigInt = Paillier::decrypt(&dk, d).into();
    println!("decrypted total sum is {}", m);

    let c1 = Paillier::encrypt(&ek, &*vec![1, 5, 10]);
    let c2 = Paillier::encrypt(&ek, &*vec![2, 10, 20]);
    let c3 = Paillier::encrypt(&ek, &*vec![3, 15, 30]);
    let c4 = Paillier::encrypt(&ek, &*vec![4, 20, 40]);

    // add up all four encryptions
    let c = Paillier::add(
        &ek,
        &Paillier::add(&ek, &c1, &c2),
        &Paillier::add(&ek, &c3, &c4),
    );

    let d = Paillier::mul(&ek, &c, 2);

    //
    // Decryption
    //

    let m = Paillier::decrypt(&dk, &c);
    let n = Paillier::decrypt(&dk, &d);
    println!("decrypted total sum is {:?}", m);
    println!("... and after multiplying {:?}", n);
    assert_eq!(m, vec![10, 50, 100]);
}
