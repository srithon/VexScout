
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
    division: String,

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
    DivisionContext(String), // division name
    RoundContext(u16), // round number
    MatchListContext(Vec<Match>),
    MatchContext(Match), // match id
    ConfigContext,
    StatsContext,
    HistoryContext, // team name
    TeamContext(String), // team name
    OrganizationContext(String), // organization name
}

impl fmt::Display for ProgramContext
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            ProgramContext::CompetitionContext(sku) => {
                write!(f, "{}> ", sku)
            },
            ProgramContext::ConfigContext => {
                write!(f, "config> ")
            },
            ProgramContext::HistoryContext => {
                write!(f, "history> ")
            },
            ProgramContext::DivisionContext(division_name) => {
                write!(f, "{}> ", division_name)
            },
            ProgramContext::RoundContext(round_num) => {
                write!(f, "{}> ", round_num)
            },
            ProgramContext::MatchContext(match_struct) => {
                write!(f, "#{}> ", match_struct.match_num)
            },
            // todo
            ProgramContext::MatchListContext(match_list) => {
                Ok(())
            },
            ProgramContext::OrganizationContext(organization_name) => {
                write!(f, "{}> ", organization_name)
            },
            ProgramContext::TeamContext(team_name) => {
                write!(f, "{}> ", team_name)
            },
            ProgramContext::StatsContext => {
                write!(f, "stats>")
            }
        }
    }
}