#![allow(warnings)]

use std::panic::{RefUnwindSafe, UnwindSafe};

use expect_test::expect;
use salsa::{Db, Durability};

// Axes:
//
// Threading
// * Intra-thread
// * Cross-thread -- part of cycle is on one thread, part on another
//
// Recovery strategies:
// * Panic
// * Fallback
// * Mixed -- multiple strategies within cycle participants
//
// Across revisions:
// * N/A -- only one revision
// * Present in new revision, not old
// * Present in old revision, not new
// * Present in both revisions
//
// Dependencies
// * Tracked
// * Untracked -- cycle participant(s) contain untracked reads
//
// Layers
// * Direct -- cycle participant is directly invoked from test
// * Indirect -- invoked a query that invokes the cycle
//
//
// | Thread | Recovery | Old, New | Dep style | Layers   | Test Name      |
// | ------ | -------- | -------- | --------- | ------   | ---------      |
// | Intra  | Panic    | N/A      | Tracked   | direct   | cycle_memoized |
// | Intra  | Panic    | N/A      | Untracked | direct   | cycle_volatile |
// | Intra  | Fallback | N/A      | Tracked   | direct   | cycle_cycle  |
// | Intra  | Fallback | N/A      | Tracked   | indirect | inner_cycle |
// | Intra  | Fallback | Both     | Tracked   | direct   | cycle_revalidate |
// | Intra  | Fallback | New      | Tracked   | direct   | cycle_appears |
// | Intra  | Fallback | Old      | Tracked   | direct   | cycle_disappears |
// | Intra  | Fallback | Old      | Tracked   | direct   | cycle_disappears_durability |
// | Intra  | Mixed    | N/A      | Tracked   | direct   | cycle_mixed_1 |
// | Intra  | Mixed    | N/A      | Tracked   | direct   | cycle_mixed_2 |
// | Cross  | Panic    | N/A      | Tracked   | both     | parallel/parallel_cycle_none_recover.rs |
// | Cross  | Fallback | N/A      | Tracked   | both     | parallel/parallel_cycle_one_recover.rs |
// | Cross  | Fallback | N/A      | Tracked   | both     | parallel/parallel_cycle_mid_recover.rs |
// | Cross  | Fallback | N/A      | Tracked   | both     | parallel/parallel_cycle_all_recover.rs |

// TODO: The following test is not yet ported.
// | Intra  | Fallback | Old      | Tracked   | direct   | cycle_disappears_durability |

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Error {
    cycle: Vec<String>,
}

#[salsa::jar]
struct Jar(
    MyInput,
    memoized_a,
    memoized_b,
    volatile_a,
    volatile_b,
    ABC,
    cycle_a,
    cycle_b,
    cycle_c,
);

#[salsa::db(Jar)]
#[derive(Default)]
struct Database;

impl RefUnwindSafe for Database {}

#[salsa::input(db = Db, jar = Jar)]
struct MyInput {}

#[salsa::tracked(jar = Jar)]
fn memoized_a(db: &Db, input: MyInput) {
    memoized_b(db, input)
}

#[salsa::tracked(jar = Jar)]
fn memoized_b(db: &Db, input: MyInput) {
    memoized_a(db, input)
}

#[salsa::tracked(jar = Jar)]
fn volatile_a(db: &Db, input: MyInput) {
    db.report_untracked_read();
    volatile_b(db, input)
}

#[salsa::tracked(jar = Jar)]
fn volatile_b(db: &Db, input: MyInput) {
    db.report_untracked_read();
    volatile_a(db, input)
}

/// The queries A, B, and C in `Database` can be configured
/// to invoke one another in arbitrary ways using this
/// enum.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum CycleQuery {
    None,
    A,
    B,
    C,
    AthenC,
}

#[salsa::input(db = Db, jar = Jar)]
struct ABC {
    a: CycleQuery,
    b: CycleQuery,
    c: CycleQuery,
}

impl CycleQuery {
    fn invoke(self, db: &Db, abc: ABC) -> Result<(), Error> {
        match self {
            CycleQuery::A => cycle_a(db, abc),
            CycleQuery::B => cycle_b(db, abc),
            CycleQuery::C => cycle_c(db, abc),
            CycleQuery::AthenC => {
                let _ = cycle_a(db, abc);
                cycle_c(db, abc)
            }
            CycleQuery::None => Ok(()),
        }
    }
}

#[salsa::tracked(jar = Jar, recovery_fn=recover_a)]
fn cycle_a(db: &Db, abc: ABC) -> Result<(), Error> {
    abc.a(db).invoke(db, abc)
}

fn recover_a(db: &Db, cycle: &salsa::Cycle, abc: ABC) -> Result<(), Error> {
    Err(Error {
        cycle: cycle.all_participants(db),
    })
}

#[salsa::tracked(jar = Jar, recovery_fn=recover_b)]
fn cycle_b(db: &Db, abc: ABC) -> Result<(), Error> {
    abc.b(db).invoke(db, abc)
}

fn recover_b(db: &Db, cycle: &salsa::Cycle, abc: ABC) -> Result<(), Error> {
    Err(Error {
        cycle: cycle.all_participants(db),
    })
}

#[salsa::tracked(jar = Jar)]
fn cycle_c(db: &Db, abc: ABC) -> Result<(), Error> {
    abc.c(db).invoke(db, abc)
}

#[track_caller]
fn extract_cycle(f: impl FnOnce() + UnwindSafe) -> salsa::Cycle {
    let v = std::panic::catch_unwind(f);
    if let Err(d) = &v {
        if let Some(cycle) = d.downcast_ref::<salsa::Cycle>() {
            return cycle.clone();
        }
    }
    panic!("unexpected value: {:?}", v)
}

#[test]
fn cycle_memoized() {
    let mut db = Database::default();
    let input = MyInput::new(&db, salsa::Durability::LOW);
    let cycle = extract_cycle(|| memoized_a(&db, input));
    let expected = expect![[r#"
        [
            "memoized_a(0)",
            "memoized_b(0)",
        ]
    "#]];
    expected.assert_debug_eq(&cycle.all_participants(&db));
}

#[test]
fn cycle_volatile() {
    let mut db = Database::default();
    let input = MyInput::new(&db, salsa::Durability::LOW);
    let cycle = extract_cycle(|| volatile_a(&db, input));
    let expected = expect![[r#"
        [
            "volatile_a(0)",
            "volatile_b(0)",
        ]
    "#]];
    expected.assert_debug_eq(&cycle.all_participants(&db));
}

#[test]
fn expect_cycle() {
    //     A --> B
    //     ^     |
    //     +-----+

    let mut db = Database::default();
    let abc = ABC::new(
        &db,
        CycleQuery::B,
        CycleQuery::A,
        CycleQuery::None,
        salsa::Durability::LOW,
    );
    assert!(cycle_a(&db, abc).is_err());
}

#[test]
fn inner_cycle() {
    //     A --> B <-- C
    //     ^     |
    //     +-----+
    let mut db = Database::default();
    let abc = ABC::new(
        &db,
        CycleQuery::B,
        CycleQuery::A,
        CycleQuery::B,
        salsa::Durability::LOW,
    );
    let err = cycle_c(&db, abc);
    assert!(err.is_err());
    let expected = expect![[r#"
        [
            "cycle_a(0)",
            "cycle_b(0)",
        ]
    "#]];
    expected.assert_debug_eq(&err.unwrap_err().cycle);
}

#[test]
fn cycle_revalidate() {
    //     A --> B
    //     ^     |
    //     +-----+
    let mut db = Database::default();
    let abc = ABC::new(
        &db,
        CycleQuery::B,
        CycleQuery::A,
        CycleQuery::None,
        salsa::Durability::LOW,
    );
    assert!(cycle_a(&db, abc).is_err());
    abc.set_b(salsa::Durability::LOW, &mut db).to(CycleQuery::A); // same value as default
    assert!(cycle_a(&db, abc).is_err());
}

#[test]
fn cycle_recovery_unchanged_twice() {
    //     A --> B
    //     ^     |
    //     +-----+
    let mut db = Database::default();
    let abc = ABC::new(
        &db,
        CycleQuery::B,
        CycleQuery::A,
        CycleQuery::None,
        salsa::Durability::LOW,
    );
    assert!(cycle_a(&db, abc).is_err());

    abc.set_c(salsa::Durability::LOW, &mut db).to(CycleQuery::A); // force new revision
    assert!(cycle_a(&db, abc).is_err());
}

#[test]
fn cycle_appears() {
    let mut db = Database::default();

    //     A --> B
    let abc = ABC::new(
        &db,
        CycleQuery::B,
        CycleQuery::None,
        CycleQuery::None,
        salsa::Durability::LOW,
    );
    assert!(cycle_a(&db, abc).is_ok());

    //     A --> B
    //     ^     |
    //     +-----+
    abc.set_b(salsa::Durability::LOW, &mut db).to(CycleQuery::A);
    assert!(cycle_a(&db, abc).is_err());
}

#[test]
fn cycle_disappears() {
    let mut db = Database::default();

    //     A --> B
    //     ^     |
    //     +-----+
    let abc = ABC::new(
        &db,
        CycleQuery::B,
        CycleQuery::A,
        CycleQuery::None,
        salsa::Durability::LOW,
    );
    assert!(cycle_a(&db, abc).is_err());

    //     A --> B
    abc.set_b(salsa::Durability::LOW, &mut db)
        .to(CycleQuery::None);
    assert!(cycle_a(&db, abc).is_ok());
}

/// A variant on `cycle_disappears` in which the values of
/// `a` and `b` are set with durability values.
/// If we are not careful, this could cause us to overlook
/// the fact that the cycle will no longer occur.
#[test]
fn cycle_disappears_durability() {
    let mut db = Database::default();
    let abc = ABC::new(
        &mut db,
        CycleQuery::None,
        CycleQuery::None,
        CycleQuery::None,
        salsa::Durability::LOW,
    );
    abc.set_a(salsa::Durability::LOW, &mut db).to(CycleQuery::B);
    abc.set_b(salsa::Durability::HIGH, &mut db)
        .to(CycleQuery::A);

    assert!(cycle_a(&db, abc).is_err());

    // At this point, `a` read `LOW` input, and `b` read `HIGH` input. However,
    // because `b` participates in the same cycle as `a`, its final durability
    // should be `LOW`.
    //
    // Check that setting a `LOW` input causes us to re-execute `b` query, and
    // observe that the cycle goes away.
    abc.set_a(salsa::Durability::LOW, &mut db)
        .to(CycleQuery::None);

    assert!(cycle_b(&mut db, abc).is_ok());
}

#[test]
fn cycle_disappears_durability2() {
    let mut db = Database::default();
    let abc = ABC::new(
        &mut db,
        CycleQuery::None,
        CycleQuery::None,
        CycleQuery::None,
        salsa::Durability::LOW,
    );
    abc.set_a(salsa::Durability::MEDIUM, &mut db)
        .to(CycleQuery::B);
    abc.set_b(salsa::Durability::HIGH, &mut db)
        .to(CycleQuery::A);

    assert!(cycle_a(&db, abc).is_err());

    // At this point, `a` read `LOW` input, and `b` read `HIGH` input. However,
    // because `b` participates in the same cycle as `a`, its final durability
    // should be `LOW`.
    //
    // Check that setting a `LOW` input causes us to re-execute `b` query, and
    // observe that the cycle goes away.
    abc.set_a(salsa::Durability::MEDIUM, &mut db)
        .to(CycleQuery::None);

    assert!(cycle_b(&mut db, abc).is_ok());
}

#[test]
fn cycle_mixed_1() {
    let mut db = Database::default();

    //     A --> B <-- C
    //           |     ^
    //           +-----+
    let abc = ABC::new(
        &db,
        CycleQuery::B,
        CycleQuery::C,
        CycleQuery::B,
        salsa::Durability::LOW,
    );

    let expected = expect![[r#"
        [
            "cycle_b(0)",
            "cycle_c(0)",
        ]
    "#]];
    expected.assert_debug_eq(&cycle_c(&db, abc).unwrap_err().cycle);
}

#[test]
fn cycle_mixed_2() {
    let mut db = Database::default();

    // Configuration:
    //
    //     A --> B --> C
    //     ^           |
    //     +-----------+
    let abc = ABC::new(
        &db,
        CycleQuery::B,
        CycleQuery::C,
        CycleQuery::A,
        salsa::Durability::LOW,
    );
    let expected = expect![[r#"
        [
            "cycle_a(0)",
            "cycle_b(0)",
            "cycle_c(0)",
        ]
    "#]];
    expected.assert_debug_eq(&cycle_a(&db, abc).unwrap_err().cycle);
}

#[test]
fn cycle_deterministic_order() {
    // No matter whether we start from A or B, we get the same set of participants:
    let f = || {
        let mut db = Database::default();

        //     A --> B
        //     ^     |
        //     +-----+
        let abc = ABC::new(
            &db,
            CycleQuery::B,
            CycleQuery::A,
            CycleQuery::None,
            salsa::Durability::LOW,
        );
        (db, abc)
    };
    let (db, abc) = f();
    let a = cycle_a(&db, abc);
    let (db, abc) = f();
    let b = cycle_b(&db, abc);
    let expected = expect![[r#"
        (
            [
                "cycle_a(0)",
                "cycle_b(0)",
            ],
            [
                "cycle_a(0)",
                "cycle_b(0)",
            ],
        )
    "#]];
    expected.assert_debug_eq(&(a.unwrap_err().cycle, b.unwrap_err().cycle));
}

#[test]
fn cycle_multiple() {
    // No matter whether we start from A or B, we get the same set of participants:
    let mut db = Database::default();

    // Configuration:
    //
    //     A --> B <-- C
    //     ^     |     ^
    //     +-----+     |
    //           |     |
    //           +-----+
    //
    // Here, conceptually, B encounters a cycle with A and then
    // recovers.
    let abc = ABC::new(
        &db,
        CycleQuery::B,
        CycleQuery::AthenC,
        CycleQuery::A,
        salsa::Durability::LOW,
    );

    let c = cycle_c(&db, abc);
    let b = cycle_b(&db, abc);
    let a = cycle_a(&db, abc);
    let expected = expect![[r#"
        (
            [
                "cycle_a(0)",
                "cycle_b(0)",
            ],
            [
                "cycle_a(0)",
                "cycle_b(0)",
            ],
            [
                "cycle_a(0)",
                "cycle_b(0)",
            ],
        )
    "#]];
    expected.assert_debug_eq(&(
        c.unwrap_err().cycle,
        b.unwrap_err().cycle,
        a.unwrap_err().cycle,
    ));
}

#[test]
fn cycle_recovery_set_but_not_participating() {
    let mut db = Database::default();

    //     A --> C -+
    //           ^  |
    //           +--+
    let abc = ABC::new(
        &db,
        CycleQuery::C,
        CycleQuery::None,
        CycleQuery::C,
        salsa::Durability::LOW,
    );

    // Here we expect C to panic and A not to recover:
    let r = extract_cycle(|| drop(cycle_a(&db, abc)));
    let expected = expect![[r#"
        [
            "cycle_c(0)",
        ]
    "#]];
    expected.assert_debug_eq(&r.all_participants(&db));
}
