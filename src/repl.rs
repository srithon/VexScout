
use std::path::Path;

use serde::{Deserialize, Serialize};
// use serde_json::Result;

pub struct ReplInterface
{
    config: ReplConfiguration,
    contexts: Vec<ProgramContext>
}

impl ReplInterface
{
    pub fn new(configuration_file_path: &Option<&Path>) -> ReplInterface
    {
        let configuration = if let Some(path) = configuration_file_path {
            let file = std::fs::File::open(path);
            if let Ok(file) = file
            {
                println!("File is ok!");
                let json = serde_json::from_reader(file);
                if let Ok(config) = json
                {
                    config
                }
                else
                {
                    ReplConfiguration::default()
                }
            }
            else
            {
                println!("File not ok!");
                ReplConfiguration::default()
            }
        }
        else
        {
            ReplConfiguration::default()
        };

        ReplInterface
        {
            config: configuration,
            contexts: Vec::<ProgramContext>::new() 
        }
    }

    pub fn get_configuration(&self) -> &ReplConfiguration
    {
        &self.config
    }

    pub fn get_contexts(&self) -> &Vec<ProgramContext>
    {
        &self.contexts
    }

    pub fn add_context(&mut self, context: ProgramContext)
    {
        self.contexts.push(context);
    }
}

#[derive(Serialize, Deserialize)]
pub struct ReplConfiguration
{
    match_load_default_to_organization: bool,
    current_team: String,
}

impl ReplConfiguration
{
    fn default() -> ReplConfiguration
    {
        ReplConfiguration
        {
            match_load_default_to_organization: false,
            current_team: String::new()
        }
    }

    pub fn get_current_team(&self) -> &String
    {
        &self.current_team
    }

    pub fn is_match_load_default_to_organization(&self) -> &bool
    {
        &self.match_load_default_to_organization
    }
}

struct Match
{
    team_list: Vec<String>,

    round: u8,
    match_num: u16,

    red1: String,
    red2: String,
    blue1: String,
    blue2: String,

    red_score: u16,
    blue_score: u16,

    scored: bool
}

pub enum ProgramContext
{
    CompetitionContext(String), // sku
    MatchListContext(Vec<Match>),
    MatchContext(Match), // match id
    ConfigContext,
    StatsContext(String), // team name or organization name
    HistoryContext(String), // team name
    TeamContext(String), // team name
    OrganizationContext(String), // organization name
}