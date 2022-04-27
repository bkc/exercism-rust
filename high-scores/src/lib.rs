#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
    highest_scores: Vec<u32>,
}

const MAXIMUM_HIGHEST_SCORES: usize = 3;

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        let maximum_result_length = scores.len().min(MAXIMUM_HIGHEST_SCORES);

        HighScores {
            scores,
            highest_scores: scores.iter().fold(
                Vec::with_capacity(MAXIMUM_HIGHEST_SCORES + 1),
                |mut highest_scores, item| {
                    if highest_scores.is_empty() {
                        highest_scores.push(*item);
                    } else if highest_scores.last().unwrap() <= item
                        || highest_scores.len() < maximum_result_length
                    {
                        // temporarily add 0 to end of vec
                        if highest_scores.len() < MAXIMUM_HIGHEST_SCORES {
                            highest_scores.push(0);
                        }
                        // insert new item in correct location, that moves added 0 to the right
                        for idx in 0..highest_scores.len() {
                            if highest_scores[idx] <= *item {
                                highest_scores.insert(idx, *item);
                                break;
                            }
                        }
                        // truncate vec to max length
                        if highest_scores.len() > maximum_result_length {
                            highest_scores.truncate(maximum_result_length)
                        }
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
