struct Book {} // Okay, first I need a book struct.
// Nothing in there yet - will add later

enum BookType {
    // A book can be hardcover or softcover, so add an enum
    HardCover,
    SoftCover,
}

fn get_book(book: &Book) -> Option<String> {
    todo!()
} // ⚠️ get_book should take a &Book and return an Option<String>

fn delete_book(book: Book) -> Result<(), String> {
    todo!()
} // delete_book should take a Book and return a Result...
// TODO: impl block and make these functions methods...
fn check_book_type(book_type: &BookType) {
    // Let's make sure the match statement works
    match book_type {
        BookType::HardCover => println!("It's hardcover"),
        BookType::SoftCover => println!("It's softcover"),
    }
}

fn main() {
    let book_type = BookType::HardCover;
    check_book_type(&book_type); // Okay, let's check this function!
}
