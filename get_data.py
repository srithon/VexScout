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