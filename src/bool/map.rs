use crate::map::Map;

impl Map for bool {
    fn ok(self) -> Result<Self, Self> {
        if self { Ok(true) } else { Err(false) }
    }

    fn ok_or<E>(self, err: E) -> Result<Self, E> {
        if self { Ok(true) } else { Err(err) }
    }

    fn map<T>(self, f: impl FnOnce() -> T) -> Option<T> {
        if self { Some(f()) } else { None }
    }

    fn unwrap(self) -> Self {
        self.expect("Expected 'true', got 'false'")
    }

    fn expect(self, message: impl std::fmt::Display) -> Self {
        if self { self } else { panic!("{message}") }
    }
}
