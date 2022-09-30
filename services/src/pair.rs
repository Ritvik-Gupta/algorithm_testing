pub type Pair<A, B> = (A, B);

pub trait PairRefOper<A: Copy, B: Copy> {
    fn x(&self) -> A;
    fn y(&self) -> B;
}

impl<A: Copy, B: Copy> PairRefOper<A, B> for Pair<A, B> {
    fn x(&self) -> A {
        self.0
    }

    fn y(&self) -> B {
        self.1
    }
}

pub trait PairMutOper<A, B> {
    fn to_first(self) -> A;
    fn to_second(self) -> B;
}

impl<A, B> PairMutOper<A, B> for Pair<A, B> {
    fn to_first(self) -> A {
        self.0
    }

    fn to_second(self) -> B {
        self.1
    }
}
