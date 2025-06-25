#[derive(Debug, PartialEq)]
pub enum LibraryError {
    BookNotFound(String),
    BookAlreadyBorrowed(String),
    BookNotBorrowed(String),
    InvalidIsbn(String),
    BookAlreadyExists(String),
}

// 本の情報を示す構造体
#[derive(Debug, Clone, PartialEq)]
pub struct Book {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub is_borrowed: bool,
}

impl Book {
    pub fn new(isbn: String, title: String, author: String) -> Result<Self, LibraryError> {
        // ISBNの形式をチェック
        if isbn.is_empty() || isbn.len() < 10 {
            return Err(LibraryError::InvalidIsbn(isbn));
        }

        Ok(Book {
            isbn,
            title,
            author,
            is_borrowed: false,
        })
    }

    pub fn borrow_book(&mut self) -> Result<(), LibraryError> {
        //既に貸出中なら貸出中のエラーを返す
        if self.is_borrowed {
            return Err(LibraryError::BookAlreadyBorrowed(self.isbn.clone()));
        }

        //貸出中でなければ貸出を行う
        self.is_borrowed = true;
        Ok(())
    }

    pub fn return_book(&mut self) -> Result<(), LibraryError> {
        //貸出中でなければ返却できないエラーを返す
        if !self.is_borrowed {
            return Err(LibraryError::BookNotBorrowed(self.isbn.clone()));
        }

        //貸出中であれば返却を行う
        self.is_borrowed = false;
        Ok(())
    }
} //impl Book

#[derive(Debug, Default)]
pub struct Library {
    books: Vec<Book>,
}

impl Library {
    pub fn new() -> Self {
        Library { books: Vec::new() }
    }

    pub fn add_book(&mut self, book: Book) -> Result<(), LibraryError> {
        // ISBNが重複している場合はエラーを返す
        if self.books.iter().any(|b| b.isbn == book.isbn) {
            return Err(LibraryError::BookAlreadyExists(book.isbn.clone()));
        }

        // 本を追加する
        self.books.push(book);
        Ok(())
    }

    pub fn find_book(&mut self, isbn: &str) -> Option<&Book> {
        self.books.iter().find(|b| b.isbn == isbn)
    }

    pub fn list_available_books(&self) -> Vec<&Book> {
        self.books.iter().filter(|b| !b.is_borrowed).collect()
    }

    pub fn book_count(&self) -> usize {
        self.books.len()
    }
} //impl Library

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_library() -> Library {
        Library::new()
    }

    fn create_test_book() -> Book {
        Book::new(
            "978-4-06-519465-6".to_string(),
            "Rustプログラミング入門".to_string(),
            "山田太郎".to_string(),
        )
        .unwrap()
    }

    // =========================
    // Book構造体のテスト
    // =========================
    #[test]
    fn 正常な本を作成できること() {
        // Arrange & Act
        let book = Book::new(
            "978-4-06-519465-6".to_string(),
            "Rustプログラミング入門".to_string(),
            "山田太郎".to_string(),
        );

        // Assert
        assert!(book.is_ok());
        let book = book.unwrap();
        assert_eq!(book.isbn, "978-4-06-519465-6");
        assert_eq!(book.title, "Rustプログラミング入門");
        assert_eq!(book.author, "山田太郎");
        assert!(!book.is_borrowed);
    }

    #[test]
    #[allow(non_snake_case)]
    fn 無効なISBNではエラーを返すこと() {
        // Arrange & Act
        let book = Book::new(
            "123".to_string(),
            "テスト本".to_string(),
            "テスト著者".to_string(),
        );

        // Assert
        assert!(book.is_err());
        assert_eq!(
            book.unwrap_err(),
            LibraryError::InvalidIsbn("123".to_string())
        );
    }

    #[test]
    fn 本を貸出できること() {
        // Arrange
        let mut book = create_test_book();

        // Act
        let result = book.borrow_book();

        // Assert
        assert!(result.is_ok());
        assert!(book.is_borrowed);
    }

    #[test]
    fn 既に貸出中の本は貸出できないこと() {
        // Arrange
        let mut book = create_test_book();
        book.borrow_book().unwrap(); //貸出中にしてエラーは無視

        // Act
        let result = book.borrow_book(); //再度借りる

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            LibraryError::BookAlreadyBorrowed(book.isbn.clone())
        );
    }

    #[test]
    fn 貸出中の本を返却できること() {
        // Arrange
        let mut book = create_test_book();
        book.borrow_book().unwrap(); //貸出中にしておく

        // Act
        let result = book.return_book();

        // Assert
        assert!(result.is_ok());
        assert!(!book.is_borrowed);
    }

    #[test]
    fn 貸し出してない本は返却できないこと() {
        // Arrange
        let mut book = create_test_book();

        // Act
        let result = book.return_book(); //貸出していないので返却

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            LibraryError::BookNotBorrowed(book.isbn.clone())
        );
    }

    // =========================
    // Library構造体のテスト
    // =========================
    #[test]
    fn 図書館を作成できること() {
        // Arrange & Act
        let library = setup_library();

        // Assert
        assert_eq!(library.book_count(), 0);
    }

    #[test]
    fn 図書館に本を追加できること() {
        // Arrange
        let mut library = setup_library();
        let book = create_test_book();

        // Act
        let result = library.add_book(book.clone());

        // Assert
        assert!(result.is_ok());
        assert_eq!(library.book_count(), 1);
    }

    #[test]
    #[allow(non_snake_case)]
    fn 同じISBNの本は追加できないこと() {
        // Arrange
        let mut library = setup_library();
        let book1 = create_test_book();
        let book2 = create_test_book(); // 同じ本を再度追加
        library.add_book(book1).unwrap(); // 最初の追加は成功

        // Act
        let result = library.add_book(book2); // 2回目の追加はエラー

        // Assert
        assert!(result.is_err());
        assert_eq!(library.book_count(), 1);
        assert_eq!(
            result.unwrap_err(),
            LibraryError::BookAlreadyExists("978-4-06-519465-6".to_string())
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn ISBNで本を検索できること() {
        // Arrange
        let mut library = setup_library();
        let book = create_test_book();
        library.add_book(book.clone()).unwrap();

        // Act
        let found_book = library.find_book(&book.isbn);

        // Assert
        assert!(found_book.is_some());
        assert_eq!(found_book.unwrap(), &book);
    }

    #[test]
    fn 存在しない本は検索できないこと() {
        // Arrange
        let mut library = setup_library();

        // Act
        let found_book = library.find_book("978-4-06-519465-6");

        // Assert
        assert!(found_book.is_none());
    }

    #[test]
    fn 利用可能な本の一覧を取得できること() {
        // Arrange
        let mut library = setup_library();
        let mut book1 = create_test_book();
        let book2 = Book::new(
            "978-4-06-519466-6".to_string(),
            "別な本".to_string(),
            "別の著者".to_string(),
        )
        .unwrap();

        book1.borrow_book().unwrap(); // book1を貸出中にする

        // 図書館に本を追加
        library.add_book(book1).unwrap();
        library.add_book(book2).unwrap();

        // Act
        let available_books = library.list_available_books();

        // Assert
        assert_eq!(available_books.len(), 1);
        assert_eq!(available_books[0].isbn, "978-4-06-519466-6");
    }
}
