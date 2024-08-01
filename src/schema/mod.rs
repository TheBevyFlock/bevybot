use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct IssueComment {
    pub action: Action,
    pub comment: Comment,
    pub sender: Sender,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Created,
    #[serde(other)]
    Other,
}

#[derive(Deserialize, Debug)]
pub struct Comment {
    pub body: String,
}

#[derive(Deserialize, Debug)]
pub struct Sender {
    pub login: String,
}
