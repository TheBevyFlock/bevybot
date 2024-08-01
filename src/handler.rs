use crate::schema::{Action, IssueComment};

// TODO: Instead of hard-coding usernames, fetch their IDs and compare that.
const ALLOWED_SENDERS: &[&str] = &["BD103"];

pub async fn handle(issue_comment: IssueComment) {
    // Must be a whitelisted sender.
    if !ALLOWED_SENDERS.contains(&issue_comment.sender.login.as_str()) {
        return;
    }

    // Only handle comments when it is first created, not edited nor deleted.
    if !matches!(issue_comment.action, Action::Created) {
        return;
    }

    // Must ping bevybot.
    if !issue_comment.comment.body.contains("@bevybot") {
        return;
    }

    println!("{:#?}", issue_comment);
}
