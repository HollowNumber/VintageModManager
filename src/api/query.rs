/// * tagids\[\]: Filter by tag id (AND)
///
/// * gameversion or gv: Filter by game version id
///
/// * gameversions\[\]: Filter by game version ids (OR)
///
/// * author: Filter by author id
///
/// * text: Search by mod text and title
///
/// * orderby: Order by, one of: 'asset.created', 'lastreleased', 'Downloads', 'Follows', 'Comments', 'trendingpoints' (default: asset.created)
///
/// * orderdirection: Order direction, one of: 'desc', 'asc' (default: desc)
///
/// * Example: Search Example: http://mods.vintagestory.at/api/mods?text=jack&tagids\[\]=7&tagids\[\]=8&orderby=Downloads

#[derive(Debug, PartialEq)]
pub enum OrderBy {
    AssetCreated,
    LastReleased,
    Downloads,
    Follows,
    Comments,
    TrendingPoints,
}

#[derive(Debug, PartialEq)]
pub enum OrderDirection {
    Desc,
    Asc,
}

#[derive(Debug)]
pub struct Query {
    /// Vector of tag IDs to filter by (AND).
    pub tag_ids: Vec<u16>,
    /// Optional game version ID to filter by.
    pub game_version: Option<u16>,
    /// Vector of game version IDs to filter by (OR).
    pub game_versions: Vec<u16>,
    /// Optional author ID to filter by.
    pub author: Option<u16>,
    /// Optional text to search by mod text and title.
    pub text: Option<String>,
    /// Optional order by field.
    pub order_by: Option<OrderBy>,
    /// Optional order direction.
    pub order_direction: Option<OrderDirection>,
}

impl Query {
    /// Creates a new `Query` instance with default values.
    pub fn new() -> Self {
        Self {
            tag_ids: Vec::new(),
            game_version: None,
            game_versions: Vec::new(),
            author: None,
            text: None,
            order_by: None,
            order_direction: None,
        }
    }

    /// Sets the tag IDs for the query.
    ///
    /// # Arguments
    ///
    /// * `tag_ids` - A vector of tag IDs.
    ///
    /// # Returns
    ///
    /// The updated `Query` instance.
    ///
    /// # Example
    /// ```
    /// let query = api::Query::new().with_tag_ids(vec![1, 2, 3]).build();
    ///
    /// assert_eq!(query, "tagids[]=1&tagids[]=2&tagids[]=3");
    /// ```
    pub fn with_tag_ids(mut self, tag_ids: Vec<u16>) -> Self {
        self.tag_ids = tag_ids;
        self
    }

    /// Sets the game version for the query.
    ///
    /// # Arguments
    ///
    /// * `game_version` - A game version ID.
    ///
    /// # Returns
    ///
    /// The updated `Query` instance.
    ///
    /// # Example
    /// ```
    /// let query = api::Query::new().with_game_version(42).build();
    ///
    /// assert_eq!(query, "gameversion=42");
    /// ```
    ///
    pub fn with_game_version(mut self, game_version: u16) -> Self {
        self.game_version = Some(game_version);
        self
    }

    /// Sets the game versions for the query.
    ///
    /// # Arguments
    ///
    /// * `game_versions` - A vector of game version IDs.
    ///
    /// # Returns
    ///
    /// The updated `Query` instance.
    ///
    /// # Example
    ///
    /// ```
    /// let query = api::Query::new()
    ///     .with_game_versions(vec![1, 2])
    ///     .build();
    ///
    /// assert_eq!(query, "gameversions[]=1&gameversions[]=2");
    /// ```
    /// This will return mods that are compatible with game version 1 OR 2.
    pub fn with_game_versions(mut self, game_versions: Vec<u16>) -> Self {
        self.game_versions = game_versions;
        self
    }

    /// Sets the author for the query.
    ///
    /// # Arguments
    ///
    /// * `author` - An author ID.
    ///
    /// # Returns
    ///
    /// The updated `Query` instance.
    ///
    /// # Example
    /// ```
    /// let query = api::Query::new()
    ///     .with_author(7)
    ///     .build();
    ///
    /// assert_eq!(query, "author=7");
    /// ```
    pub fn with_author(mut self, author: u16) -> Self {
        self.author = Some(author);
        self
    }

    /// Sets the text for the query.
    ///
    /// # Arguments
    ///
    /// * `text` - A string to search by mod text and title.
    ///
    /// # Returns
    ///
    /// The updated `Query` instance.
    ///
    /// # Example
    ///
    /// ```
    /// let query = api::Query::new()
    ///     .with_text("example")
    ///     .build();
    ///
    /// assert_eq!(query, "text=example");
    /// ```
    ///
    pub fn with_text(mut self, text: &str) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Sets the order by field for the query.
    ///
    /// # Arguments
    ///
    /// * `order_by` - An `OrderBy` enum value.
    ///
    /// # Returns
    ///
    /// The updated `Query` instance.
    ///
    /// # Example
    /// ```
    /// let query = api::Query::new()
    ///     .with_order_by(api::OrderBy::Downloads)
    ///     .build();
    ///
    /// assert_eq!(query, "orderby=Downloads");
    /// ```
    pub fn with_order_by(mut self, order_by: OrderBy) -> Self {
        self.order_by = Some(order_by);
        self
    }

    /// Sets the order direction for the query.
    ///
    /// # Arguments
    ///
    /// * `order_direction` - An `OrderDirection` enum value.
    ///
    /// # Returns
    ///
    /// The updated `Query` instance.
    ///
    /// # Example
    /// ```
    /// let query = api::Query::new()
    ///     .with_order_direction(api::OrderDirection::Asc)
    ///     .build();
    ///
    /// assert_eq!(query, "orderdirection=asc");
    /// ```
    pub fn with_order_direction(mut self, order_direction: OrderDirection) -> Self {
        self.order_direction = Some(order_direction);
        self
    }

    /// Builds the query string from the `Query` instance.
    ///
    /// # Returns
    ///
    /// A `String` representing the query string.
    ///
    /// # Example
    /// ```
    /// let query = api::Query::new()
    ///     .with_tag_ids(vec![1, 2])
    ///     .with_game_version(42)
    ///     .with_author(7)
    ///     .with_text("example")
    ///     .with_order_by(api::OrderBy::Downloads)
    ///     .with_order_direction(api::OrderDirection::Desc)
    ///     .build();
    ///
    /// assert_eq!(query, "tagids[]=1&tagids[]=2&gameversion=42&author=7&text=example&orderby=downloads&orderdirection=desc");
    /// ```
    pub fn build(&self) -> String {
        let mut query_string = String::new();

        if !self.tag_ids.is_empty() {
            for tag_id in &self.tag_ids {
                query_string.push_str(&format!("tagids[]={}&", tag_id));
            }
        }

        if let Some(game_version) = self.game_version {
            query_string.push_str(&format!("gameversion={}&", game_version));
        }

        if !self.game_versions.is_empty() {
            for game_version in &self.game_versions {
                query_string.push_str(&format!("gameversions[]={}&", game_version));
            }
        }

        if let Some(author) = self.author {
            query_string.push_str(&format!("author={}&", author));
        }

        if let Some(ref text) = self.text {
            query_string.push_str(&format!("text={}&", text));
        }

        if let Some(ref order_by) = self.order_by {
            let order_by_str = match order_by {
                OrderBy::AssetCreated => "asset.created",
                OrderBy::LastReleased => "lastreleased",
                OrderBy::Downloads => "downloads",
                OrderBy::Follows => "follows",
                OrderBy::Comments => "comments",
                OrderBy::TrendingPoints => "trendingpoints",
            };
            query_string.push_str(&format!("orderby={}&", order_by_str));
        }

        if let Some(ref order_direction) = self.order_direction {
            let order_direction_str = match order_direction {
                OrderDirection::Desc => "desc",
                OrderDirection::Asc => "asc",
            };
            query_string.push_str(&format!("orderdirection={}&", order_direction_str));
        }

        query_string.trim_end_matches('&').to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_new() {
        let query = Query::new();

        assert_eq!(query.tag_ids, Vec::<u16>::new());
        assert_eq!(query.game_version, None);
        assert_eq!(query.game_versions, Vec::<u16>::new());
        assert_eq!(query.author, None);
        assert_eq!(query.text, None);
        assert_eq!(query.order_by, None);
        assert_eq!(query.order_direction, None);
    }

    #[test]
    fn test_query_with_tag_ids() {
        let query = Query::new().with_tag_ids(vec![1, 2, 3]);

        assert_eq!(query.tag_ids, vec![1, 2, 3]);
    }

    #[test]
    fn test_query_with_game_version() {
        let query = Query::new().with_game_version(42);

        assert_eq!(query.game_version, Some(42));
    }

    #[test]
    fn test_query_with_game_versions() {
        let query = Query::new().with_game_versions(vec![1, 2]);

        assert_eq!(query.game_versions, vec![1, 2]);
    }

    #[test]
    fn test_query_with_author() {
        let query = Query::new().with_author(7);

        assert_eq!(query.author, Some(7));
    }

    #[test]
    fn test_query_with_text() {
        let query = Query::new().with_text("example");

        assert_eq!(query.text, Some("example".into()));
    }

    #[test]
    fn test_query_with_order_by() {
        let query = Query::new().with_order_by(OrderBy::Downloads);

        assert_eq!(query.order_by, Some(OrderBy::Downloads));
    }

    #[test]
    fn test_query_with_order_direction() {
        let query = Query::new().with_order_direction(OrderDirection::Asc);

        assert_eq!(query.order_direction, Some(OrderDirection::Asc));
    }

    #[test]
    fn test_query_build() {
        let query = Query::new()
            .with_tag_ids(vec![1, 2])
            .with_game_version(42)
            .with_author(7)
            .with_text("example")
            .with_order_by(OrderBy::Downloads)
            .with_order_direction(OrderDirection::Desc)
            .build();

        assert_eq!(
            query,
            "tagids[]=1&tagids[]=2&gameversion=42&author=7&text=example&orderby=downloads&orderdirection=desc"
        );
    }
}
