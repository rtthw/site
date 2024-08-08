#![doc = include_str!("../README.md")]


#[doc = include_str!("data/posts.md")]
pub mod posts {
    #[doc = include_str!("data/posts/how_i_made_this_site.md")]
    pub mod how_i_made_this_site {}
}

#[doc = include_str!("data/guide_to_rust.md")]
pub mod guide_to_rust {}
