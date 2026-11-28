use meta_signal_flow::Continuity;
use rkyv::rancor::Error;

#[derive(rkyv::Archive, rkyv::Serialize)]
enum PreviousContinuity {
    OnDemand,
    Maintained,
}

#[test]
fn previous_positional_archive_discriminants_survive() {
    for (previous, current) in [
        (PreviousContinuity::OnDemand, Continuity::OneShot),
        (PreviousContinuity::Maintained, Continuity::Maintained),
    ] {
        let old = rkyv::to_bytes::<Error>(&previous).unwrap();
        let new = rkyv::to_bytes::<Error>(&current).unwrap();
        assert_eq!(old.as_slice(), new.as_slice());
        assert_eq!(rkyv::from_bytes::<Continuity, Error>(&old).unwrap(), current);
    }
}

#[cfg(feature = "datom")]
#[test]
fn on_demand_is_no_longer_configurable() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Compactable, Protosizable, ReaderBudget};
    let budget = || Budget {
        remaining: 4096,
        reader: ReaderBudget::try_from(4096).unwrap(),
        depth: 0,
        maximum_depth: 1024,
    };
    for (text, choice) in [
        ("OneShot", Continuity::OneShot),
        ("Maintained", Continuity::Maintained),
    ] {
        assert_eq!(
            Potential::<Continuity>::from(text)
                .actualize(&mut budget())
                .unwrap(),
            choice
        );
        assert_eq!(choice.datomize(vec![]).protosize().compact(), text);
    }
    assert!(
        Potential::<Continuity>::from("OnDemand")
            .actualize(&mut budget())
            .is_err()
    );
}
