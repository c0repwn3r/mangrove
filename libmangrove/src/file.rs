pub trait FileOps {
    fn to_file(data: Self, filename: String);
    fn from_file(filename: String) -> Self;
}
