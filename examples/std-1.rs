// extern crate hashmap;
// Jon does this, but seems to no longer be necessary
use hashmap::HashMap;

fn main() {
    // Type inference lets us omit an explicit type signature (which
    // would be `HashMap<String, String>` in this example).
    // Vincent: added Turbofish because example is not the same as when Jon copied it.
    let mut book_reviews = HashMap::<&str,&str>::new();

    // Review some books.
    book_reviews.insert(
        "Adventures of Huckleberry Finn",
        "My favorite book.",
    );
    book_reviews.insert(
        "Grimms' Fairy Tales",
        "Masterpiece.",
    );
    book_reviews.insert(
        "Pride and Prejudice",
        "Very enjoyable.",
    );
    book_reviews.insert(
        "The Adventures of Sherlock Holmes",
        "Eye lyked it alot.",
    );

    // Check for a specific one.
    // When collections store owned values (String), they can still be
    // queried using references (&str).
    if !book_reviews.contains_key(&"Les Misérables") {
        println!(
            "We've got {} reviews, but Les Misérables ain't one.",
            book_reviews.len()
        );
    }

    // oops, this review has a lot of spelling mistakes, let's delete it.
    book_reviews.remove(&"The Adventures of Sherlock Holmes");

    // Look up the values associated with some keys.
    let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
    for &book in &to_find {
        match book_reviews.get(&book) {
            Some(review) => println!("{book}: {review}"),
            None => println!("{book} is unreviewed."),
        }
    }

    // Iterate over everything.
    for (book, review) in &book_reviews {
        println!("{book}: \"{review}\"");
    }
}
