use typed_builder::TypedBuilder;

pub mod brand;
pub mod category;
pub mod category_tree;
pub mod colors;
pub mod country;
pub mod material;
pub mod size;

#[derive(TypedBuilder, Debug, Clone)]
pub struct Filter {
    #[builder(default, setter(strip_option))]
    pub search_text: Option<String>,
    #[builder(default, setter(strip_option))]
    pub catalog_ids: Option<String>,
    #[builder(default, setter(strip_option))]
    pub color_ids: Option<String>,
    #[builder(default, setter(strip_option))]
    pub brand_ids: Option<String>,
    #[builder(default, setter(strip_option))]
    pub countries_ids: Option<String>,
    #[builder(default, setter(strip_option))]
    pub size_ids: Option<String>,
    #[builder(default, setter(strip_option))]
    pub article_status: Option<Vec<ArticleStatus>>,
    #[builder(default, setter(strip_option))]
    pub sort_by: Option<SortBy>,
}

#[derive(Debug, Clone)]
pub enum ArticleStatus {
    NewTags,
    NewNoTags,
    VeryGood,
    Good,
    Satisfactory,
}

impl From<&ArticleStatus> for &str {
    /// From `&ArticleStatus` to `&str`
    fn from(status: &ArticleStatus) -> Self {
        match *status {
            ArticleStatus::NewTags => "6",
            ArticleStatus::NewNoTags => "1",
            ArticleStatus::VeryGood => "2",
            ArticleStatus::Good => "3",
            ArticleStatus::Satisfactory => "4",
        }
    }
}

#[derive(Debug, Clone)]
pub enum SortBy {
    Relevance,
    PriceDescendant,
    PriceAscendant,
    NewestFirst,
}

impl From<&SortBy> for &str {
    /// From `&SortBy` to `&str`
    fn from(sort: &SortBy) -> Self {
        match *sort {
            SortBy::Relevance => "relevance",
            SortBy::PriceDescendant => "price_high_to_low",
            SortBy::PriceAscendant => "price_low_to_high",
            SortBy::NewestFirst => "newest_first",
        }
    }
}
