///
///tagids[]: Filter by tag id (AND)
// gameversion or gv: Filter by game version id
// gameversions[]: Filter by game version ids (OR)
// author: Filter by author id
// text: Search by mod text and title
// orderby: Order by, one of: 'asset.created', 'lastreleased', 'downloads', 'follows', 'comments', 'trendingpoints' (default: asset.created)
// orderdirection: Order direction, one of: 'desc', 'asc' (default: desc)
// Example: Search Example: http://mods.vintagestory.at/api/mods?text=jack&tagids[]=7&tagids[]=8&orderby=downloads

use serde::Serialize;


#[derive(Serialize)]
pub struct Query {
    pub tagids: Option<Vec<u16>>,
    pub gameversion: Option<u16>,
    pub gameversions: Option<Vec<u16>>,
    pub author: Option<u16>,
    pub text: Option<String>,
    pub orderby: Option<String>,
    pub orderdirection: Option<String>,
}

impl Query {
    pub fn new() -> Self {
        Self {
            tagids: None,
            gameversion: None,
            gameversions: None,
            author: None,
            text: None,
            orderby: None,
            orderdirection: None,
        }
    }


    pub fn query_from_tags(tags: Vec<u16>) -> Self {
        Self {
            tagids: Some(tags),
            gameversion: None,
            gameversions: None,
            author: None,
            text: None,
            orderby: None,
            orderdirection: None,
        }
    }

    pub fn to_query_string(&self) -> String {
        let mut query_string = String::new();
        if let Some(ref tagids) = self.tagids {
            for tagid in tagids {
                query_string.push_str(&format!("tagids[]={}&", tagid));
            }
        }
        if let Some(gameversion) = self.gameversion {
            query_string.push_str(&format!("gameversion={}&", gameversion));
        }
        if let Some(ref gameversions) = self.gameversions {
            for gv in gameversions {
                query_string.push_str(&format!("gameversions[]={}&", gv));
            }
        }
        if let Some(author) = self.author {
            query_string.push_str(&format!("author={}&", author));
        }
        if let Some(ref text) = self.text {
            query_string.push_str(&format!("text={}&", text));
        }
        if let Some(ref orderby) = self.orderby {
            query_string.push_str(&format!("orderby={}&", orderby));
        }
        if let Some(ref orderdirection) = self.orderdirection {
            query_string.push_str(&format!("orderdirection={}&", orderdirection));
        }
        query_string.trim_end_matches('&').to_string()



        }

}
