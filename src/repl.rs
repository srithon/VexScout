use std::path::Path;

mod repl
{
    ReplInterface,
    ProgramContext
}

struct ReplInterface
{
    config: HashMap<String, String>,
    contexts: Vec<ProgramContext>
}

struct ReplConfiguration
{
    match_load_default_to_organization: bool,
    current_team: String,
}

impl ReplInterface
{
    
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

enum ProgramContext
{
    CompetitionContext(String), // sku
    MatchListContext(Vec<Match>),
    MatchContext(Match), // match id
    ConfigContext,
    StatsContext(String), // team name or organization name
    HistoryContext(String) // team name
    TeamContext(String), // team name
    OrganizationContext(String), // organization name
}