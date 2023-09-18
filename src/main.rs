use num_bigint::BigInt;
use std::str::FromStr;

fn ecadd(a: (BigInt, BigInt), b: (BigInt, BigInt)) -> (BigInt, BigInt) {
    let pcurve = BigInt::parse_bytes(b"115792089237316195423570985008687907853269984665640564039457584007908834671663", 10).unwrap();
    let lam_add: BigInt = ((&b.1 - &a.1) * modinv(&(&b.0 - &a.0), &pcurve)) % (&pcurve);
    let x = ((&lam_add * &lam_add) - &a.0 - &b.0) % (&pcurve);
    let y = ((&lam_add * &(a.0 - &x)) - &a.1) % (&pcurve);

    let x_final = if x < BigInt::from_str("0").unwrap() { x + &pcurve } else { x };
    let y_final = if y < BigInt::from_str("0").unwrap() { y + &pcurve } else { y };

    return (x_final, y_final);

} 

fn ecdouble(a: (BigInt, BigInt)) -> (BigInt, BigInt) {
    let pcurve = BigInt::parse_bytes(b"115792089237316195423570985008687907853269984665640564039457584007908834671663", 10).unwrap();
    let acurve: BigInt = BigInt::from_str("0").unwrap();
    let lam_double: BigInt = ((3 * &a.0 * &a.0) + &acurve) * modinv(&(&BigInt::from(2) * &a.1), &pcurve) % (&pcurve);
    let x: BigInt = (&lam_double * &lam_double - 2 * &a.0) % (&pcurve);
    let y: BigInt = (&lam_double * (&a.0 - x.clone()) - &a.1) % (&pcurve);

    let x_final = if x < BigInt::from_str("0").unwrap() { x + &pcurve } else { x };
    let y_final = if y < BigInt::from_str("0").unwrap() { y + &pcurve } else { y };

    return (x_final, y_final);
}

fn modinv(a0: &BigInt, m0: &BigInt) -> BigInt {
    let one: BigInt = BigInt::from(1);
    let mut a = a0.clone();
    let mut m = m0.clone();
    let mut x0 = BigInt::from(0);
    let mut inv = BigInt::from(1);

    while a > one {
        inv -= (&a / &m) * &x0;
        a %= &m;
        std::mem::swap(&mut a, &mut m);
        std::mem::swap(&mut x0, &mut inv);
    }
    if inv < BigInt::from(0) {
        inv += m0;
    }
    inv
}

fn main() {

    let generator = (BigInt::parse_bytes(b"55066263022277343669578718895168534326250603453777594175500187360389116729240", 10).unwrap(), BigInt::parse_bytes(b"32670510020758816978083085130507043184471273380659243275938904335757337482424", 10).unwrap());
    
    // Precomputed points on the curve, from 2G to 12G:
    
    let _2_g_pre = (BigInt::from_str("89565891926547004231252920425935692360644145829622209833684329913297188986597").unwrap(), BigInt::from_str("12158399299693830322967808612713398636155367887041628176798871954788371653930").unwrap());
    let _3_g_pre = (BigInt::from_str("112711660439710606056748659173929673102114977341539408544630613555209775888121").unwrap(), BigInt::from_str("25583027980570883691656905877401976406448868254816295069919888960541586679410").unwrap());
    let _4_g_pre = (BigInt::from_str("103388573995635080359749164254216598308788835304023601477803095234286494993683").unwrap(), BigInt::from_str("37057141145242123013015316630864329550140216928701153669873286428255828810018").unwrap());
    let _5_g_pre = (BigInt::from_str("21505829891763648114329055987619236494102133314575206970830385799158076338148").unwrap(), BigInt::from_str("98003708678762621233683240503080860129026887322874138805529884920309963580118").unwrap());
    let _6_g_pre = (BigInt::from_str("115780575977492633039504758427830329241728645270042306223540962614150928364886").unwrap(), BigInt::from_str("78735063515800386211891312544505775871260717697865196436804966483607426560663").unwrap());
    let _7_g_pre = (BigInt::from_str("41948375291644419605210209193538855353224492619856392092318293986323063962044").unwrap(), BigInt::from_str("48361766907851246668144012348516735800090617714386977531302791340517493990618").unwrap());
    let _8_g_pre = (BigInt::from_str("21262057306151627953595685090280431278183829487175876377991189246716355947009").unwrap(), BigInt::from_str("41749993296225487051377864631615517161996906063147759678534462689479575333124").unwrap());
    let _9_g_pre = (BigInt::from_str("78173298682877769088723994436027545680738210601369041078747105985693655485630").unwrap(), BigInt::from_str("92362876758821804597230797234617159328445543067760556585160674174871431781431").unwrap());
    let _10_g_pre = (BigInt::from_str("72488970228380509287422715226575535698893157273063074627791787432852706183111").unwrap(), BigInt::from_str("62070622898698443831883535403436258712770888294397026493185421712108624767191").unwrap());
    let _11_g_pre = (BigInt::from_str("53957576663012291606402345341061437133522758407718089353314528343643821967563").unwrap(), BigInt::from_str("98386217607324929854432842186271083758341411730506808463586570492533445740059").unwrap());
    let _12_g_pre = (BigInt::from_str("94111259592240215275188773285036844871058226277992966241101117022315524122714").unwrap(), BigInt::from_str("76870767327212528811304566602812752860184934880685532702451763239157141742375").unwrap());

    // Verification eccadd function:

    let _3_g = ecadd(generator.clone(), _2_g_pre.clone()); // Computing 3G
    let _4_g = ecadd(generator.clone(), _3_g_pre.clone()); // Computing 4G
    let _5_g = ecadd(generator.clone(), _4_g_pre.clone()); // Computing 5G
    let _6_g = ecadd(_2_g_pre.clone(), _4_g_pre.clone()); // Computing 6G
    let _7_g = ecadd(generator.clone(), _6_g_pre.clone()); // Computing 7G
    let _8_g = ecadd(_7_g_pre.clone(), generator.clone()); // Computing 8G
    let _9_g = ecadd(_5_g_pre.clone(), _4_g_pre.clone()); // Computing 9G
    let _10_g = ecadd(_7_g_pre.clone(), _3_g_pre.clone()); // Computing 10G
    let _11_g = ecadd(_8_g_pre.clone(), _3_g_pre.clone()); // Computing 11G
    let _12_g = ecadd(_9_g_pre.clone(), _3_g_pre.clone()); // Computing 12G

    assert_eq!(_3_g_pre, _3_g.clone()); // Checking 3G
    assert_eq!(_4_g_pre, _4_g.clone()); // Checking 4G
    assert_eq!(_5_g_pre, _5_g.clone()); // Checking 5G
    assert_eq!(_6_g_pre, _6_g.clone()); // Checking 6G
    assert_eq!(_7_g_pre, _7_g.clone()); // Checking 7G
    assert_eq!(_8_g_pre, _8_g.clone()); // Checking 8G
    assert_eq!(_9_g_pre, _9_g.clone()); // Checking 9G
    assert_eq!(_10_g_pre, _10_g.clone()); // Checking 10G
    assert_eq!(_11_g_pre, _11_g.clone()); // Checking 11G
    assert_eq!(_12_g_pre, _12_g.clone()); // Checking 12G

    // Verification ecdouble function:

    let (_2_g_x, _) = _2_g_pre.clone(); // X-coordinates of 2G
    let (_, _2_g_y) = _2_g_pre.clone(); // Y-coordinates of 2G

    let (_3_g_x, _) = _3_g_pre.clone(); // X-coordinates of 3G
    let (_, _3_g_y) = _3_g_pre.clone(); // Y-coordinates of 3G

    let (_4_g_x, _) = _4_g_pre.clone(); // X-coordinates of 4G
    let (_, _4_g_y) = _4_g_pre.clone(); // Y-coordinates of 4G

    let (_5_g_x, _) = _5_g_pre.clone(); // X-coordinates of 5G
    let (_, _5_g_y) = _5_g_pre.clone(); // Y-coordinates of 5G

    let (_6_g_x, _) = _6_g_pre.clone(); // X-coordinates of 6G
    let (_, _6_g_y) = _6_g_pre.clone(); // Y-coordinates of 6G

    // Doubling points:

    let _4g = ecdouble((_2_g_x, _2_g_y)); // Doubling 2G, computing 4G
    let _6g = ecdouble((_3_g_x, _3_g_y)); // Doubling 3G, computing 6G
    let _8g = ecdouble((_4_g_x, _4_g_y)); // Doubling 4G, computing 8G
    let _10g = ecdouble((_5_g_x, _5_g_y)); // Doubling 5G, computing 10G
    let _12g = ecdouble((_6_g_x, _6_g_y)); // Doubling 6G, computing 12G

    assert_eq!(_4_g_pre, _4g); // Checking 4G
    assert_eq!(_6_g_pre, _6g); // Checking 6G
    assert_eq!(_8_g_pre, _8g); // Checking 8G
    assert_eq!(_10_g_pre, _10g); // Checking 10G
    assert_eq!(_12_g_pre, _12g); // Checking 12G

}
