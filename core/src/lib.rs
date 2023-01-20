pub struct User {}

pub trait Projects {
    fn find(&self, user: &User);
    fn find_one(&self, user: &User);
}
