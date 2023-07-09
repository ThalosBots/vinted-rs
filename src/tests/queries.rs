use crate::db::DbController;
use crate::model::filter::Filter;
use crate::queries::VintedWrapperError;
use crate::VintedWrapper;
use bb8_postgres::tokio_postgres::NoTls;

const DB_URL: &str = "postgres://postgres:postgres@localhost/vinted-rs";
const POOL_SIZE: u32 = 5;

fn _calculate_color_props(hex_color1: &str) -> (f64, f64, f64) {
    let color1 = _hex_to_rgb(hex_color1);

    let r_prop = color1.0 as f64 / 255.0;
    let g_prop = color1.1 as f64 / 255.0;
    let b_prop = color1.2 as f64 / 255.0;

    (r_prop, g_prop, b_prop)
}

fn _hex_to_rgb(hex_color: &str) -> (u8, u8, u8) {
    let hex = hex_color.trim_start_matches('#');

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();

    (r, g, b)
}

#[tokio::test]
async fn test_get_item_query_text() {
    let vinted = VintedWrapper::new();

    let filter: Filter = Filter::builder().search_text(String::from("shoes")).build();

    match vinted.get_items(&filter, 1).await {
        // Limitado el numero de elementos a 1
        Ok(items) => {
            assert_eq!(items.items.len(), 1);
        }
        Err(err) => match err {
            VintedWrapperError::ItemNumberError => unreachable!(),
            VintedWrapperError::CookiesError(_) => (),
        },
    };
}

#[tokio::test]
async fn test_get_item_brands() {
    let vinted = VintedWrapper::new();
    let db: DbController<NoTls> = DbController::new(DB_URL, POOL_SIZE, NoTls).await.unwrap();
    let brand = db.get_brand_by_name(&String::from("Adidas")).await.unwrap();

    let filter: Filter = Filter::builder().brand_ids(brand.id.to_string()).build();

    match vinted.get_items(&filter, 1).await {
        // Limitado el numero de elementos a 1
        Ok(items) => {
            assert_eq!(items.items.get(0).unwrap().brand_title, brand.title);
        }
        Err(err) => match err {
            VintedWrapperError::ItemNumberError => unreachable!(),
            VintedWrapperError::CookiesError(_) => (),
        },
    };
}

#[tokio::test]
async fn test_get_items_brands() {
    let vinted = VintedWrapper::new();
    let db: DbController<NoTls> = DbController::new(DB_URL, POOL_SIZE, NoTls).await.unwrap();
    let brand = db.get_brand_by_name(&String::from("Adidas")).await.unwrap();

    let filter: Filter = Filter::builder().brand_ids(brand.id.to_string()).build();

    match vinted.get_items(&filter, 10).await {
        Ok(items) => {
            for item in items.items {
                assert_eq!(item.brand_title, brand.title);
            }
        }
        Err(err) => match err {
            VintedWrapperError::ItemNumberError => unreachable!(),
            VintedWrapperError::CookiesError(_) => (),
        },
    };
}

#[tokio::test]
#[ignore]
async fn test_get_items_catalogs_no_db() {
    let vinted = VintedWrapper::new();
    //Woman elements
    let filter: Filter = Filter::builder().catalog_ids(String::from("1904")).build();
    let substrings = vec![
        "women", "mujer", "femme", "kobiety", "donna", "moterims", "noi", "dames", "zeny", "damen",
        "femei", "mulher", "beauty", "femmes", "dam",
    ];

    match vinted.get_items(&filter, 10).await {
        Ok(items) => {
            assert_eq!(items.items.len(), 10);
            items.items.iter().for_each(|item| {
                let url_item: &str = &item.url;
                let category = url_item.split('/').nth(3).unwrap();
                println!("{:?}", category);
                assert!(
                    substrings.contains(&category),
                    "Category not found {}",
                    category
                );
            });
        }
        Err(err) => match err {
            VintedWrapperError::ItemNumberError => unreachable!(),
            VintedWrapperError::CookiesError(_) => (),
        },
    };
}

#[tokio::test]
async fn test_get_items_by_price() {
    let vinted = VintedWrapper::new();
    let min = 50;
    let max = 100;

    let filter: Filter = Filter::builder().price_from(min).price_to(max).build();

    match vinted.get_items(&filter, 10).await {
        Ok(items) => {
            assert_eq!(items.items.len(), 10);
            let ok: bool = items.items.iter().all(|item| {
                let price: f32 = item.price.parse().unwrap();
                price <= max as f32 && price >= min as f32
            });

            assert!(ok);
        }
        Err(err) => match err {
            VintedWrapperError::ItemNumberError => unreachable!(),
            VintedWrapperError::CookiesError(_) => (),
        },
    };
}

#[tokio::test]
async fn test_get_items_by_size() {
    let vinted = VintedWrapper::new();
    let size_id = String::from("1568");
    let size_title = String::from("XS");

    let filter: Filter = Filter::builder().size_ids(size_id).build();

    match vinted.get_items(&filter, 20).await {
        Ok(items) => {
            assert_eq!(items.items.len(), 20);
            let ok: bool = items.items.iter().all(|item| item.size_title == size_title);

            assert!(ok);
        }
        Err(err) => match err {
            VintedWrapperError::ItemNumberError => unreachable!(),
            VintedWrapperError::CookiesError(_) => (),
        },
    };
}

#[tokio::test]
async fn test_get_items_by_material() {
    let vinted = VintedWrapper::new();
    let id = 49; // Silk

    let filter: Filter = Filter::builder().material_ids(id.to_string()).build();
    let num: usize = 15;

    match vinted.get_items(&filter, num as u32).await {
        Ok(items) => {
            assert_eq!(items.items.len(), num);
        }
        Err(err) => match err {
            VintedWrapperError::ItemNumberError => unreachable!(),
            VintedWrapperError::CookiesError(_) => (),
        },
    };
}

#[tokio::test]
async fn test_get_items_by_color() {
    let vinted = VintedWrapper::new();
    let id = 7; //Red
                //let hex = "#CC3300"; //Red

    //let props = calculate_color_props(hex);

    let filter: Filter = Filter::builder().color_ids(id.to_string()).build();

    let num: usize = 20;

    match vinted.get_items(&filter, num as u32).await {
        Ok(items) => {
            assert_eq!(items.items.len(), num);
        }
        Err(err) => match err {
            VintedWrapperError::ItemNumberError => unreachable!(),
            VintedWrapperError::CookiesError(_) => (),
        },
    };
}
