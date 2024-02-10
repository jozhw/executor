// this is essentially the totality of the search matches with a count of successful matches

use super::search_match::SearchMatch;

pub struct TraverseMatch {
    // count the number of matches
    pub match_count: i32,
    // all of the files that matched that includes file name and path
    pub files_matched: Vec<SearchMatch>,
}
