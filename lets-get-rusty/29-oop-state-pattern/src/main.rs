use blog::Post;

fn main()
{
    let mut draft = Post::new();
    draft.add_text("I ate a salad for lunch today");

    let pending_review = draft.request_review();

    let approved = pending_review.approve();

    assert_eq!("I ate a salad for lunch today", approved.content());
}
