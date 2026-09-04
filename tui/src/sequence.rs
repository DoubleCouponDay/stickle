use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct Ready;

#[derive(Clone, Copy)]
pub struct Building;

#[derive(Clone, Copy)]
pub struct Passed;

#[derive(Clone, Copy)]
pub struct Failed;

#[derive(Clone, Copy)]
pub struct Cursor<S> {
    index: usize,
    length: usize,
    state: PhantomData<S>,
}

#[derive(Clone, Copy)]
pub enum Sequence {
    Ready(Cursor<Ready>),
    Building(Cursor<Building>),
    Passed(Cursor<Passed>),
    Failed(Cursor<Failed>),
}

impl<S> Cursor<S> {
    pub fn index(&self) -> usize {
        self.index
    }

    fn holding<T>(self) -> Cursor<T> {
        Cursor {
            index: self.index,
            length: self.length,
            state: PhantomData,
        }
    }

    fn advancing<T>(self) -> Cursor<T> {
        let index = match self.length {
            0 => 0,
            length => (self.index + 1) % length,
        };

        Cursor {
            index,
            length: self.length,
            state: PhantomData,
        }
    }
}

impl Cursor<Ready> {
    fn new(length: usize) -> Self {
        Cursor {
            index: 0,
            length,
            state: PhantomData,
        }
    }

    fn aimed(self, index: usize) -> Self {
        Cursor {
            index,
            length: self.length,
            state: PhantomData,
        }
    }

    fn started(self) -> Cursor<Building> {
        self.holding()
    }
}

impl Cursor<Building> {
    fn passed(self) -> Cursor<Passed> {
        self.advancing()
    }

    fn failed(self) -> Cursor<Failed> {
        self.holding()
    }
}

impl Cursor<Passed> {
    fn ready(self) -> Cursor<Ready> {
        self.holding()
    }
}

impl Cursor<Failed> {
    fn ready(self) -> Cursor<Ready> {
        self.holding()
    }
}

impl Sequence {
    pub fn new(length: usize) -> Self {
        Sequence::Ready(Cursor::new(length))
    }

    fn settled(self) -> Cursor<Ready> {
        match self {
            Sequence::Ready(cursor) => cursor,
            Sequence::Building(cursor) => cursor.holding(),
            Sequence::Passed(cursor) => cursor.ready(),
            Sequence::Failed(cursor) => cursor.ready(),
        }
    }

    pub fn armed(self) -> (Self, Option<usize>) {
        match self {
            Sequence::Building(cursor) => (Sequence::Building(cursor), None),
            settled => {
                let cursor = settled.settled();
                (Sequence::Ready(cursor), Some(cursor.index()))
            }
        }
    }

    pub fn started(self, index: usize) -> Self {
        match self {
            Sequence::Building(cursor) => Sequence::Building(cursor),
            settled => Sequence::Building(settled.settled().aimed(index).started()),
        }
    }

    pub fn finished(self, ok: bool) -> Self {
        match self {
            Sequence::Building(cursor) if ok => Sequence::Passed(cursor.passed()),
            Sequence::Building(cursor) => Sequence::Failed(cursor.failed()),
            settled => settled,
        }
    }
}
