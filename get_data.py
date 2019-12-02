import requests
import sys

API_ENDPOINT = 'https://api.vexdb.io/v1'

class InteractiveRepl:
    """

    config is a dictionary
    {
        current_team: string
    }

    """
    def __init__(self, config):
        self.config = config

    """
    ( ) in commands denote optional letters
    ( ) in other contexts denotes comments
    
    < > denotes an argument
    
    Tabbing denotes subcommands a/o related comments

    | (single) at the beginning of a line denotes a group
        Groups can be broken
    
    Double tabbing OR
    Single tabbed groups
        denote subcommands a/o related comments that apply
        to the last (latest) group
    
    || (double) surrounding a word denotes a keyword non-positional argument
        These must come after the base command, but order does not matter
    
    * at the beginning of a line means that the command creates a subcontext
        Exit out from subcontext with 'exit'
        Preface input with 'global' to issue commands from the global context
            This will usually exit out from subcontext by default

    command list
        comp(etition) <sku>
        team <team name>
        config
            set <key> <value>
                (throws error if invalid type)
            get <key>
                (throws error if invalid key)
            update
                (updates config file if anything changed)
        match (default next)
            load
                (if team name supplied)
                    (load organization's match list)
                        (!! CONFIGURATION: organization or team)
            | * next
            | * prev
                    (if team name supplied)
                        (return team's last played match)
                    (else)
                        (return last updated match)
            * lookup <n>
                (n would be nth match)
            wait
            history
                (returns team's record; match history)
                        team <name>
        stats (default [current_team])
            || comp || (default)
            || world ||
            | * <team name>
            | * <organization name>
                (loads stats)
                    (750B vs 750)
            rank
                    <team name>
        * history <team name>
            (<team name> defaults to our team)
            (returns match history of given team)

    """
    def eval(self, string):
        print(string[::-1])
    def lookup(self, key):
        try:
            return self.config[key]
        except KeyError:
            return None
    def prompt(self):
        current_team = self.lookup('current_team')
        # todo allow for multi-line inputs
        if current_team == None:
            try:
                return input("> ")
            except KeyboardInterrupt:
                sys.exit()
        else:
            try:
                return input(current_team + "> ")
            except KeyboardInterrupt:
                sys.exit()

s = requests.Session()

# Todo implement configuration file

config = {
    "current_team": "750B"
}

repl = InteractiveRepl(config)

user_input = repl.prompt()
while not user_input.lower() == 'exit':
    repl.eval(user_input)
    user_input = repl.prompt()