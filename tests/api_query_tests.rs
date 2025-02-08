use api::query::{OrderBy, OrderDirection, Query};

#[cfg(test)]
mod tests {
    use super::*;

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
        let query = Query::new().with_game_versions(vec![1, 2, 3]);
        assert_eq!(query.game_versions, vec![1, 2, 3]);
    }

    #[test]
    fn test_query_with_author() {
        let query = Query::new().with_author(7);
        assert_eq!(query.author, Some(7));
    }

    #[test]
    fn test_query_with_text() {
        let query = Query::new().with_text("example".to_string());
        assert_eq!(query.text, Some("example".to_string()));
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
            .with_text("example".to_string())
            .with_order_by(OrderBy::Downloads)
            .with_order_direction(OrderDirection::Desc);

        let query_string = query.build();
        assert_eq!(
            query_string,
            "tagids[]=1&tagids[]=2&gameversion=42&author=7&text=example&orderby=downloads&orderdirection=desc"
        );
    }
}
