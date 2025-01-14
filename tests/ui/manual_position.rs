#![warn(clippy::manual_position)]
#![allow(clippy::needless_return)]

fn main() {
    unwrap_test();
    expect_test();
    try_test();
    map_test();
    find_test();
    deref_test();
    rposition_test();
}

fn expect_test() {
    let v = [1, 2, 3, 5, 6];
    let _ = v.iter().enumerate().find(|(_, &ch)| ch == 3).expect("").0;
    let _ = v.iter().enumerate().find(|(_, &ch)| ch == 3).expect("").1;
}

fn unwrap_test() {
    let v = [1, 2, 3, 5, 6];
    let _ = v.iter().enumerate().find(|(_, &ch)| ch == 3).unwrap().0;
    let _ = v.iter().enumerate().find(|(_, &ch)| ch == 3).unwrap().1;
}

fn try_test() -> Option<()> {
    let v = [1, 2, 3, 5, 6];
    let _ = v.iter().enumerate().find(|(_, &ch)| ch == 3)?.0;
    let _ = v.iter().enumerate().find(|(_, &ch)| ch == 3)?.1;
    Some(())
}

fn map_test() {
    let v = [1, 2, 3, 5, 6];
    let _ = v.iter().enumerate().find(|&(_, &ch)| ch == 3).map(|c| c.0);
    #[rustfmt::skip]
    let _ = v.iter().enumerate().find(|(_, &ch)| ch == 3).map(|c| { return c.0  });
    let _ = v.iter().enumerate().find(|(_, &ch)| ch == 3).map(|(p, _)| p);
    let _ = v.iter().enumerate().find(|(_, &ch)| ch == 3).map(|(p, ..)| {
        return p;
    });

    // Only trigger when returning the first field of the result
    let x = (1, 2);
    let _ = v.iter().enumerate().find(|(_, &ch)| ch == 3).map(|_| x.0);
}

fn find_test() {
    let v = [1, 2, 3, 5, 6];
    let _ = v.iter().enumerate().find(|(_, &ch)| ch == 3).unwrap().0;
    // Doesn't trigger when the predicate use the index
    let _ = v.iter().enumerate().find(|&(i, _)| i >= 1).unwrap().0;
    let _ = v.iter().enumerate().find(|&(i, &ch)| ch == 3 && i >= 1).unwrap().0;
}

fn deref_test() {
    let v = [1, 2, 3, 5, 6];
    let _ = v.iter().enumerate().find(|(_, &ch)| ch == 3).unwrap().0;
    let _ = v.iter().enumerate().find(|&(_, &ch)| ch == 3).unwrap().0;
    let _ = v.iter().enumerate().find(|&(_, ch)| *ch == 3).unwrap().0;
    // let _ = v.iter().enumerate().find(|(_, ch)| **ch == 3).unwrap().0;
    // Sugestion make:
    // let _ = v.iter().position(|ch|**ch == 3).unwrap();
    // but need to be:
    // let _ = v.iter().position(|ch|*ch == 3).unwrap();
    // or
    // let _ = v.iter().position(|&ch|ch == 3).unwrap();
}

fn rposition_test() {
    let v = [1, 2, 3, 5, 3, 6];
    let _ = v.iter().enumerate().rev().find(|(_, &ch)| ch == 3).unwrap().0;
}
