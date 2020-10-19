use super::models::{Game, PitchingStats, Position, Team};
use super::team_pair::TeamPair;
use super::State;

impl Game {
    pub fn pitcher_ids(&self) -> Option<TeamPair<&str>> {
        Some(TeamPair {
            home: &self.home_pitcher.as_ref()?,
            away: &self.away_pitcher.as_ref()?,
        })
    }

    pub fn team_ids(&self) -> TeamPair<&str> {
        TeamPair {
            home: &self.home_team,
            away: &self.away_team,
        }
    }

    pub fn teams<'a>(&self, state: &'a State) -> Option<TeamPair<&'a Team>> {
        self.team_ids()
            .and_then(|x| state.teams.iter().find(|y| x == y.id))
    }

    pub fn pitcher_positions<'a>(&self, state: &'a State) -> Option<TeamPair<&'a Position>> {
        self.pitcher_ids()?
            .and_then(|x| state.players.iter().find(|y| x == y.id))
    }

    pub fn pitcher_stats<'a>(&self, state: &'a State) -> Option<TeamPair<&'a PitchingStats>> {
        self.pitcher_ids()?
            .and_then(|x| state.pitcher_stats.iter().find(|y| x == y.player_id))
    }
}

impl Team {
    pub fn at_bats<'a>(&'a self, state: &'a State) -> impl Iterator<Item = Option<usize>> + 'a {
        self.lineup.iter().map(move |x| {
            state
                .at_bats
                .iter()
                .find(|y| x == &y.player_id)
                .map(|y| y.at_bats)
        })
    }

    pub fn strikeouts<'a>(&'a self, state: &'a State) -> impl Iterator<Item = Option<usize>> + 'a {
        self.lineup.iter().map(move |x| {
            state
                .strikeouts
                .iter()
                .find(|y| x == &y.player_id)
                .map(|y| y.strikeouts)
        })
    }
}
