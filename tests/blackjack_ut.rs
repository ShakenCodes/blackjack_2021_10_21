#[cfg(test)]
use demonstrate::demonstrate;

#[cfg(test)]
demonstrate! {
    describe "In a new context" {
        use blackjack::*;
        use hamcrest2::prelude::*;

        it "succeeds" {
            assert_that!(sum(2, 2), eq(4));
        }
        #[should_panic]
        it "fails" {
            assert_that!(sum(2, 2), gt(5));
        }
    }
}
