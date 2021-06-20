mod post;

use post::post::{Post};


fn main() {
    // create a Post
    let mut my_post = Post::new();
    // add text to the Post
    my_post.add_text("My awesome post here. O love it");
    // send the Post to review
    my_post.request_review();
    println!("my text: {}", my_post.content());
    // approve the Post
    my_post.approve();
    println!("my text: {}", my_post.content());
    my_post.reject();
    println!("my text reject: {}", my_post.content());


    // create a Post
    let mut my_post_reject = Post::new();
    // add text to the Post
    my_post_reject.add_text("My awesome post here. O love it");
    // send the Post to review
    my_post_reject.request_review();
    println!("my text: {}", my_post_reject.content());
    // reject the Post
    my_post_reject.reject();
    println!("my text reject: {}", my_post_reject.content());
    // approve the Post
    my_post_reject.approve();
    println!("my text can't be approve: {}", my_post_reject.content());
    my_post_reject.request_review();
    my_post_reject.approve();
    my_post_reject.approve();
    my_post_reject.add_text("I change itt");
    println!("my text have approved: {}", my_post_reject.content());

}
