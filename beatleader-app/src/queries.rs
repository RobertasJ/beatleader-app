use bon::Builder;
use freya::query::QueryCapability;
use reqwest::IntoUrl;

use crate::prelude::*;

#[derive(Clone, PartialEq, Hash, Eq)]
/// query input: image url
pub struct FetchImage;

impl QueryCapability for FetchImage {
    type Ok = Vec<u8>;

    type Err = beatleader_api::Error;

    type Keys = Url;

    fn run(&self, url: &Self::Keys) -> impl Future<Output = Result<Self::Ok, Self::Err>> {
        BlApi::download_image(url.clone())
    }
}

#[derive(Clone, PartialEq, Hash, Eq)]
pub struct FetchPlayer;

impl QueryCapability for FetchPlayer {
    type Ok = objects::Player;

    type Err = beatleader_api::Error;

    type Keys = PlayerId;

    fn run(&self, id: &Self::Keys) -> impl Future<Output = Result<Self::Ok, Self::Err>> {
        BlApi::player(id)
    }
}

#[derive(Clone, PartialEq, Hash, Eq)]
pub struct FetchCurrentPlayer;

impl QueryCapability for FetchCurrentPlayer {
    type Ok = objects::Player;

    type Err = beatleader_api::Error;

    type Keys = ();

    async fn run(&self, _: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        let id = BlApi::identity().await?.id;
        BlApi::player(&id).await
    }
}

#[derive(Clone, PartialEq, Hash, Eq)]
pub struct FetchPlayerScores;

impl QueryCapability for FetchPlayerScores {
    type Ok = objects::Scores;

    type Err = beatleader_api::Error;

    type Keys = PlayerId;

    fn run(&self, id: &Self::Keys) -> impl Future<Output = Result<Self::Ok, Self::Err>> {
        BlApi::player_scores(id)
    }
}

#[derive(Clone, PartialEq, Hash, Eq)]
pub struct FetchLeaderboard;

impl QueryCapability for FetchLeaderboard {
    type Ok = objects::Leaderboard;

    type Err = beatleader_api::Error;

    type Keys = LeaderboardId;

    async fn run(&self, id: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        let leaderboard = BlApi::leaderboard(id).await;
        println!("meow");
        leaderboard
    }
}
