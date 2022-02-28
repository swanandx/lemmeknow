use crate::Matches;

/// Structure defining the fields of a Filter
#[derive(Default)]
pub struct Filters {
    /// Keep Data having minimun Rarity of supplied `min_rarity`
    /// Defaults to None
    pub min_rarity: Option<f32>,
    /// Keep Data having maximun Rarity of supplied `max_rarity`
    /// Defaults to None
    pub max_rarity: Option<f32>,
    /// Only include the Data which have at least one of the specified `tags`
    /// Defaults to empty Vecotr
    pub tags: Vec<String>,
    /// Only include Data which doesn't have any of the `excluded_tags`
    /// Defaults to empty Vector
    pub exclude_tags: Vec<String>,
}

impl Filters {
    /// Filter the Matches as per user supplied options
    ///
    /// # Arguments
    ///
    /// * matches: &mut Vec<Matches> - Vector containing the matched results.
    ///
    /// # Examples
    ///
    /// ```
    /// use lemmeknow::{what_is, filter::Filters};
    /// let mut result = what_is("ctf{flag}");
    /// let filt =  Filters{
    ///   min_rarity: Some(1.0),
    /// ..Filters::default()
    /// };
    /// filt.filter(&mut result);
    /// assert_eq!(result[0].data.Name, "Capture The Flag (CTF) Flag");
    /// ```
    pub fn filter(&self, matches: &mut Vec<Matches>) {
        if let Some(min_rarity) = self.min_rarity {
            matches.retain(|x| x.data.Rarity >= min_rarity)
        }
        if let Some(max_rarity) = self.max_rarity {
            matches.retain(|x| x.data.Rarity <= max_rarity)
        }
        if !self.tags.is_empty() {
            matches.retain(|x| self.tags.iter().any(|y| x.data.Tags.contains(y)))
        }
        if !self.exclude_tags.is_empty() {
            matches.retain(|x| self.exclude_tags.iter().any(|y| !x.data.Tags.contains(y)))
        }
    }
}
