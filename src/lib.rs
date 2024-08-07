#![doc = include_str!("../README.md")]


#[doc = include_str!("data/posts.md")]
pub mod posts {
    #[doc = include_str!("data/posts/test.md")]
    pub mod test {}
}

#[doc = include_str!("data/learn_rust.md")]
pub mod learn_rust {}
