
use std::path::Path;

use serde::{Deserialize, Serialize};
// use serde_json::Result;
use std::io::{self, Write};
use std::fmt;

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

    fn get_contexts_mut(&mut self) -> &mut Vec<ProgramContext>
    {
        &mut self.contexts
    }

    pub fn add_context(&mut self, context: ProgramContext)
    {
        self.contexts.push(context);
    }

    pub fn prompt(&self) -> String
    {
        let mut user_input = String::new();

        let contexts = self.get_contexts();
        if contexts.len() != 0
        {
            for context in contexts
            {
                print!("{}", context);
            }
        }
        else
        {
            print!("> ");
        }

        let _ = io::stdout().flush();
        io::stdin().read_line(&mut user_input)
            .expect("Error getting input");
        user_input
    }

    pub fn eval(&mut self, input: String) -> Result<(), u8>
    {
        // println!("{}", input.chars().rev().skip(1).collect::<String>());
        let mut words = input.trim_end().split_ascii_whitespace();

        if let Some(mut command) = words.next()
        {
            let global = command.eq_ignore_ascii_case("global");
            
            if !global
            {
                if command.eq_ignore_ascii_case("exit")
                {
                    if self.get_contexts_mut().pop().is_some()
                    {
                        return Ok(());
                    }
                    else
                    {
                        return Err(9);
                    }
                }
            }

            let current_context = self.get_contexts().last();
            if current_context.is_some() && !global
            {
                let context = current_context.unwrap();
                match context
                {
                    _ => ()
                }
            }
            else // list is empty, base context
            {
                if global
                {
                    let command_maybe = words.next();
                    if let Some(second_word) = command_maybe
                    {
                        command = second_word;
                    }
                    else
                    {
                        println!("Please enter a valid command after 'global'");
                        return Err(1);
                    }

                    // todo do something better than clearing every time
                    self.get_contexts_mut().clear();
                }
                match command.to_ascii_lowercase().as_str()
                {
                    "config" => {
                        println!("config!!");
                        self.add_context(ProgramContext::ConfigContext);
                    },
                    "competition" | "comp" => {
                        println!("competition!");
                        if let Some(sku) = words.next()
                        {
                            // todo verify sku is valid
                            self.add_context(ProgramContext::CompetitionContext(sku.to_owned()));
                        }
                    },
                    "stats" => {
                        println!("stats");

                        if let Some(team_or_organization_name) = words.next()
                        {
                            // todo verify name is valid and not random string
                            let last_char = team_or_organization_name.chars().last();
                            if let Some(character) = last_char
                            {
                                // todo clear or branch off from old contexts

                                if character.is_alphabetic()
                                {
                                    let team_context = ProgramContext::TeamContext(team_or_organization_name.to_string());
                                    self.add_context(team_context);
                                    self.add_context(ProgramContext::StatsContext);
                                }
                                else
                                {
                                    let organization_context = ProgramContext::OrganizationContext(team_or_organization_name[..team_or_organization_name.len() - 1].to_string());
                                    self.add_context(organization_context);
                                    self.add_context(ProgramContext::StatsContext);
                                }
                            }
                            else // originally thought this was just a newline but dont think so
                            {
                                println!("How did you do that?");
                                return Err(1);
                            }
                        }
                        else // enter stats mode
                        {
                            self.add_context(ProgramContext::StatsContext);
                        }
                    },
                    "team" => {
                        println!("team!");

                        if let Some(team_name) = words.next()
                        {
                            // todo verify valid team name
                            self.add_context(ProgramContext::TeamContext(team_name.to_string()));
                        }
                        else // no team specified
                        {
                            println!("Please specify a team");
                            return Err(1);
                        }
                    },
                    "exit" => {
                        return Err(9);
                    }
                    _ => {
                        println!("whatever");
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct ReplConfiguration
{
    match_load_default_to_organization: bool,
    current_team: String
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