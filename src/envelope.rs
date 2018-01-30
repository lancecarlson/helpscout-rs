#[derive(Debug, Copy, Clone, Deserialize)]
pub struct Collection<T> {
    pub page: i32,
    pub pages: i32,
    pub count: i32,
    pub items: Vec<T>,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct Item<T> {
    pub item: T,
}
