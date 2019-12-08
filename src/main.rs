/// ( ) in commands denote optional letters
/// ( ) in other contexts denotes comments
/// < > denotes an argument
/// Tabbing denotes subcommands a/o related comments
/// | (single) at the beginning of a line denotes a group
///     Groups can be broken
/// Double tabbing OR
/// Single tabbed groups
///     denote subcommands a/o related comments that apply
///     to the last (latest) group
/// || (double) surrounding a word denotes a keyword non-positional argument
///     These must come after the base command, but order does not matter
/// * at the beginning of a line means that the command creates a subcontext
///     Exit out from subcontext with 'exit'
///     Preface input with 'global' to issue commands from the global context
///         This will usually exit out from subcontext by default
/// command list
///     Within Each Context
///     * config;       ConfigContext
///         set <key> <value>
///             (throws error if invalid type)
///         get <key>
///             (throws error if invalid key)
///         update
///             (updates config file if anything changed)
/// 
///     * team <team name>  TeamContext | OrganizationContext
///         ==== OrganizationContext ====
///            list
///               (lists all teams in organization)
///         stats (default [current_team]);     StatsContext
///             | * <team name>
///             | * <organization name>
///                 (loads stats)
///                     (750B vs 750)
///             rank
///                     <team name>
///         history
/// 
///     * comp(etition) <sku>   CompetitionContext
///         team <team name>
///     
///         match (default next);   MatchContext
///             * load      MatchListContext
///                 (if team name supplied)
///                     (load organization's match list)
///                         (!! CONFIGURATION: organization or team)
///             | * next
///             | * prev
///                 (if team name supplied)
///                     (return team's last played match)
///                 (else)
///                     (return last updated match)
///             * lookup <n>
///                 (n would be nth match)
///         wait
///     * stats     StatsContext
///         | * <team name> | <organization name>
///         | * <competition sku>       
///         | * <state abbreviation>    StatsListContext
///     

use std::io::{self, Write};
use std::path::Path;
mod repl;

fn main() {
    println!("Hello, world!");

    let mut user_input = String::new();

    let mut repl_interface = repl::ReplInterface::new(&Some(Path::new("testing.json")));
    let repl_configuration = repl_interface.get_configuration();
    println!("{}\n{}", repl_configuration.get_current_team(), repl_configuration.is_match_load_default_to_organization());

    loop
    {
        let input = repl_interface.prompt();
        if let Err(9) = repl_interface.eval(input)
        {
            break;
        }
    }
}
