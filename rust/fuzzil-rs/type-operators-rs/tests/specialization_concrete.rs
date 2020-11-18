#![cfg_attr(feature = "specialization", feature(specialization))]

#[macro_use]
extern crate type_operators;

#[cfg(feature = "specialization")]
mod test {
    type_operators! {
        [A, B, C, D, E]

        concrete Nat: Default => usize where #[derive(Debug, Default)] {
            P => 0,
            I(N: Nat = P) => 1 + 2 * N,
            O(N: Nat = P) => 2 * N,
            Undefined => panic!("Undefined type-level arithmetic result!"),
            Error => panic!("Error: Attempted to perform arithmetic on types which do not represent numbers!"),
            DEFAULT => panic!("This is not a number!"),
        }

        concrete Bool => bool {
            False => false,
            True => true,
            DEFAULT => panic!("This is not a boolean!"),
        }

        (Pred) Predecessor(Nat): Nat {
            [Undefined] => Undefined
            [P] => Undefined
            forall (N: Nat) {
                [(O N)] => (I (# N))
                [(I N)] => (O N)
                {N} => Error
            }
        }

        (Succ) Successor(Nat): Nat {
            [Undefined] => Undefined
            [P] => I
            forall (N: Nat) {
                [(O N)] => (I N)
                [(I N)] => (O (# N))
                {N} => Error
            }
        }

        (Sum) Adding(Nat, Nat): Nat {
            [P, P] => P
            forall (N: Nat) {
                [(O N), P] => (O N)
                [(I N), P] => (I N)
                [P, (O N)] => (O N)
                [P, (I N)] => (I N)
            }
            forall (N: Nat, M: Nat) {
                [(O M), (O N)] => (O (# M N))
                [(I M), (O N)] => (I (# M N))
                [(O M), (I N)] => (I (# M N))
                [(I M), (I N)] => (O (# (# M N) I))
                {M, N} => Error
            }
        }

        (Difference) Subtracting(Nat, Nat): Nat {
            forall (N: Nat) {
                [N, P] => N
            }
            forall (N: Nat, M: Nat) {
                [(O M), (O N)] => (O (# M N))
                [(I M), (O N)] => (I (# M N))
                [(O M), (I N)] => (I (# (# M N) I))
                [(I M), (I N)] => (O (# M N))
                {M, N} => Error
            }
        }

        (Product) Multiplying(Nat, Nat): Nat {
            forall (N: Nat) {
                [P, N] => P
            }
            forall (N: Nat, M: Nat) {
                [(O M), N] => (# M (O N))
                [(I M), N] => (@Adding N (# M (O N)))
                {M, N} => Error
            }
        }

        (If) NatIf(Bool, Nat, Nat): Nat {
            forall (T: Nat, U: Nat) {
                [True, T, U] => T
                [False, T, U] => U
            }
            forall (B: Bool, T: Nat, U: Nat) {
                {B, T, U} => Error
            }
        }

        (NatIsUndef) NatIsUndefined(Nat): Bool {
            [Undefined] => True
            [P] => False
            forall (M: Nat) {
                [(O M)] => False
                [(I M)] => False
                {M} => Error
            }
        }

        (NatUndef) NatUndefined(Nat, Nat): Nat {
            forall (M: Nat) {
                [Undefined, M] => Undefined
                [P, M] => M
            }
            forall (M: Nat, N: Nat) {
                [(O N), M] => M
                [(I N), M] => M
                {M, N} => Error
            }
        }

        (TotalDifference) TotalSubtracting(Nat, Nat): Nat {
            [P, P] => P
            [Undefined, P] => Undefined
            forall (N: Nat) {
                [N, Undefined] => Undefined
                [P, (O N)] => (# P N)
                [P, (I N)] => Undefined
                [(O N), P] => (O N)
                [(I N), P] => (I N)
                [Undefined, (O N)] => Undefined
                [Undefined, (I N)] => Undefined
            }
            forall (N: Nat, M: Nat) {
                [(O M), (O N)] => (@NatUndefined (# M N) (O (# M N)))
                [(I M), (O N)] => (@NatUndefined (# M N) (I (# M N)))
                [(O M), (I N)] => (@NatUndefined (# (# M N) I) (I (# (# M N) I)))
                [(I M), (I N)] => (@NatUndefined (# M N) (O (# M N)))
                {M, N} => Error
            }
        }

        (Quotient) Quotienting(Nat, Nat): Nat {
            forall (D: Nat) {
                [Undefined, D] => Undefined
                [P, D] => (@NatIf (@NatIsUndefined (@TotalSubtracting P D)) O (@Successor (# (@TotalSubtracting P D) D)))
            }
            forall (N: Nat, D: Nat) {
                [(O N), D] => (@NatIf (@NatIsUndefined (@TotalSubtracting (O N) D)) O (@Successor (# (@TotalSubtracting (O N) D) D)))
                [(I N), D] => (@NatIf (@NatIsUndefined (@TotalSubtracting (I N) D)) O (@Successor (# (@TotalSubtracting (I N) D) D)))
                {N, D} => Error
            }
        }

        (Remainder) Remaindering(Nat, Nat): Nat {
            forall (D: Nat) {
                [Undefined, D] => Undefined
                [P, D] => (@NatIf (@NatIsUndefined (@TotalSubtracting P D)) P (# (@TotalSubtracting P D) D))
            }
            forall (N: Nat, D: Nat) {
                [(O N), D] => (@NatIf (@NatIsUndefined (@TotalSubtracting (O N) D)) (O N) (# (@TotalSubtracting (O N) D) D))
                [(I N), D] => (@NatIf (@NatIsUndefined (@TotalSubtracting (I N) D)) (I O) (# (@TotalSubtracting (I N) D) D))
                {N, D} => Error
            }
        }
    }

    #[test]
    fn invariants() {
        assert_eq!(<I<I> as Nat>::reify(), 3);
        assert_eq!(<I<O<I>> as Nat>::reify(), 5);
        assert_eq!(<Sum<I<O<I>>, I<I>> as Nat>::reify(), 8);
        assert_eq!(<Difference<I<I>, O<I>> as Nat>::reify(), 1);
        assert_eq!(<Difference<O<O<O<I>>>, I<I>> as Nat>::reify(), 5);
        assert_eq!(<Product<I<I>, I<O<I>>> as Nat>::reify(), 15);
        assert_eq!(<Quotient<I<I>, O<I>> as Nat>::reify(), 1);
        assert_eq!(<Remainder<I<O<O<I>>>, O<O<I>>> as Nat>::reify(), 1);
    }

    #[test]
    #[should_panic]
    fn blanketed_1() {
        let _ = <usize as Nat>::reify();
    }

    #[test]
    #[should_panic]
    fn blanketed_2() {
        let _ = <Sum<u32, u64> as Nat>::reify();
    }
}
