// 迭代器对象是可变的才能调用next方法。for...in...的语法糖自动取得了所有权并变为可变，所以创建迭代器对象时不用强制可变
// .iter() for an iterator of references不可变引用
// .iter_mut() for an iterator of mutable references可变引用
// .into_iter() for an iterator of values (not references)获取所有权
// 对iter调用map方法可以产生一个新的迭代器，不会消耗元素
// 迭代器是惰性的，除非调用消耗型的方法

//Todo about the mutability of struct and enum
#[derive(Debug, Clone)]
struct Library {
    library_type: LibraryType,
    books: Vec<String>,
}

#[derive(Debug, Clone)]
enum LibraryType {
    City,
    Country,
}

impl Library {
    fn add_book(&mut self, book: &str) {
        self.books.push(book.to_string());
    }
    fn new(library_type: LibraryType) -> Self {
        Self {
            // simplification
            library_type,
            books: Vec::new(),
        }
    }
}

impl Iterator for Library {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        // rust allows String+&str(be cautious about the order)
        self.books.pop().map(|book| book + " is found!")
    }
}

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec1_a = vec1.iter().map(|x| x + 1).collect::<Vec<i32>>();
    let vec1_b = vec1.into_iter().map(|x| x + 2).collect::<Vec<i32>>();
    println!("{:?}\n{:?}", vec1_a, vec1_b);
    // manually create the iterator
    let mut my_library = Library::new(LibraryType::Country);
    my_library.add_book("The Doom of the Darksword");
    my_library.add_book("Demian - die Geschichte einer Jugend");
    my_library.library_type = LibraryType::City;
    my_library.add_book("구운몽");
    my_library.add_book("吾輩は猫である");
    for item in my_library.clone() {
        // we can use a for loop now. Give it a clone so Library won't be destroyed
        println!("{}", item);
    }
}
