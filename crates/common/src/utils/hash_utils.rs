
use uuid::Uuid;

pub fn gene_uuid() -> String {
    let id = Uuid::new_v4();
    let mut no_hyphens = String::with_capacity(32);
    let uuid_str = id.to_string();

    no_hyphens.push_str(&uuid_str[0..8]);
    no_hyphens.push_str(&uuid_str[9..13]);
    no_hyphens.push_str(&uuid_str[14..18]);
    no_hyphens.push_str(&uuid_str[19..23]);
    no_hyphens.push_str(&uuid_str[24..]);

    no_hyphens
}
