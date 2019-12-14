
use std::path::Path;

use serde::{Deserialize, Serialize};
// use serde_json::Result;
use std::io::{self, Write};
use std::fmt;

use regex::Regex;

use lazy_static::{lazy_static};

fn team_name_is_valid(name: &str) -> bool
{
    lazy_static! {
        static ref REGEX_MATCHER: Regex = Regex::new(r"[0-9]+[a-zA-Z]").unwrap();
    }
    REGEX_MATCHER.is_match(name)
}

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

    ///
    /// Error Codes
    /// 0: No subcommand specified
    /// 1: Invalid command
    /// 2: Unknown Error
    /// 3: Invalid input
    /// 4: Missing argument
    /// 9: Exit
    pub fn eval(&mut self, input: String) -> Result<(), u8>
    {
        // println!("{}", input.chars().rev().skip(1).collect::<String>());
        let mut words = input.trim().split_ascii_whitespace();

        if let Some(command) = words.next()
        {
            let command = command.trim().to_ascii_lowercase();
            let global = command.eq("global");
            
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

            let mut contexts = self.get_contexts().iter();
            let first_context = {
                if global
                {
                    &ProgramContext::BaseContext
                }
                else
                {
                    if let Some(context) = contexts.next()
                    {
                        context
                    }
                    else
                    {
                        &ProgramContext::BaseContext
                    }
                }
            };

            match first_context
            {
                ProgramContext::ConfigContext => {

                },
                ProgramContext::TeamContext(team_name) => {
                    if let Some(secondary_context) = contexts.next()
                    {
                        match secondary_context
                        {
                            ProgramContext::StatsContext => {
                                if let Some(subcommand) = words.next()
                                {
                                    match subcommand.to_ascii_lowercase().as_str()
                                    {
                                        "graph" => {
                                            println!("graphing!");
                                        },
                                        "competition" => {
                                            println!("competition stats");
                                        },
                                        _ => {
                                            println!("invalid subcommand");
                                        }
                                    }
                                }
                            },
                            ProgramContext::HistoryContext => {
                                println!("team history");
                            },
                            _ => ()
                        }
                    }
                    else
                    {
                        match command.as_str()
                        {
                            "stats" => {
                                self.add_context(ProgramContext::StatsContext);
                            },
                            "history" => {
                                self.add_context(ProgramContext::HistoryContext);
                            },
                            _ => {
                                println!("Invalid subcommand");
                            }
                        }
                    }
                },
                ProgramContext::OrganizationContext(organization_name) => {
                    if let Some(subcontext) = contexts.next()
                    {
                        match subcontext
                        {
                            ProgramContext::StatsContext => {

                            },
                            ProgramContext::HistoryContext => {

                            },
                            _ => ()
                        }
                    }
                    else
                    {
                        match command.as_str()
                        {
                            "list" => {
                                println!("listing all teams in {}", organization_name);
                            },
                            "graph" => {
                                println!("graphing!");
                            },
                            "competition" => {
                                println!("competition stats");
                            },
                            _ => {
                                println!("invalid subcommand");
                            }
                        }
                    }
                },
                ProgramContext::CompetitionContext(competition_sku) => {
                    if let Some(subcontext) = contexts.next()
                    {
                        match subcontext
                        {
                            ProgramContext::TeamContext(team_name) => {
                                if let Some(subcommand) = words.next()
                                {
                                    let subcommand = subcommand.trim().to_ascii_lowercase();
                                    
                                    match subcommand.as_str()
                                    {
                                        "load" => {
                                            println!("loading all matches for {}", team_name);
                                        },
                                        "next" => {
                                            println!("finding next match for {}", team_name);
                                        },
                                        "prev" => {
                                            println!("finding previous match for {}", team_name);
                                        },
                                        "lookup" => {
                                            println!("looking up nth match for {}", team_name);
                                        },
                                        _ => ()
                                    }
                                }
                                else
                                {
                                    return Err(0);
                                }
                            },
                            ProgramContext::MatchContext(match_struct) => {
                                println!("Match");
                            },
                            ProgramContext::MatchListContext(match_struct_list) => {
                                println!("Multiple matches");
                            },
                            _ => ()
                        }
                    }
                    else
                    {
                        match command.as_str()
                        {
                            "team" => {
                                if let Some(team_name) = words.next()
                                {
                                    // todo verify team name integrity
                                    if team_name_is_valid(team_name)
                                    {
                                        println!("Valid team name: {}", team_name);
                                        self.add_context(ProgramContext::TeamContext(team_name.to_owned()));
                                    }
                                    else
                                    {
                                        println!("Invalid team name: {}", team_name);
                                        return Err(3);
                                    }
                                }
                                else
                                {
                                    return Err(0);
                                }
                            },
                            "match" => {
                                println!("match");

                                if let Some(subcommand) = words.next()
                                {
                                    let subcommand = subcommand.trim().to_ascii_lowercase();
                                    
                                    match subcommand.as_str()
                                    {
                                        "load" => {
                                            println!("loading")
                                        },
                                        "next" => {
                                            println!("finding next match");
                                        },
                                        "prev" => {
                                            println!("finding previous match");
                                        },
                                        "lookup" => {
                                            println!("looking up nth match");
                                        },
                                        _ => ()
                                    }
                                }
                                else
                                {
                                    return Err(0);
                                }
                            },
                            "wait" => {
                                println!("wait");
                            },
                            _ => {
                                println!("Invalid subcommand");
                            }
                        }
                    }
                },
                ProgramContext::StatsContext => {
                    if let Some(target) = words.next()
                    {
                        // figure out what it means; team name, org name...
                        println!("target of stats is {}", target);
                    }
                    else
                    {
                        return Err(0);
                    }
                },
                ProgramContext::BaseContext => {
                    let real_command = {
                        if global
                        {
                            let command_maybe = words.next();
                            if let Some(second_word) = command_maybe
                            {
                                // self.get_contexts_mut().clear();
                                second_word
                            }
                            else
                            {
                                println!("Please enter a command after 'global'");
                                return Err(1);
                            }
                        }
                        else
                        {
                            &command
                        }
                    }.to_ascii_lowercase();

                    match real_command.as_str()
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
                        "stats" | "team" => {    
                            if let Some(team_or_organization_name) = words.next()
                            {
                                let last_char = team_or_organization_name.chars().last();
                                if let Some(character) = last_char
                                {
                                    // todo clear or branch off from old contexts
    
                                    if character.is_alphabetic()
                                    {
                                        let team_name = team_or_organization_name.to_string();
                                        
                                        if team_name_is_valid(&team_name)
                                        {
                                            let team_context = ProgramContext::TeamContext(team_name.to_ascii_uppercase());
                                            self.add_context(team_context);
                                        }
                                        else
                                        {
                                            println!("Invalid team name: {}", team_name);                                            
                                            return Err(3);
                                        }
                                    }
                                    // otherwise if characters are all digits
                                    else if team_or_organization_name.chars().all(|x| !x.is_alphabetic())
                                    {
                                        let organization_context = ProgramContext::OrganizationContext(team_or_organization_name.to_string());
                                        self.add_context(organization_context);
                                    }
                                    else
                                    {
                                        println!("Invalid organization/team name: {}", team_or_organization_name);
                                        return Err(3);
                                    }

                                    if real_command.eq("stats")
                                    {
                                        self.add_context(ProgramContext::StatsContext);
                                    }
                                }
                            }
                            else if real_command.eq("stats") // enter stats mode
                            {
                                self.add_context(ProgramContext::StatsContext);
                            }
                            else
                            {
                                println!("Please enter a team name!");
                                return Err(4);
                            }
                        },
                        "exit" => {
                            return Err(9);
                        },
                        _ => {
                            println!("whatever");
                        }
                    }
                },
                _ => {
                    panic!("Invalid base context!");
                }
            }
        }

        Ok(())
    }
}

// todo have option for VEXU support
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
    BaseContext
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
            ProgramContext::MatchListContext(match_list) => {
                write!(f, "list> ")
            },
            ProgramContext::OrganizationContext(organization_name) => {
                write!(f, "{}> ", organization_name)
            },
            ProgramContext::TeamContext(team_name) => {
                write!(f, "{}> ", team_name)
            },
            ProgramContext::StatsContext => {
                write!(f, "stats> ")
            },
            ProgramContext::BaseContext => {
                panic!("Calling fmt on base context?");
            }
        }
    }
}