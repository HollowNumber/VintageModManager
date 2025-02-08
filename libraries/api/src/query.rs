///
/// tagids\[\]: Filter by tag id (AND)
/// gameversion or gv: Filter by game version id
/// gameversions\[\]: Filter by game version ids (OR)
/// author: Filter by author id
/// text: Search by mod text and title
/// orderby: Order by, one of: 'asset.created', 'lastreleased', 'Downloads', 'Follows', 'Comments', 'trendingpoints' (default: asset.created)
/// orderdirection: Order direction, one of: 'desc', 'asc' (default: desc)
/// Example: Search Example: http://mods.vintagestory.at/api/mods?text=jack&tagids\[\]=7&tagids\[\]=8&orderby=Downloads

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
    /// "gameversions[]=1&gameversions[]=2"
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
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
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
    pub fn with_order_direction(mut self, order_direction: OrderDirection) -> Self {
        self.order_direction = Some(order_direction);
        self
    }

    /// Builds the query string from the `Query` instance.
    ///
    /// # Returns
    ///
    /// A `String` representing the query string.
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
