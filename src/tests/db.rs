use crate::{
    db::DbController,
    model::filter::{brand::Brand, category::Category, country::Country},
};
use bb8_postgres::tokio_postgres::NoTls;

const DB_URL: &str = "postgres://postgres:postgres@localhost/vinted-rs";
const POOL_SIZE: u32 = 5;

#[tokio::test]
async fn test_get_brand_by_name() {
    let db: DbController<NoTls> = DbController::new(DB_URL, POOL_SIZE, NoTls).await.unwrap();

    let brand_name: String = String::from("adidas");

    let b: Brand = db.get_brand_by_name(&brand_name).await.unwrap();

    assert_eq!(
        b,
        Brand::builder()
            .title(String::from("adidas"))
            .id(14)
            .url(String::from("https://www.vinted.es/brand/adidas"))
            .build()
    );
}

#[tokio::test]
async fn test_get_brands_by_name() {
    let db: DbController<NoTls> = DbController::new(DB_URL, POOL_SIZE, NoTls).await.unwrap();

    let brand_name: String = String::from("adidas");

    let brands: Vec<Brand> = db.get_brands_by_name(&brand_name).await.unwrap();

    assert_eq!(brands.len(), 38);
}

#[tokio::test]
async fn test_get_category_by_name() {
    let db: DbController<NoTls> = DbController::new(DB_URL, POOL_SIZE, NoTls).await.unwrap();

    let category_name: String = String::from("Women");

    let c = db.get_category_by_title(&category_name).await.unwrap();

    assert_eq!(
        c,
        Category::builder()
            .code(String::from("WOMEN_ROOT"))
            .url(String::from("/women"))
            .url_en(String::from("/women"))
            .title(String::from("Women"))
            .id(1904)
            .parent_id(0)
            .build()
    );
}

#[tokio::test]
async fn test_get_country_by_iso() {
    let db: DbController<NoTls> = DbController::new(DB_URL, POOL_SIZE, NoTls).await.unwrap();
    let c = db.get_country_by_iso(&String::from("ES")).await.unwrap();

    assert_eq!(
        c,
        Country::builder()
            .id(7)
            .name(String::from("Espagne"))
            .local_name(String::from("España"))
            .iso_code(String::from("ES"))
            .flag(String::from("🇪🇸"))
            .build()
    );
}
