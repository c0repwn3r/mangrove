pub trait FileOps {
    fn to_file(data: Self, filename: String) -> Result<(), String>;
    fn from_file(filename: String) -> Result<Self, String>
    where
        Self: Sized;
}
