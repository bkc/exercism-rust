#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
    highest_scores: Vec<u32>,
}

const MAXIMUM_HIGHEST_SCORES: usize = 3;

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores {
            scores,
            highest_scores: scores.iter().fold(
                Vec::new() as Vec<u32>,
                |mut highest_scores, item| {
                    if highest_scores.is_empty() {
                        // length 0
                        highest_scores.push(*item)
                    } else if highest_scores.len() < MAXIMUM_HIGHEST_SCORES {
                        // length 2
                        if highest_scores[0] <= *item {
                            highest_scores.insert(0, *item)
                        } else {
                            highest_scores.push(*item)
                        }
                    } else if highest_scores[0] <= *item {
                        highest_scores.insert(0, *item);
                        highest_scores.truncate(MAXIMUM_HIGHEST_SCORES)
                    } else if highest_scores[1] <= *item {
                        highest_scores.insert(1, *item);
                        highest_scores.truncate(MAXIMUM_HIGHEST_SCORES)
                    } else if highest_scores[2] < *item {
                        highest_scores[2] = *item;
                    }
                    highest_scores
                },
            ),
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.highest_scores.is_empty() {
            None
        } else {
            Some(self.highest_scores[0])
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.highest_scores.clone()
    }
}
