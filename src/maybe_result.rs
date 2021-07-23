use std::pin::Pin;

pub enum MaybeResult<F, T> {
    Future(F),
    Result(T),
    Gone,
}

impl<F, T> MaybeResult<F, T> {
    pub fn project(self: Pin<&mut Self>) -> MaybeResult<Pin<&mut F>, &mut T> {
        // safety: we only need to keep the `Future` pinned
        let this = unsafe { self.get_unchecked_mut() };

        match this {
            Self::Future(f) => {
                // safety: the `Future` variant is always pinned
                let f = unsafe { Pin::new_unchecked(f) };
                MaybeResult::Future(f)
            }
            Self::Result(t) => MaybeResult::Result(t),
            Self::Gone => MaybeResult::Gone,
        }
    }

    pub fn is_result(self: Pin<&mut Self>) -> bool {
        // safety: we don't actually access any of the fields
        let this = unsafe { self.get_unchecked_mut() };
        matches!(this, Self::Result(_))
    }

    pub fn set_result(self: Pin<&mut Self>, t: T) {
        // safety: we're throwing away the Future, so we no longer
        // need to be pinned
        let this = unsafe { self.get_unchecked_mut() };
        *this = Self::Result(t)
    }

    /// Panics if variant isn't `Self::Result<T>`
    pub fn take_result(self: Pin<&mut Self>) -> T {
        let this = unsafe { self.get_unchecked_mut() };
        if let Self::Result(_) = this {
            // okay good
        } else {
            panic!("trying to take Result when the variant isn't Result")
        }

        let mut alt = Self::Gone;
        std::mem::swap(this, &mut alt);
        if let Self::Result(t) = alt {
            t
        } else {
            unreachable!()
        }
    }
}
