// We are deriving the Debug trait for our struct, which allows us to print its contents for debugging purposes.
#[derive(Debug)]
// We define a public struct named HighScores that has a lifetime parameter 'a and a field scores which is a reference to an array of u32 integers.
pub struct HighScores<'a> {
    scores: &'a[u32],
}

// We are implementing methods for our struct HighScores.
impl<'a> HighScores<'a> {
    // This is a constructor method that takes a reference to an array of u32 integers and returns a new instance of HighScores.
    pub fn new(scores: &'a[u32]) -> Self {
        Self { scores }
    }

    // This method returns the scores field of the HighScores instance.
    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    // This method returns the last score in the scores field of the HighScores instance, if it exists.
    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    // This method returns the highest score in the scores field of the HighScores instance, if it exists.
    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    // This method returns the top three highest scores in the scores field of the HighScores instance.
    pub fn personal_top_three(&self) -> Vec<u32> {
        // We create a mutable copy of the scores field.
        let mut scores = self.scores.to_vec();
        // We sort the scores in ascending order.
        scores.sort_unstable();
        // We reverse the order of the scores to get them in descending order, take the first three scores, copy them into a new iterator, and collect them into a vector.
        scores.iter().rev().take(3).copied().collect()
    }
}