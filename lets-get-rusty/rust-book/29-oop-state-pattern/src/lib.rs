pub struct Post {
    content: String
}

pub struct DraftPost {
    content: String
}

impl Post {
    pub fn new() -> DraftPost
    {
        DraftPost { content: String::new() }
    }

    pub fn content(&self) -> &str
    {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str)
    {
        self.content.push_str(text);
    }

    // Takes ownership of self. Means it will consume and invalidate the old
    // state and return the new state
    pub fn request_review(self) -> PendingReviewPost
    {
        PendingReviewPost { content: self.content }
    }
}

pub struct PendingReviewPost {
    content: String
}

impl PendingReviewPost {
    // Takes ownership of self. Means it will consume and invalidate the old
    // state and return the new state
    pub fn approve(self) -> Post
    {
        Post { content: self.content }
    }
}
