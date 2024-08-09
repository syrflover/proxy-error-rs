extern crate proxy_error;

mod target {
    use super::parent;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("parent: {0}")]
        Parent(#[from] parent::Error),
    }

    impl Error {
        pub fn is<'a, T>(&'a self) -> bool
        where
            T: TryFrom<&'a Error, Error = bool>,
        {
            match T::try_from(self) {
                Ok(_) => unreachable!(),
                Err(r) => return r,
            }
        }

        pub fn into<T>(self) -> T
        where
            T: TryFrom<Error, Error = ()>,
        {
            match T::try_from(self) {
                Ok(r) => r,
                Err(_) => unreachable!(),
            }
        }
    }
}

mod parent {
    use proxy_error::proxy_error;

    use super::{child, target};

    #[derive(Debug, thiserror::Error)]
    #[proxy_error(target::Error, Parent)]
    pub enum Error {
        #[error("child: {0}")]
        Child(#[from] child::Error),
    }
}

mod child {
    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[allow(dead_code)]
        #[error("some error")]
        SomeError,
    }
}

#[test]
fn test() {
    let err = target::Error::Parent(parent::Error::Child(child::Error::SomeError));

    assert!(err.is::<child::Error>());

    assert!(matches!(
        err.into::<child::Error>(),
        child::Error::SomeError
    ));
}
