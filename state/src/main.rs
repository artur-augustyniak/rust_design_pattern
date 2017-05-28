extern crate state;

use state::Post;


//A blog post starts as an empty draft.
//Once the draft is done, we request a review of the post.
//Once the post is approved, it gets published.
//Only published blog posts return content to print so that we can't accidentally print the text of a post that hasn't been approved.
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
