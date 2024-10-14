use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ApiResponse {
    Response(ResponseMsg),
    Stats(StatsResponse),
    History(Vec<ResponseMsg>),
    Invalid(BadRequest),
}

#[derive(Deserialize, Debug)]
pub struct ResponseMsg {
    token: String,
    device: String,
    cards: Vec<String>,
    #[serde(rename = "dealerCards")]
    dealer_cards: Vec<String>,
    #[serde(rename = "handValue")]
    hand_value: u8,
    #[serde(rename = "dealerValue")]
    dealer_value: u8,
    status: String,
}

#[derive(Deserialize, Debug)]
pub struct BadRequest {
    message: String,
}

#[derive(Deserialize, Debug)]
pub struct StatsResponse {
    wins: usize,
    loses: usize,
    draws: usize,
}

impl std::fmt::Display for ApiResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Response(r) => write!(f, "{r}"),
            Self::Stats(s) => write!(f, "{s}"),
            Self::Invalid(i) => write!(f, "{i}"),
            Self::History(history) => {
                if history.is_empty() {
                    writeln!(f, "*** No history to display! ***")?;
                } else {
                    for (idx, history) in history.into_iter().enumerate() {
                        writeln!(f, "*** Game {} ***", idx + 1)?;
                        write!(f, "{history}")?;
                        writeln!(f, "")?;
                    }
                }

                Ok(())
            }
        }
    }
}

impl std::fmt::Display for ResponseMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "--- Game ({}): {} ---", self.status, self.token)?;
        writeln!(f, "Hand: {:?} ({})", self.cards, self.hand_value)?;
        if !self.dealer_cards.is_empty() {
            writeln!(f, "Dealer: {:?} ({})", self.dealer_cards, self.dealer_value)?;
        }
        writeln!(f, "Device: {}", self.device)?;
        writeln!(f, "------")
    }
}

impl std::fmt::Display for StatsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "--- BlackJack Game Stats ---")?;
        writeln!(f, "Total Wins: {}", self.wins)?;
        writeln!(f, "Total Losses: {}", self.loses)?;
        writeln!(f, "Total Draws: {}", self.draws)?;
        writeln!(f, "------")
    }
}

impl std::fmt::Display for BadRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.message)
    }
}
